use diesel::{self, ExpressionMethods, QueryDsl, RunQueryDsl, SqliteConnection};

/// Get all employees from the database.
pub fn get(conn: &SqliteConnection) -> Vec<String> {
    use schema::employees::*;
    table
        .select(name)
        .load::<String>(conn)
        .expect("Query failed")
}

/// Insert a new employee into the database.
///
/// Create the `name_sort` column from the `name` by assuming that the last word of the full name
/// is the last name. That is not true in general but should be enough for our purposes: sorting a
/// list of employees by name in an invoice.
pub fn insert<S: AsRef<str>>(
    conn: &SqliteConnection,
    name: S,
) -> Result<i32, diesel::result::Error> {
    use schema::employees;

    let reversed_name = {
        let words: Vec<_> = name.as_ref().split(' ').collect();
        let len = words.len();
        let mut tmp = words[len - 1].to_string();
        tmp.push_str(", ");
        for word in &words[..len - 1] {
            tmp.push_str(word);
        }
        tmp
    };

    let values = (
        employees::name.eq(name.as_ref()),
        employees::name_sort.eq(reversed_name),
    );

    diesel::insert_or_ignore_into(employees::table)
        .values(&values)
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
        use super::insert;

        let conn = ::db::connect();
        empty_tables(&conn);

        let id = insert(&conn, "Alice A.").unwrap();
        let id2 = insert(&conn, "Bob B.").unwrap();
        assert_ne!(id, id2);

        assert_eq!(insert(&conn, "Alice A.").unwrap(), id);
        assert_eq!(insert(&conn, "Bob B.").unwrap(), id2);
    }

    #[test]
    fn test_get_employees() {
        let conn = ::db::connect();
        empty_tables(&conn);

        let names = vec!["Alice A.", "Bob B.", "Charlie C."];
        for name in &names {
            super::insert(&conn, name).unwrap();
        }

        let mut retrieved_names = ::employees::get(&conn);
        retrieved_names.sort_unstable();
        assert_eq!(retrieved_names, names);
    }
}
