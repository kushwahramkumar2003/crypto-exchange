use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize,Deserialize)]
pub struct CreateUserOutput{
    pub id:String,
}

#[derive(Serialize,Deserialize)]
pub struct SigninUserOutput{
    pub jwt:String,
}

#[derive(Serialize,Deserialize)]
pub struct ProfileOutput{
    pub id: Uuid,
    pub email: String,
    pub role: String,
}