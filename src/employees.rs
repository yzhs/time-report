use diesel::{QueryDsl, RunQueryDsl, SqliteConnection};

pub fn get_employees(conn: &SqliteConnection) -> Vec<String> {
    use schema::employees::*;
    table.select(name).load::<String>(conn).unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    fn empty_tables(conn: &SqliteConnection) {
        diesel::delete(schema::employees::table)
            .execute(conn)
            .unwrap();
    }

    #[test]
    fn test_insert_employee() {
        let conn = ::establish_connection();
        empty_tables(&conn);

        let id = insert_employee(&conn, "Alice A.").unwrap();
        let id2 = insert_employee(&conn, "Bob B.").unwrap();
        assert_ne!(id, id2);
        assert_eq!(insert_employee(&conn, "Alice A.").unwrap(), id);
        assert_eq!(insert_employee(&conn, "Bob B.").unwrap(), id2);
    }

    #[test]
    fn test_get_employees() {
        let conn = ::establish_connection();
        empty_tables(&conn);

        let names = vec!["Alice A.", "Bob B.", "Charlie C."];
        for name in &names {
            insert_employee(&conn, name).unwrap();
        }

        assert_eq!(get_employees(&conn), names);
    }
}
