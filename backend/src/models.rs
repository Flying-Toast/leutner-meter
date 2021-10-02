use chrono::{DateTime, Utc, Duration, Datelike, Weekday, Timelike};
use diesel::{
    prelude::*,
    Queryable, Associations, RunQueryDsl,
    dsl::sum,
};
use crate::{
    DbConn, BackendError,
    schema::{votes, meals, tickets},
};
use std::fmt;
use rocket::serde::Serialize;

#[derive(Queryable)]
pub struct Ticket {
    pub id: i32,
    pub ticket: String,
    pub case_id: String,
    pub timestamp: i32,
}

#[derive(Insertable)]
#[table_name = "tickets"]
struct NewTicket {
    ticket: String,
    case_id: String,
    timestamp: i32,
}

fn current_unix_time() -> i32 {
    std::time::UNIX_EPOCH.elapsed().unwrap().as_millis() as i32
}

impl Ticket {
    pub async fn purge_old_tickets(conn: &DbConn) -> Result<usize, diesel::result::Error> {
        use tickets::dsl;

        let hour = 60 * 60;
        let day = hour * 24;

        let max_time_ago = std::time::Duration::from_secs(day * 5);

        let earliest_time = current_unix_time() - (max_time_ago.as_millis() as i32);

        conn.run(move |c| {
            diesel::delete(
                tickets::table.filter(
                    dsl::timestamp.le(earliest_time)
                )
            ).execute(c)
        }).await
    }

    pub async fn is_valid(ticket: &str, conn: &DbConn) -> Result<bool, diesel::result::Error> {
        use tickets::dsl;

        let t2 = ticket.to_string();
        conn.run(move |c| {
            tickets::table
                .filter(dsl::ticket.eq(t2))
                .count()
                .get_result::<i64>(c)
        }).await.map(|x| x != 0)
    }

    pub async fn insert_new(ticket: String, case_id: String, conn: &DbConn) -> Result<usize, diesel::result::Error> {
        conn.run(move |c| {
            diesel::insert_into(tickets::table)
                .values(NewTicket {
                    ticket,
                    case_id,
                    timestamp: current_unix_time(),
                })
                .execute(c)
        }).await
    }

    pub async fn get_case_id(conn: &DbConn, ticket: &str) -> Result<String, diesel::result::Error> {
        use tickets::dsl;

        let t2 = ticket.to_string();
        conn.run(move |c| {
            tickets::table
                .filter(dsl::ticket.eq(t2))
                .select(dsl::case_id)
                .first(c)
        }).await
    }
}

#[derive(Queryable, Associations, Identifiable)]
#[belongs_to(Meal, foreign_key = "meal_id")]
pub struct Vote {
    pub id: i32,
    pub meal_id: i32,
    pub voter_caseid: String,
    pub score: i32,
}

impl Vote {
    /// the usize in the returned Ok result is the number of affected rows
    pub async fn insert_for_current_meal<'a>(conn: &DbConn, voter_caseid: String, score: i32) -> Result<usize, BackendError> {
        if let Some(curr_meal) = Meal::get_or_create_current(conn).await {
            let curr_meal = curr_meal.map_err(BackendError::DieselError)?;

            if curr_meal.has_user_voted(&voter_caseid, conn).await.map_err(BackendError::DieselError)? {
                return Err(BackendError::UserAlreadyVoted);
            }

            conn.run(move |c| {
                diesel::insert_into(votes::table)
                    .values(NewVote {
                        meal_id: curr_meal.id,
                        voter_caseid,
                        score,
                    })
                    .execute(c)
                    .map_err(BackendError::DieselError)
            }).await
        } else {
            Err(BackendError::NoMealInProgress)
        }
    }
}

#[derive(Insertable)]
#[table_name = "votes"]
struct NewVote {
    meal_id: i32,
    voter_caseid: String,
    score: i32,
}

#[derive(Copy, Clone, Debug, Serialize)]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "lowercase")]
pub enum MealPeriod {
    Breakfast,
    Brunch,
    Lunch,
    Dinner,
}

impl fmt::Display for MealPeriod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use MealPeriod::*;
        let string = match self {
            Breakfast => "breakfast",
            Brunch => "brunch",
            Lunch => "lunch",
            Dinner => "dinner",
        };

        write!(f, "{}", string)
    }
}

macro_rules! time {
    ($hr:literal : $min:literal AM) => {{
        $min + $hr * 60
    }};

    ($hr:literal : $min:literal PM) => {{
        $min + ($hr + 12) * 60
    }};
}

impl MealPeriod {
    /// The meal that Leutner is serving right now
    pub fn current(dt: &DateTime<Utc>) -> Option<Self> {
        use Weekday::*;

        let weekday = dt.weekday();
        let atime = dt.minute() + 60 * dt.hour();

        // breakfast
        if matches!(weekday, Mon | Tue | Wed | Thu | Fri) {
            if atime >= time!(7:00 AM) && atime <= time!(10:30 AM) {
                return Some(MealPeriod::Breakfast);
            }
        }

        // brunch
        if matches!(weekday, Sat | Sun) {
            if atime >= time!(9:30 AM) && atime <= time!(2:30 PM) {
                return Some(MealPeriod::Brunch);
            }
        }

        // lunch
        if let Fri = weekday {
            if atime >= time!(11:00 AM) && atime <= time!(5:00 PM) {
                return Some(MealPeriod::Lunch);
            }
        } else {
            if atime >= time!(11:00 AM) && atime <= time!(4:00 PM) {
                return Some(MealPeriod::Lunch);
            }
        }

        // dinner
        if atime >= time!(5:00 PM) && atime <= time!(8:00 PM) {
            return Some(MealPeriod::Dinner);
        }

        None
    }

    fn as_int(&self) -> i32 {
        use MealPeriod::*;

        match self {
            Breakfast => 0,
            Brunch => 1,
            Lunch => 2,
            Dinner => 3,
        }
    }

    pub fn from_int(int: i32) -> Self {
        use MealPeriod::*;

        match int {
            0 => Breakfast,
            1 => Brunch,
            2 => Lunch,
            3 => Dinner,
            _ => panic!("Invalid meal_period int"),
        }
    }
}

/// returns (year, month, day, _)
fn now() -> (i32, i32, i32, DateTime<Utc>) {
    let now = Utc::now() - Duration::hours(4);

    (now.year() as i32, now.month() as i32, now.day() as i32, now)
}

#[derive(Queryable, Debug, Identifiable, Clone)]
pub struct Meal {
    pub id: i32,
    pub year: i32,
    pub month: i32,
    pub day: i32,
    pub meal_period: i32,
}

#[derive(Insertable)]
#[table_name = "meals"]
struct NewMeal {
    year: i32,
    month: i32,
    day: i32,
    meal_period: i32,
}

no_arg_sql_function!(
    last_insert_rowid,
    diesel::sql_types::Integer,
    "SQLite last_insert_rowid function"
);

impl Meal {
    /// Ok value is (total_score, num_votes)
    pub async fn get_stats(&self, conn: &DbConn) -> Result<(i64, i64), diesel::result::Error> {
        let s2 = self.clone();
        let total_score = conn.run(move |c| {
            Vote::belonging_to(&s2)
                .select(sum(votes::dsl::score))
                .get_result::<Option<i64>>(c)
        }).await?.unwrap_or(0);

        let s3 = self.clone();
        let num_votes = conn.run(move |c| {
            Vote::belonging_to(&s3)
                .count()
                .get_result::<i64>(c)
        }).await?;

        Ok((total_score, num_votes))
    }

    /// Whether or not the given caseid has already voted in this period
    async fn has_user_voted(&self, case_id: &str, conn: &DbConn) -> Result<bool, diesel::result::Error> {
        let s2 = self.clone();
        let case_id_clone = case_id.to_string();
        conn.run(move |c| {
            Vote::belonging_to(&s2)
                .filter(votes::dsl::voter_caseid.eq(case_id_clone))
                .count()
                .get_result::<i64>(c)
        }).await.map(|cnt| cnt != 0)
    }

    /// The `Meal` instance for the current meal on the current day
    pub async fn get_or_create_current(conn: &DbConn) -> Option<Result<Self, diesel::result::Error>> {
        use meals::dsl;

        let (nowyear, nowmonth, nowday, nowdt) = now();
        let curr_period = MealPeriod::current(&nowdt)?;

        let result = conn.run(move |c| {
            meals::table
                .filter(dsl::day.eq(nowday))
                .filter(dsl::month.eq(nowmonth))
                .filter(dsl::year.eq(nowyear))
                .filter(dsl::meal_period.eq(curr_period.as_int()))
                .first::<Meal>(c)
        }).await;

        Some(if let Err(diesel::result::Error::NotFound) = result {
            let meal_id = conn.run(move |c| {
                let _ = diesel::insert_into(meals::table)
                    .values(NewMeal {
                        year: nowyear,
                        month: nowmonth,
                        day: nowday,
                        meal_period: curr_period.as_int(),
                    })
                    .execute(c)?;

                diesel::select(last_insert_rowid)
                    .get_result::<i32>(c)
            }).await;

            if let Ok(id) = meal_id {
                conn.run(move |c| {
                    meals::table
                        .find(id)
                        .get_result(c)
                }).await
            } else {
                Err(meal_id.unwrap_err())
            }
        } else {
            match result {
                Ok(meal) => Ok(meal),
                Err(e) => Err(e),
            }
        })
    }
}
