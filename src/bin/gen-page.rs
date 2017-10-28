extern crate rfi;
extern crate dotenv;
extern crate handlebars;
#[macro_use] extern crate serde_derive;

use dotenv::dotenv;
use rfi::*;
use rfc::Rfc;
use errors::*;
use handlebars::Handlebars;

#[derive(Serialize)]
struct Data {
    open_rfcs: Vec<Rfc>,
    unknown_rfcs: Vec<Rfc>,
    closed_rfcs: Vec<Rfc>,
}

fn main() {
    dotenv().ok();
    let token = github::Token::from_env_var("GH_TOKEN").expect("GH_TOKEN");
    let github = github::Client::new(token);
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

    let mut handlebars = Handlebars::new();

    let data = Data {
        open_rfcs: filtered_rfcs(Rfc::is_open),
        unknown_rfcs: filtered_rfcs(Rfc::is_unknown),
        closed_rfcs: filtered_rfcs(Rfc::is_closed),
    };

    handlebars.register_template_string("index", include_str!("../../index.handlebars")).unwrap();
    handlebars.register_partial("rfc_table", include_str!("../../rfc_table.handlebars")).unwrap();
    
    let index = handlebars.render("index", &data).unwrap();

    println!("{}", index);
}
