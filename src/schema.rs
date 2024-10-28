// @generated automatically by Diesel CLI.

diesel::table! {
    articles (id) {
        id -> Int4,
        title -> Varchar,
        sys_title -> Varchar,
        published -> Bool,
        content -> Nullable<Jsonb>,
        do_aws_sync -> Nullable<Bool>,
    }
}
