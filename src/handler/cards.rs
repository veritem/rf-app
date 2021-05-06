use std::env;

use diesel::result::Error;
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;

use crate::connection;
use crate::model::{Card, NewCard};
use crate::repository;

#[get("/")]
pub fn all_cards(connection: connection::DbConnection) -> Result<Json<Vec<Card>>, Status> {
    repository::get_all_cards(&connection)
        .map(|cards| Json(cards))
        .map_err(|error| error_status(error))
}

#[post("/", format = "application/json", data = "<new_card>")]
pub fn create_card(
    new_card: Json<NewCard>,
    connection: connection::DbConnection,
) -> Result<status::Created<Json<Card>>, Status> {
    repository::create_card(new_card.into_inner(), &connection)
        .map(|card| card_created(card))
        .map_err(|error| error_status(error))
}

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError,
    }
}

fn card_created(card: Card) -> status::Created<Json<Card>> {
    status::Created(
        format!(
            "{host}:{port}/cards/{id}",
            host = host(),
            port = port(),
            id = card.id
        )
        .to_string(),
        Some(Json(card)),
    )
}

fn host() -> String {
    env::var("ROCKET_ADDRESS").expect("ROCKET_ADDRESS  must be set")
}

fn port() -> String {
    env::var("ROCKET_PORT").expect("Rocket port must be set")
}
