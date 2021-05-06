#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;

use crate::model;
use crate::schema::transactions;
use crate::schema::transactions::dsl::*;

use rocket_contrib::uuid::Uuid;

pub fn create_transaction(
  new_trans: model::NewTransaction,
  connection: &PgConnection,
) -> QueryResult<model::Transaction> {
  diesel::insert_into(transactions::table)
    .values(&new_trans)
    .get_result(connection)
}

pub fn get_transactions(connection: &PgConnection) -> QueryResult<Vec<model::Transaction>> {
  transactions
    .limit(100)
    .load::<model::Transaction>(&*connection)
}

pub fn get_transaction(
  trans_id: Uuid,
  connection: &PgConnection,
) -> QueryResult<model::Transaction> {
  transactions::table
    .find(trans_id)
    .get_result::<model::Transaction>(connection)
}

pub fn update_transaction(
  trans_id: Uuid,
  transaction: model::Transaction,
  connection: &PgConnection,
) -> QueryResult<model::Transaction> {
  diesel::update(transactions::table.find(trans_id))
    .set(&transaction)
    .get_result(connection)
}

pub fn delete_transaction(transaction_id: Uuid, connection: &PgConnection) -> QueryResult<usize> {
  diesel::delete(transactions::table.find(transaction_id)).execute(connection)
}
