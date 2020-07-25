table! {
    chores (id) {
        id -> Integer,
        name -> Text,
        description -> Nullable<Text>,
    }
}

table! {
    rooms (id) {
        id -> Integer,
        name -> Text,
        description -> Nullable<Text>,
    }
}

allow_tables_to_appear_in_same_query!(
    chores,
    rooms,
);
