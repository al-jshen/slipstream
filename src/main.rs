#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
use rocket::response::Redirect;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/search/<query>")]
fn search(query: String) -> Redirect {
    let redir_url = "https://google.ca";
    Redirect::to(redir_url)
}

fn extract_command(query: &str) -> &str {
    if let Some(command) = query.split_whitespace().next() {
        return command;
    } else {
        return query;
    }
}

fn split_query(query: &str) -> Vec<&str> {
    query.split_ascii_whitespace().collect::<Vec<_>>()
}

fn main() {
    //rocket::ignite().mount("/", routes![index, search]).launch();
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_extract_command() {
        assert_eq!(extract_command("gh al-jshen/slipstream"), "gh");
        assert_eq!(extract_command("yt"), "yt");
        assert_eq!(extract_command("yt "), "yt");
        assert_eq!(extract_command(""), "");
    }

    #[test]
    fn test_split_query() {
        assert_eq!(
            split_query("gh al-jshen/slipstream"),
            vec!["gh", "al-jshen/slipstream"]
        );
        assert_eq!(
            split_query("gh al-jshen slipstream"),
            vec!["gh", "al-jshen", "slipstream"]
        );
        assert_eq!(split_query("yt"), vec!["yt"]);
        assert_eq!(split_query("yt "), vec!["yt"]);
    }
}
