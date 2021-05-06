use rocket;

use crate::connection;
use crate::handler;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{ContentType, Header, Method, Status};
use rocket::{response, Request, Response};
use rust_embed::RustEmbed;
use std::{ffi::OsStr, io::Cursor, path::PathBuf};

#[derive(RustEmbed)]
#[folder = "frontend/build"]
struct AppUI;

pub struct CORS();

impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to requests",
            kind: Kind::Response,
        }
    }

    fn on_response(&self, request: &Request, response: &mut Response) {
        if request.method() == Method::Options || response.content_type() == Some(ContentType::JSON)
        {
            response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
            response.set_header(Header::new(
                "Access-Control-Allow-Methods",
                "POST, GET, OPTIONS",
            ));
            response.set_header(Header::new("Access-Control-Allow-Headers", "Content-Type"));
            response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
        }

        if request.method() == Method::Options {
            response.set_header(ContentType::Plain);
            response.set_sized_body(Cursor::new(""));
        }
    }
}

#[get("/")]
fn handle_frontend<'a>() -> response::Result<'a> {
    AppUI::get("index.html").map_or_else(
        || Err(Status::NotFound),
        |d| Response::build().sized_body(Cursor::new(d)).ok(),
    )
}

#[get("/<file..>", rank = 1)]
pub fn handle_frontend_assets<'r>(file: PathBuf) -> response::Result<'r> {
    let filename = file.display().to_string();
    AppUI::get(&filename).map_or_else(
        || Err(Status::NotFound),
        |d| {
            let ext = file
                .as_path()
                .extension()
                .and_then(OsStr::to_str)
                .ok_or_else(|| Status::new(400, "Could not get the file extension"))?;
            let content_type = ContentType::from_extension(ext)
                .ok_or_else(|| Status::new(400, "Could not get file content tpye"))?;
            response::Response::build()
                .header(content_type)
                .sized_body(Cursor::new(d))
                .ok()
        },
    )
}

pub fn create_routes() {
    rocket::ignite()
        .attach(CORS())
        .manage(connection::init_pool())
        .mount("/", routes![handle_frontend, handle_frontend_assets])
        .mount(
            "/cards",
            routes![handler::cards::create_card, handler::cards::all_cards],
        )
        .mount(
            "/transactions",
            routes![
                handler::transactions::all_transactions,
                handler::transactions::get_transaction,
                handler::transactions::create_transaction,
                handler::transactions::update_transaction,
                handler::transactions::delete_transaction
            ],
        )
        .launch();
}
