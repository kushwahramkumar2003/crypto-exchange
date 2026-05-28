use std::{f64::consts::E, str::FromStr, sync::{Arc, Mutex}};

use db::{Db::Db, models::user::UserAuthError};
use poem::{Error, handler, http::{StatusCode, response}, web::{Data, Json, Path}};

use crate::{dto::auth_dto::{request_input::{CreateUserInput, SigninRequest}, request_output::{CreateUserOutput, SigninUserOutput}}, utils::jwt::generate_jwt};

#[handler]
pub fn signup(
    Json(data):Json<CreateUserInput>,
    Data(db):Data<&Arc<Mutex<Db>>>
) -> Result<Json<CreateUserOutput>, Error>  {
   let mut locked_db = db.lock().unwrap();

   let user_id = match locked_db.signup(data.email, data.password, String::from_str("ADMIN")?) {
       Ok(id) => id,
        Err(UserAuthError::Conflict) => {
            return Err(Error::from_status(StatusCode::CONFLICT));
        }
        Err(UserAuthError::Db(_)) => {
            return Err(Error::from_status(StatusCode::INTERNAL_SERVER_ERROR));
        }
   }; 

   let response = CreateUserOutput {
        id: user_id,
    };

    Ok(Json(response))
}

#[handler]
pub fn login(Json(
    data):Json<SigninRequest>,
    Data(db):Data<&Arc<Mutex<Db>>>
) -> Result<Json<SigninUserOutput>, Error> {
    let mut locked_db = db.lock().unwrap();
    let res = match locked_db.signin(data.email, data.password) {
        Ok(user_id)=>user_id,
        Err(_)=>{
            return Err(Error::from_status(StatusCode::INTERNAL_SERVER_ERROR));
        }
    };
    let user_id = match res {
        Some(id)=>id,
        None=>{return Err(Error::from_status(StatusCode::INTERNAL_SERVER_ERROR))}
    };
    let jwt = match generate_jwt(user_id) {
        Ok(token)=>token,
        Err(_)=>{
            return Err(Error::from_status(StatusCode::INTERNAL_SERVER_ERROR));
        }
    };
    let response = SigninUserOutput{
        jwt
    };
    Ok(Json(response))
}