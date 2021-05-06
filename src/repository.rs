#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;

use crate::model::{Card, NewCard, NewTransaction, Transaction};
use crate::schema::cards::dsl::*;
use crate::schema::transactions::dsl::*;
use crate::schema::{cards, transactions};

pub fn create_transaction(
  new_trans: NewTransaction,
  connection: &PgConnection,
) -> QueryResult<Transaction> {
  diesel::insert_into(transactions::table)
    .values(&new_trans)
    .get_result(connection)
}

pub fn create_card(new_card: NewCard, connection: &PgConnection) -> QueryResult<Card> {
  diesel::insert_into(cards::table)
    .values(&new_card)
    .get_result(connection)
}

pub fn get_transactions(connection: &PgConnection) -> QueryResult<Vec<Transaction>> {
  transactions.limit(100).load::<Transaction>(&*connection)
}

pub fn get_all_cards(connection: &PgConnection) -> QueryResult<Vec<Card>> {
  cards.limit(100).load::<Card>(&*connection)
}

pub fn get_transaction(trans_id: i32, connection: &PgConnection) -> QueryResult<Transaction> {
  transactions::table
    .find(trans_id)
    .get_result::<Transaction>(connection)
}

pub fn update_transaction(
  trans_id: i32,
  transaction: Transaction,
  connection: &PgConnection,
) -> QueryResult<Transaction> {
  diesel::update(transactions::table.find(trans_id))
    .set(&transaction)
    .get_result(connection)
}

pub fn delete_transaction(transaction_id: i32, connection: &PgConnection) -> QueryResult<usize> {
  diesel::delete(transactions::table.find(transaction_id)).execute(connection)
}
