#![allow(proc_macro_derive_resolution_fallback)]

use crate::schema::transactions;

#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug)]
#[table_name = "transactions"]
pub struct Transaction {
    pub id: i32,
    pub initial_balance: f64,
    pub transport_fare: f64,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "transactions"]
pub struct NewTransaction {
    pub initial_balance: f64,
    pub transport_fare: f64,
}
