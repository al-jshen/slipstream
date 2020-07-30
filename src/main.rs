#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
use encoding::encode;
use rocket::response::Redirect;
use std::path::PathBuf;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/search/<query..>")]
fn search(query: PathBuf) -> Redirect {
    let url = process_query(query.to_str().unwrap().to_owned());
    Redirect::to(url)
}

fn process_query(query: String) -> String {
    let (cmd, args) = split_query(&query);
    match cmd {
        "ads" => process_ads(args),
        "gh" => process_github(args),
        _ => process_google([cmd, &args].join(" ")),
    }
}

fn process_google(args: String) -> String {
    let query = encode::encode(&args);
    format!("https://www.google.com/search?q={}", query)
}

fn process_github(args: String) -> String {
    format!("https://github.com/{}", args)
}

fn process_ads(args: String) -> String {
    let processed = args.replace("y:", "year:").replace("a:", "author:");
    let query = encode::encode(&processed);
    format!(
        "https://ui.adsabs.harvard.edu/search/q={}&sort=date%20desc%2C%20bibcode%20desc&p_=0",
        query
    )
}

fn split_query(query: &str) -> (&str, String) {
    let split = query.split_ascii_whitespace().collect::<Vec<_>>();
    let cmd = split.first().unwrap();
    let args = split[1..].join(" ");
    (cmd, args)
}

fn main() {
    //println!("{}", process_query("gh al-jshen/slipstream".to_owned()));
    rocket::ignite().mount("/", routes![index, search]).launch();
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_split_query() {
        assert_eq!(
            split_query("gh al-jshen/slipstream"),
            ("gh", "al-jshen/slipstream".to_owned()),
        );
        assert_eq!(
            split_query("gh al-jshen slipstream"),
            ("gh", "al-jshen slipstream".to_owned())
        );
        assert_eq!(split_query("yt"), ("yt", "".to_owned()));
    }
}
