infer_schema!("dotenv:DATABASE_URL");

table! {
    items_view (id, name, day, type_of_week, start, end, remark) {
        id -> Integer,
        name -> Text,
        day -> Date,
        type_of_week -> Integer,
        start -> Time,
        end -> Time,
        remark -> Text,
    }
}
