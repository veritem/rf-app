#![allow(proc_macro_derive_resolution_fallback)]

use crate::schema::cards;
use crate::schema::transactions;
use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug)]
#[table_name = "transactions"]
pub struct Transaction {
    pub id: i32,
    pub initial_balance: f64,
    pub transport_fare: f64,
    pub card_id: Uuid,
}

#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug)]
#[table_name = "cards"]
pub struct Card {
    pub id: Uuid,
    pub owner_names: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "transactions"]
pub struct NewTransaction {
    pub initial_balance: f64,
    pub transport_fare: f64,
    pub card_id: Uuid,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "cards"]
pub struct NewCard {
    pub owner_names: String,
}
