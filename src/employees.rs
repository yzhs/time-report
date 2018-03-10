use diesel::{self, ExpressionMethods, QueryDsl, RunQueryDsl, SqliteConnection};

pub fn get(conn: &SqliteConnection) -> Vec<String> {
    use schema::employees::*;
    table.select(name).load::<String>(conn).unwrap()
}

pub fn insert<S: AsRef<str>>(
    conn: &SqliteConnection,
    name: S,
) -> Result<i32, diesel::result::Error> {
    use schema::employees;

    diesel::insert_or_ignore_into(employees::table)
        .values(&employees::name.eq(name.as_ref()))
        .execute(conn)
        .expect("Error creating new employee record");

    employees::table
        .select(employees::id)
        .filter(employees::name.eq(name.as_ref()))
        .first::<i32>(conn)
}

#[cfg(test)]
mod test {
    use super::*;

    fn empty_tables(conn: &SqliteConnection) {
        diesel::delete(::schema::employees::table)
            .execute(conn)
            .unwrap();
    }

    #[test]
    fn test_insert_employee() {
        let conn = ::db::connect();
        empty_tables(&conn);

        let id = insert_employee(&conn, "Alice A.").unwrap();
        let id2 = insert_employee(&conn, "Bob B.").unwrap();
        assert_ne!(id, id2);
        assert_eq!(insert_employee(&conn, "Alice A.").unwrap(), id);
        assert_eq!(insert_employee(&conn, "Bob B.").unwrap(), id2);
    }

    #[test]
    fn test_get_employees() {
        let conn = ::db::connect();
        empty_tables(&conn);

        let names = vec!["Alice A.", "Bob B.", "Charlie C."];
        for name in &names {
            insert_employee(&conn, name).unwrap();
        }

        assert_eq!(::employees::get(&conn), names);
    }
}
