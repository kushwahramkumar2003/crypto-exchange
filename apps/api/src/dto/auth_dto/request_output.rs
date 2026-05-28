use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize)]
pub struct CreateUserOutput{
    pub id:String,
}

#[derive(Serialize,Deserialize)]
pub struct SigninUserOutput{
    pub jwt:String,
}