#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
use encoding::encode;
use rocket::response::Redirect;
use std::path::PathBuf;

#[get("/")]
fn index() -> &'static str {
    "slipstream"
}

#[get("/help")]
fn help() -> &'static str {
    "
    Command \t Function            \t Examples \n
    !help   \t This help page      \t also !list, ?
    ads     \t NASA/ADS search     \t ads ^tacconi y:2013; ads carilli 2013 gas
    gh      \t Github profile/repo \t gh al-jshen/slipstream; gh al-jshen
    wk      \t Wikipedia search    \t wk united states; wk wikipedia
    yt      \t Youtube search      \t yt rust tutorial; yt marritza ds9
    maps    \t Google Maps search  \t maps new york city; maps VX2W+74; maps v0n 0a0; maps 32.4964474,-117.2286673
    stk     \t Stock ticker lookup \t stk fb; stk qqq
    "
}

#[get("/search/<query..>")]
fn search(query: PathBuf) -> Redirect {
    let url = process_query(query.to_str().unwrap().to_owned());
    Redirect::to(url)
}

fn process_query(query: String) -> String {
    let (cmd, args) = split_query(&query);
    match cmd {
        "!list" | "!help" | "?" => "/help".to_string(),
        "ads" => process_ads(args),
        "gh" => process_github(args),
        "wk" => process_wikipedia(args),
        "yt" => process_youtube(args),
        "maps" => process_maps(args),
        "stk" => process_stockprice(args),
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

fn process_wikipedia(args: String) -> String {
    let query = encode::encode(&args);
    format!("https://en.wikipedia.org/wiki/{}", query)
}

fn process_youtube(args: String) -> String {
    let query = encode::encode(&args);
    format!("https://www.youtube.com/results?search_query={}", query)
}

fn process_maps(args: String) -> String {
    let query = encode::encode(&args);
    format!("https://www.google.ca/maps/search/{}", query)
}

fn process_stockprice(args: String) -> String {
    let query = encode::encode(&args);
    format!(
        "https://finance.yahoo.com/quote/{}?p={}&.tsrc=fin-srch",
        query, query
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
    rocket::ignite()
        .mount("/", routes![index, help, search])
        .launch();
}
