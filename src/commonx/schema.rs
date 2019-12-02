table! {
    projects (id) {
        id -> Integer,
        name -> Text,
    }
}

table! {
    tasks (id) {
        id -> Integer,
        name -> Text,
        project_id -> Integer,
        worker_id -> Nullable<Integer>,
    }
}

table! {
    workers (id) {
        id -> Integer,
        name -> Text,
    }
}

joinable!(tasks -> projects (project_id));
joinable!(tasks -> workers (worker_id));

allow_tables_to_appear_in_same_query!(
    projects,
    tasks,
    workers,
);
