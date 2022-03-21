use rbatis::{crud_table, rbatis::Rbatis, sql};

#[crud_table(table_name: "users" | table_columns:"id,name,email,password,type,biography,tagline")]
#[derive(Clone, Debug)]
pub struct User {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub user_type: Option<u8>,
    pub biography: Option<String>,
    pub tagline: Option<String>,
}

#[sql(rb, "select * from users where id = $1")]
pub async fn search(rb: &Rbatis, id: &i64) -> User {}
