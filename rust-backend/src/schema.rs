table! {
    meals (id) {
        id -> Integer,
        year -> Integer,
        month -> Integer,
        day -> Integer,
        meal_period -> Integer,
    }
}

table! {
    votes (id) {
        id -> Integer,
        meal_id -> Integer,
        voter_caseid -> Text,
        score -> Integer,
    }
}

joinable!(votes -> meals (meal_id));

allow_tables_to_appear_in_same_query!(
    meals,
    votes,
);
