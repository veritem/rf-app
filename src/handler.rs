use std::env;

use diesel::result::Error;
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;

use crate::connection;
use crate::model;
use crate::model::NewTransaction;
use crate::repository;

#[get("/")]
pub fn all_transactions(
    connection: connection::DbConnection,
) -> Result<Json<Vec<model::Transaction>>, Status> {
    repository::get_transactions(&connection)
        .map(|transaction| Json(transaction))
        .map_err(|error| error_status(error))
}

#[post("/", format = "application/json", data = "<new_trans>")]
pub fn create_transaction(
    new_trans: Json<NewTransaction>,
    connection: connection::DbConnection,
) -> Result<status::Created<Json<model::Transaction>>, Status> {
    repository::create_transaction(new_trans.into_inner(), &connection)
        .map(|tran| transaction_created(tran))
        .map_err(|error| error_status(error))
}

#[get("/<id>")]
pub fn get_transaction(
    id: i32,
    connection: connection::DbConnection,
) -> Result<Json<model::Transaction>, Status> {
    repository::get_transaction(id, &connection)
        .map(|trans| Json(trans))
        .map_err(|error| error_status(error))
}

#[put("/<id>", format = "application/json", data = "<transaction>")]
pub fn update_transaction(
    id: i32,
    transaction: Json<model::Transaction>,
    connection: connection::DbConnection,
) -> Result<Json<model::Transaction>, Status> {
    repository::update_transaction(id, transaction.into_inner(), &connection)
        .map(|transaction| Json(transaction))
        .map_err(|err| error_status(err))
}

#[delete("/<id>")]
pub fn delete_transaction(
    id: i32,
    connection: connection::DbConnection,
) -> Result<status::NoContent, Status> {
    repository::delete_transaction(id, &connection)
        .map(|_| status::NoContent)
        .map_err(|error| error_status(error))
}

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError,
    }
}

fn transaction_created(
    transaction: model::Transaction,
) -> status::Created<Json<model::Transaction>> {
    status::Created(
        format!(
            "{host}:{port}/transactions/{id}",
            host = host(),
            port = port(),
            id = transaction.id
        )
        .to_string(),
        Some(Json(transaction)),
    )
}

fn host() -> String {
    env::var("ROCKET_ADDRESS").expect("ROCKET_ADDRESS  must be set")
}

fn port() -> String {
    env::var("ROCKET_PORT").expect("Rocket port must be set")
}
