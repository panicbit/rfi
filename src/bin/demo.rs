extern crate rfi;
extern crate dotenv;
extern crate term_painter;

use rfi::*;
use self::rfc::Rfc;
use dotenv::dotenv;
use github::State;
use term_painter::Color;
use errors::*;

fn main() {
    dotenv().ok();
    let token = github::Token::from_env_var("GH_TOKEN").expect("GH_TOKEN");
    let github = github::Client::new(token);
    let mut rfcs: Vec<_> = rfc::get_all("rfcs", &github)
        .unwrap()
        .inspect(|rfc| if let Ok(rfc) = rfc.as_ref() {
            println!("{:?}", rfc.short_title)
        })
        .collect::<Result<_>>()
        .unwrap_or_else(|err| panic!("{:#?}", err));

    rfcs.sort_by_key(|rfc| rfc.number.clone());

    for rfc in &rfcs {
        pretty_print_rfc(&rfc);
        // if rfc.state == State::Open {
        //     println!("- [{title}](https://github.com/rust-lang/rfcs/blob/master/text/{}-{title}.md)", rfc.number, title = rfc.short_title);
        // }
    }

    // let mut file = ::std::fs::File::create("rfcs.json").unwrap();
    // json::to_writer_pretty(&mut file, &rfcs).unwrap();

    // for rfc in  {
    //     let rfc = rfc.unwrap_or_else(|e| panic!("{:#?}", e));

    //     // pretty_print_rfc(&rfc);


    // }

    // print_rfc_table(&rfcs);

    // let rfc = rfc::Rfc::from_path("rfcs/text/0385-module-system-cleanup.md", &github).unwrap();
    // let rfc = rfc::Rfc::from_path("rfcs/text/2071-impl-trait-type-alias.md", &github).unwrap();
    // let rfc = rfc::Rfc::from_path("rfcs/text/0256-remove-refcounting-gc-of-t.md", &github).unwrap();
    // pretty_print_rfc(&rfc);
    // println!("{:#?}", rfc);
}

fn pretty_print_rfc(rfc: &Rfc) {
    use term_painter::Color::*;
    use term_painter::Attr::Plain;
    use term_painter::ToStyle;

    if rfc.issues.is_empty() {
        Blue.with(||
            println!("#{:04} {:<30}", rfc.number, rfc.short_title)
        );
        return
    }

    state_to_color(rfc.state).with(||
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

fn state_to_color(state: State) -> Color {
    match state {
        State::Open => Color::Green,
        State::Closed => Color::Red,
        State::Merged => Color::Magenta,
    }
}
