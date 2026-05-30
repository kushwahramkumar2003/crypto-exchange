use std::str::FromStr;

use diesel::prelude::Insertable;
use diesel::prelude::*;
use uuid::Uuid;
use diesel::result::{DatabaseErrorKind, Error as DbError};

use crate::Db::Db;
use crate::schema::users;

#[derive(Insertable)]
#[diesel(table_name=users)]
pub struct NewUser<'a> {
    email: &'a str,
    password: &'a str,
    role: &'a str,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub role: String,
    password: String,
}

pub enum UserAuthError {
    Conflict,
    Db(DbError),
}


impl Db{
    pub fn signup(&mut self,email:String,password:String,role:String)-> Result<String, UserAuthError> {
        
        let new_user = NewUser{
            email:&email,
            password:&password,
            role:&role
        };

        let result = diesel::insert_into(users::table)
            .values(new_user)
            .returning(User::as_returning())
            .get_result(&mut self.con)
            .map_err(|err| match err {
                DbError::DatabaseError(DatabaseErrorKind::UniqueViolation, _) => {
                    UserAuthError::Conflict
                }
                other => UserAuthError::Db(other),
            })?;

            Ok(result.id.to_string())
    }

    pub fn signin(&mut self,user_email:String, user_password:String)->QueryResult<Option<String> >{
        use crate::schema::users::dsl::*;
        let user = users.filter(email.eq(user_email))
        .select(User::as_select())
        .first::<User>(&mut self.con)
        .optional()?;

        let match_user = match user {
            Some(user)  if user.password == user_password => Ok(Some(user.id.to_string())),
            Some(_)=>Ok(None),
            None=>Ok(None)
        };

        match_user

    }

    pub fn user_profile(&mut self, uuid_id:String)->QueryResult<Option<User>>{
        use crate::schema::users::dsl::*;
        let user_id = Uuid::from_str(&uuid_id).unwrap();
        let user = users.filter(id.eq(user_id))
                .select(User::as_select())
                .first::<User>(&mut self.con)
                .optional()?;
        
        let match_user = match user {
            Some(user) => Ok(Some(user)),
            None => Ok(None)
        };

        match_user
    }
}