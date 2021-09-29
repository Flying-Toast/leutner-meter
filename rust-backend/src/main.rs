#[macro_use] extern crate diesel_migrations;
#[macro_use] extern crate diesel;

pub mod schema;
pub mod models;

use rocket_sync_db_pools::database;
use rocket::{
    Rocket, Build,
    fairing::AdHoc,
};

#[derive(Debug)]
pub enum BackendError {
    DieselError(diesel::result::Error),
    NoMealInProgress,
}

impl std::fmt::Display for BackendError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use BackendError::*;

        match self {
            DieselError(e) => write!(f, "{}", e),
            NoMealInProgress => write!(f, "No meal in progress"),
        }
    }
}

#[database("sqlite_db")]
pub struct DbConn(diesel::SqliteConnection);

async fn apply_diesel_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    embed_migrations!();

    let conn = DbConn::get_one(&rocket).await
        .expect("database connection");

    println!("==> Running migrations...");
    conn.run(|c| embedded_migrations::run_with_output(c, &mut std::io::stdout())).await
        .expect("run migrations");
    println!("==> Done running migrations");

    {
        use schema::{meals, votes};
        use models::{Vote};
    }


    rocket
}

#[rocket::launch]
fn rocket() -> _ {
    rocket::build()
        .attach(DbConn::fairing())
        .attach(AdHoc::on_ignite("Apply diesel_migrations", apply_diesel_migrations))
        .mount("/", rocket::routes![
        ])
}
