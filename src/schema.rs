infer_schema!("dotenv:DATABASE_URL");

table! {
    items_view (id, employee_id, report_id, name, day, type_of_week, start, end, remark) {
        id -> Integer,
        employee_id -> Integer,
        report_id -> Integer,
        name -> Text,
        day -> Date,
        type_of_week -> Integer,
        start -> Time,
        end -> Time,
        remark -> Text,
    }
}
