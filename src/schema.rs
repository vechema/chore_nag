table! {
    assignments (id) {
        id -> Integer,
        chore_id -> Integer,
        room_id -> Integer,
        user_id -> Nullable<Integer>,
    }
}

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

table! {
    users (id) {
        id -> Integer,
        name -> Text,
        description -> Nullable<Text>,
    }
}

joinable!(assignments -> chores (chore_id));
joinable!(assignments -> rooms (room_id));
joinable!(assignments -> users (user_id));

allow_tables_to_appear_in_same_query!(
    assignments,
    chores,
    rooms,
    users,
);
