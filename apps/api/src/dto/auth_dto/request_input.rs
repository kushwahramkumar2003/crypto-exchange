use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize)]
pub struct CreateUserInput{
    pub name:String,
    pub email:String,
    pub password:String
}


#[derive(Serialize,Deserialize)]
pub struct SigninRequest{
    pub email:String,
    pub password:String
}