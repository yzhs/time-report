use super::schema::work_units;

/// Represent one row in the database.
#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct WorkUnit {
    #[serde(skip)]
    pub id: i32,
    pub name: String,
    pub date: String,
    pub week: i32,
    pub start: String,
    pub end: String,
    pub remark: Option<String>,
    #[serde(default)]
    pub processed: bool,
}

/// Data needed to create a new row in the database.
#[derive(Debug, Deserialize, Insertable)]
#[table_name = "work_units"]
pub struct NewWorkUnit {
    pub name: String,
    pub date: String,
    pub week: i32,
    pub start: String,
    pub end: String,
    pub remark: Option<String>,
}
