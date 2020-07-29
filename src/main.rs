#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
use rocket::response::Redirect;
use encoding::encode;


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

// #[get("/search/<query>")]
// fn search(query: String) -> Redirect {
//     let (cmd, args) = split_query(&query);
//     match cmd {
//         "ads" => process_ads(args), 
//         _ => process_google(args),
//     }
//     let redir_url = "https://google.ca";
//     Redirect::to(redir_url)
// }

fn process_google(args: String) -> String {
    let base_url = "https://www.google.com/search?q=";
    println!("{}", args);
    let query = encode::encode(&args);
    println!("{}", query);
    format!("{}{}", base_url, query)

}

fn split_query(query: &str) -> (&str, String) {
    let split = query.split_ascii_whitespace().collect::<Vec<_>>();
    let cmd = split.first().unwrap();
    let args = split[1..].join(" ");
    (cmd, args)
}

fn main() {
    let (cmd, args) = split_query("gh al-jshen/slipstream");
    let g_url = process_google(args);
    println!("{}", g_url);
    //rocket::ignite().mount("/", routes![index, search]).launch();
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
