#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;

use crate::model;
use crate::schema::transactions;
use crate::schema::transactions::dsl::*;

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
    .limit(10)
    .load::<model::Transaction>(&*connection)
}
