use rbatis::{crud_table, sql, rbatis::Rbatis};

#[crud_table(table_name: "users")]
pub struct User {
    pub id: Option<u64>,
    pub name: Option<String>,
    pub account_type: Option<u8>,
    pub biography: Option<String>,
    pub tagline: Option<String>
}

#[sql(rbatis, "select * from users where id = $1")]
async fn search(rbatis: &Rbatis, id: &u64) -> User {}