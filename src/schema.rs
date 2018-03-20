infer_schema!("dotenv:DATABASE_URL");

/// Tell diesel about `items_view`.
table! {
    items_view (id, employee_id, report_id, name, name_sort, day, type_of_week, start, end, remark) {
        id -> Integer,
        employee_id -> Integer,
        report_id -> Integer,
        name -> Text,
        name_sort -> Text,
        day -> Date,
        type_of_week -> Integer,
        start -> Time,
        end -> Time,
        remark -> Text,
    }
}
