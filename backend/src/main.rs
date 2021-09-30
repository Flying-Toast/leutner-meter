#[macro_use] extern crate diesel_migrations;
#[macro_use] extern crate diesel;

pub mod schema;
pub mod models;

use rocket_sync_db_pools::database;
use rocket::{
    get,
    Rocket, Build,
    fairing::AdHoc,
    fs::FileServer,
    serde::{json::Json, Serialize},
    response::{self, Responder, Response},
    request::Request,
    http::Status,
};
use std::{fmt, io::Cursor};
use models::{MealPeriod, Meal};

#[derive(Debug)]
pub enum BackendError {
    DieselError(diesel::result::Error),
    NoMealInProgress,
    UserAlreadyVoted,
}

impl fmt::Display for BackendError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use BackendError::*;

        match self {
            DieselError(e) => write!(f, "{}", e),
            NoMealInProgress => write!(f, "No meal in progress"),
            UserAlreadyVoted => write!(f, "User already submitted a vote for this period"),
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

#[rocket::launch]
fn rocket() -> _ {
    rocket::build()
        .attach(DbConn::fairing())
        .attach(AdHoc::on_ignite("Apply diesel_migrations", apply_diesel_migrations))
        .mount("/", FileServer::from("static"))
        .mount("/", rocket::routes![
            get_stats,
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
