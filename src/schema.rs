infer_schema!("dotenv:DATABASE_URL");

table! {
    /// Representation of the `items_view` view.
    items_view (id, employee_id, report_id, name, name_sort, day, type_of_week,
                start, end, remark) {
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
