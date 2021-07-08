use std::fs;
use scraper::{Html, Selector};
use selectors::attr::CaseSensitivity;
use clap::{App, Arg};

#[derive(Default)]
struct RollResult {
    player_name: String,
    date: String,
    //     die: String,
    //     result: u8,
}

fn main() {
    // ARGs
    let matches = App::new("rust_test")
        .version("0.1.0")
        .author("Peter Forsythe")
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .takes_value(true)
                .help("HTML file to parse"),
        )
        .get_matches();

    // file setup
    let file = matches.value_of("file").unwrap_or(
        "/home/peter/Documents/git/data/Chat Log for The Seeds of Evil-small.html"
        // .unwrap_or("/home/peter/Documents/git/rust_test/data/Chat Log fragment.html");
    );

    // let's get to work
    println!("getting to work");
    let contents = fs::read_to_string(file).expect("Something went wrong reading the file");
    let html_doc = Html::parse_document(&contents);
    let div_message = Selector::parse(".message").unwrap();
    let div_selector = Selector::parse("div").unwrap();

    for message in html_doc.select(&div_message) {
        let fragment = Html::parse_fragment(&message.inner_html());     

        println!(
            "message.value().attr(general) {:?}",
            message
                .value()
                .has_class("general", CaseSensitivity::CaseSensitive)
        );

        for div in fragment.select(&div_selector) {
            println!("class {:?}", div.value().);
            let all_selector = Selector::parse("*").unwrap();
            for child in div.select(&all_selector) {
                println!("child {:?}", child.html());
            }
        }
        std::process::exit(2)
    }
}