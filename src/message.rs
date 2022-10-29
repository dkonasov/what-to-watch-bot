use diesel::result::Error::NotFound;
use serde::{Serialize, Deserialize};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::{env, fs};
use crate::models::*;
use crate::schema::users::dsl::*;
use crate::step::Step;

#[derive(Serialize, Deserialize, Debug)]
pub struct Sender {
    pub id: i32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IncomingMessageInner {
    pub text: String,
    pub from: Sender
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IncomingMessage {
    pub message: IncomingMessageInner
}

pub struct OutgoingMessage {
    pub chat_id: i32,
    pub text: String,
}

impl OutgoingMessage {
    pub fn from_incoming(msg: &IncomingMessage) -> OutgoingMessage {
        let database_url = env::var("DATABASE_URL").unwrap();
        let mut connection = PgConnection::establish(&database_url).unwrap();
        let result = users
                                                        .filter(telegram_id.eq(msg.message.from.id))
                                                        .first::<User>(&mut connection);
        let user = match result {
            Ok(user) => user,
            Err(NotFound) => User { step_id: 0, telegram_id: msg.message.from.id, id: 0},
            Err(err) => panic!("{}", err)
        };
        
        let path_to_step = format!("./steps/{}.yml", user.step_id);
        let step: Step = serde_yaml::from_slice(&fs::read(path_to_step).unwrap()).unwrap();

        return OutgoingMessage{
            chat_id: msg.message.from.id,
            text: step.message
        };
    }
}