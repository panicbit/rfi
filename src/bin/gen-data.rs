extern crate rfi;
extern crate dotenv;
extern crate chrono;
extern crate serde_json as json;
#[macro_use] extern crate serde_derive;

use dotenv::dotenv;
use rfi::*;
use rfc::Rfc;
use errors::*;
use chrono::prelude::*;
use std::io;

#[derive(Serialize)]
struct Data {
    open_rfcs: Vec<Rfc>,
    unknown_rfcs: Vec<Rfc>,
    closed_rfcs: Vec<Rfc>,
    last_updated: String,
}

fn main() {
    dotenv().ok();
    let token = github::Token::from_env_var("GH_TOKEN").expect("GH_TOKEN");
    let github = github::Client::new(token);
    let last_updated = Utc::now().format("%Y-%m-%d %H:%M:%S %Z").to_string();
    let mut rfcs: Vec<_> = rfc::get_all("rfcs", &github)
        .unwrap()
        // .take(10)
        .collect::<Result<_>>()
        .unwrap_or_else(|err| panic!("{:#?}", err));
    rfcs.sort_by_key(|rfc| rfc.number.clone());

    let filtered_rfcs = |filter: fn(&Rfc) -> bool| {
        rfcs
        .iter()
        .cloned()
        .filter(filter)
        .collect()
    };

    let data = Data {
        open_rfcs: filtered_rfcs(Rfc::is_open),
        unknown_rfcs: filtered_rfcs(Rfc::is_unknown),
        closed_rfcs: filtered_rfcs(Rfc::is_closed),
        last_updated,
    };

    let stdout = io::stdout();
    let stdout = stdout.lock();

    json::to_writer(stdout, &data).unwrap();
}
