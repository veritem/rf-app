use rocket;

use crate::connection;
use crate::handler;

pub fn create_routes() {
    rocket::ignite()
        .manage(connection::init_pool())
        .mount(
            "/transactions",
            routes![handler::all_transactions, handler::create_transaction],
        )
        .launch();
}
