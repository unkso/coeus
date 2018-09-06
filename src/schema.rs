table! {
    cohort (id) {
        id -> Integer,
        name -> Text,
    }
}

table! {
    player (id) {
        id -> Integer,
        cohort_id -> Integer,
        name -> Text,
        rank -> Integer,
        revives -> Float,
        repairs -> Float,
        heals -> Float,
        rounds_played -> Integer,
        squad_score -> Float,
        flag_captures -> Integer,
        flag_defends -> Integer,
        gamemode_score -> Float,
        time_played -> Integer,
    }
}

allow_tables_to_appear_in_same_query!(
    cohort,
    player,
);
