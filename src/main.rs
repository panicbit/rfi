extern crate git2;
extern crate regex;
extern crate pulldown_cmark;
extern crate reqwest;
extern crate dotenv;
extern crate term_painter;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate error_chain;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate matches;

mod rfc;
mod errors;
mod github;

use self::rfc::Rfc;
use dotenv::dotenv;

fn main() {
    dotenv().ok();
    let token = github::Token::from_env_var("GH_TOKEN").expect("GH_TOKEN");
    let github = github::Client::new(token);

    for rfc in rfc::get_all("rfcs", &github).unwrap() {
        let rfc = rfc.unwrap_or_else(|e| panic!("{:#?}", e));
        pretty_print_rfc(&rfc);
    }

    // print_rfc_table(&rfcs);

    // let rfc = rfc::Rfc::from_path("rfcs/text/0385-module-system-cleanup.md", &github).unwrap();
    // let rfc = rfc::Rfc::from_path("rfcs/text/2071-impl-trait-type-alias.md", &github).unwrap();
    let rfc = rfc::Rfc::from_path("rfcs/text/0256-remove-refcounting-gc-of-t.md", &github).unwrap();
    pretty_print_rfc(&rfc);
    // println!("{:#?}", rfc);
}

fn pretty_print_rfc(rfc: &Rfc) {
    use github::State;
    use term_painter::Color::*;
    use term_painter::Attr::Plain;
    use term_painter::ToStyle;

    if rfc.issues.is_empty() {
        Blue.with(||
            println!("#{:04} {:<30}", rfc.number, rfc.short_title)
        );
        return
    }

    let color = rfc.issues.iter()
        .find(|t| t.state() == State::Open)
        .map(|_| Green)
        .unwrap_or(Red);

    color.with(||
        print!("#{:04} {:<30}", rfc.number, rfc.short_title)
    );

    print!(" |");

    for ticket in &rfc.issues {
        let color = match ticket.state() {
            State::Open => Green,
            State::Closed => Red,
            State::Merged => Magenta,
        };

        print!(" ");
        Plain.bg(color).with(||
            print!("#{:04}", ticket.number())
        );
    }


    println!();
}
