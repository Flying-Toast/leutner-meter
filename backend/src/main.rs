#[macro_use] extern crate diesel_migrations;
#[macro_use] extern crate diesel;

pub mod schema;
pub mod models;

use rocket_sync_db_pools::database;
use rocket::{
    get, post, uri,
    Rocket, Build,
    fairing::AdHoc,
    fs::FileServer,
    serde::{json::Json, Serialize, Deserialize},
    response::{self, Responder, Response, Redirect},
    request::{Request, FromRequest, Outcome},
    http::{Status, CookieJar, Cookie},
};
use std::{fmt, io::Cursor};
use models::{MealPeriod, Meal, Ticket, Vote};
use reqwest::get;
use rocket::tokio::{self, task};

#[derive(Debug)]
pub enum BackendError {
    DieselError(diesel::result::Error),
    NoMealInProgress,
    UserAlreadyVoted,
    ScoreOutOfRange,
    NotAuthed,
}

impl fmt::Display for BackendError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use BackendError::*;

        match self {
            DieselError(e) => write!(f, "{}", e),
            NoMealInProgress => write!(f, "No meal in progress"),
            UserAlreadyVoted => write!(f, "User already submitted a vote for this period"),
            NotAuthed => write!(f, "User is not logged in"),
            ScoreOutOfRange => write!(f, "Score out of range"),
        }
    }
}

impl<'r> Responder<'r, 'static> for BackendError {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        let string = self.to_string();
        Response::build()
            .sized_body(string.len(), Cursor::new(string))
            .raw_header("Content-Type", "text/plain")
            .status(Status::BadRequest)
            .ok()
    }
}

#[database("sqlite_db")]
pub struct DbConn(diesel::SqliteConnection);

async fn apply_diesel_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    embed_migrations!();

    let conn = DbConn::get_one(&rocket).await
        .expect("database connection");

    conn.run(|c| embedded_migrations::run_with_output(c, &mut std::io::stdout())).await
        .expect("run migrations");

    rocket
}

async fn ticket_gc(rocket: Rocket<Build>) -> Rocket<Build> {
    let conn = DbConn::get_one(&rocket).await
        .expect("database connection");

    task::spawn(async move {
        loop {
            if let Err(e) = Ticket::purge_old_tickets(&conn).await {
                eprintln!("ERROR PURGING OLD TICKETS: {:?}", e);
            }

            let hour = 60 * 60;
            let day = hour * 24;

            tokio::time::sleep(std::time::Duration::from_millis(day * 5)).await;
        }
    });

    rocket
}

#[rocket::launch]
fn rocket() -> _ {
    rocket::build()
        .attach(DbConn::fairing())
        .attach(AdHoc::on_ignite("Apply diesel_migrations", apply_diesel_migrations))
        .attach(AdHoc::on_ignite("Start ticket garbage collector", ticket_gc))
        .mount("/", FileServer::from("static"))
        .mount("/", rocket::routes![
            get_stats,
            submit_vote,
            sso_auth,
            check_ticket,
            auth_failed,
        ])
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Stats {
    current_meal: Option<MealPeriod>,
    scores_total: Option<i64>,
    num_votes: Option<i64>,
}

#[get("/stats")]
async fn get_stats(conn: DbConn) -> Result<Json<Stats>, BackendError> {
    let current_meal = Meal::get_or_create_current(&conn).await;

    Ok(Json(match current_meal {
        None => Stats {
            current_meal: None,
            scores_total: None,
            num_votes: None,
        },
        Some(meal) => {
            let meal = meal.map_err(BackendError::DieselError)?;
            let curr_stats = meal.get_stats(&conn).await.map_err(BackendError::DieselError)?;

            Stats {
                scores_total: Some(curr_stats.0),
                num_votes: Some(curr_stats.1),
                current_meal: Some(MealPeriod::from_int(meal.meal_period)),
            }
        }
    }))
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct SubmittedVote {
    score: u8,
}

#[post("/vote", data = "<vote>")]
async fn submit_vote(conn: DbConn, jar: &CookieJar<'_>, vote: Json<SubmittedVote>) -> Result<(), BackendError> {
    if let Some(c) = jar.get("ticket") {
        if Ticket::is_valid(c.value(), &conn).await.map_err(BackendError::DieselError)? {
            if vote.score > 10 {
                return Err(BackendError::ScoreOutOfRange);
            }
            let case_id = Ticket::get_case_id(&conn, c.value()).await.map_err(BackendError::DieselError)?;
            Vote::insert_for_current_meal(&conn, case_id, vote.score as i32).await?;
            return Ok(());
        }
    }

    Err(BackendError::NotAuthed)
}

struct HostHeader<'a>(&'a str);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for HostHeader<'r> {
    type Error = &'static str;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match req.headers().get_one("Host") {
            Some(hdr) => Outcome::Success(Self(hdr)),
            _ => Outcome::Failure((Status::BadRequest, "go away poo poo head")),
        }
    }
}

#[get("/sso-auth?<ticket>")]
async fn sso_auth(ticket: String, jar: &CookieJar<'_>, conn: DbConn, host: HostHeader<'_>) -> Redirect {
    let url = format!(
        "https://login.case.edu/cas/validate?ticket={}&service={}",
        ticket,
        format!("http://{}/sso-auth", host.0),
    );
    let good_redirect = Redirect::to(uri!("/#vote"));
    let bad_redirect = Redirect::to(uri!("/auth-failed"));

    if let Ok(resp) = get(url).await {
        if let Ok(text) = resp.text().await {
            let mut lines = text.lines();
            if let Some("yes") = lines.next() {
                if let Some(username) = lines.next() {
                    jar.add(Cookie::new("ticket", ticket.clone()));
                    if let Err(_) = Ticket::insert_new(ticket, username.to_string(), &conn).await {
                        return bad_redirect;
                    }
                    return good_redirect;
                }
            }
        }
    }

    bad_redirect
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct TicketCheck {
    is_valid: bool,
}

#[get("/auth-failed")]
fn auth_failed() -> &'static str {
    "Login failed. Please try again."
}

#[get("/check-ticket")]
async fn check_ticket(jar: &CookieJar<'_>, conn: DbConn) -> Json<TicketCheck> {
    let is_valid =
        match jar.get("ticket") {
            None => false,
            Some(c) => Ticket::is_valid(c.value(), &conn).await.unwrap_or(false),
        };

    Json(TicketCheck { is_valid, })
}
