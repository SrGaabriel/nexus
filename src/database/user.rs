use rbatis::{crud_table, sql, rbatis::Rbatis, core::Result};

#[crud_table(table_name: TABLE_NAME)]
pub struct User {
    pub id: Option<u64>,
    pub name: Option<String>,
    pub email: Option<String>,
    // will be removed, here just for testing purposes
    pub password: Option<String>,
    pub account_type: Option<u8>,
    pub biography: Option<String>,
    pub tagline: Option<String>
}

#[sql(rbatis, "insert into users (id, name, email, password, account_type) values($1, $2, $3, $4, $5)")]
async fn create(rbatis: &Rbatis, id: &u64, name: &String, email: &String, password: &String, account_type: &u8) -> Option<User> {}
 
pub async fn new(rbatis: &Rbatis, id: &u64, name: &String, email: &String, password: &String, account_type: &u8) -> Result<User> {
    let result: Result<Option<User>> = create(rbatis, id, name, email, password, account_type).await;
    if result.is_err() {
        return Result::Err(rbatis::core::Error::E("couldn't create new user".to_string()))
    }
    Ok(User {
        id: Some(id.clone()),
        name: Some(name.clone()),
        email: Some(email.clone()),
        password: Some(password.clone()),
        account_type: Some(account_type.clone()),
        biography: None,
        tagline: None
    })
}

#[sql(rbatis, "select * from users where id = $1")]
async fn search(rbatis: &Rbatis, id: &u64) -> User {}

#[sql(rbatis, "select * from users where email = $1")]
async fn search_by_email(rbatis: &Rbatis, email: &String) -> User {}