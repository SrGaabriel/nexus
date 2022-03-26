use rbatis::{crud_table, sql, rbatis::Rbatis};

#[crud_table(table_name: TABLE_NAME)]
pub struct User {
    pub id: Option<u64>,
    pub name: Option<String>,
    // will be removed, here just for testing purposes
    pub password: Option<String>,
    pub account_type: Option<u8>,
    pub biography: Option<String>,
    pub tagline: Option<String>
}

#[sql(rbatis, "insert into users (id, name, password, account_type, biography, tagline) values($1, $2, $3, $4, null, null)")]
async fn create(rbatis: &Rbatis, id: &u64, name: &String, password: &String, account_type: &u8) -> User {}

#[sql(rbatis, "select * from users where id = $1")]
async fn search(rbatis: &Rbatis, id: &u64) -> User {}