use std::collections::HashMap;

use rocket::request::{self, Request};
use rocket::data::{self, Data, FromData, ToByteUnit};
use rocket::http::{Status, ContentType};
use rocket::outcome::Outcome;
use serde::{Deserialize, Serialize};

// #[rocket::async_trait]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoginInformation {
    pub username: String,
    pub password: String
}

#[rocket::async_trait]
impl<'l> FromData<'l> for LoginInformation {
    type Error = Error;

    async fn from_data(_req: &'l Request<'_>, mut data: Data<'l>) -> data::Outcome<'l, Self> {
        // if data.peek_complete() {
        // }

        // println!("{:?}", data.peek(512).await);
        let result = data.peek(512).await.to_vec();
        // println!("{:?}", result);
        let result = result.iter().map(|x| (x.clone()) as char).collect::<String>();

        let result: HashMap<String, String> = serde_json::from_str(result.as_str()).unwrap();

        // println!("{result:?}");


        // let limit = req.limits().get("string").unwrap_or(256.bytes());

        // let result = data.open(limit).into_string().await {
        //     Ok(result) if result.is_complete() => result.into_inner(),
        //     Ok(_) => return Outcome::failure(Error::ParsingError),
        //     Err(_) => return Outcome::failure(Error::ParsingError)
        // };

        // println!("{:?}", data);

        Outcome::Success(LoginInformation {
            username: result.get("username").unwrap().clone(),
            password: result.get("password").unwrap().clone()
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum Error {
    Success,

    ParsingError,

    // NoUsername,
    // NoPassword,

    // UsernameNoExist,
    // PasswordWrong
}
