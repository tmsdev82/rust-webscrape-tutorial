use regex::Regex;
use reqwest::StatusCode;
use scraper::{Html, Selector};

mod utils;

#[tokio::main]
async fn main() {
    let client = utils::get_client();
    let url = "https://finance.yahoo.com";
    let result = client.get(url).send().await.unwrap();

    let raw_html = match result.status() {
        StatusCode::OK => result.text().await.unwrap(),
        _ => panic!("Something went wrong"),
    };

    let document = Html::parse_document(&raw_html);
    let article_selector = Selector::parse("a.js-content-viewer").unwrap();
    let h2select = Selector::parse("h2").unwrap();
    let h3select = Selector::parse("h3").unwrap();
    let get_text_re = Regex::new(r"->.*<").unwrap();
    let find_replace_re = Regex::new(r"[-><]").unwrap();

    for element in document.select(&article_selector) {
        let inner = element.inner_html().to_string();
        let mut h2el = element.select(&h2select);
        let mut h3el = element.select(&h3select);

        let href = match element.value().attr("href") {
            Some(target_url) => target_url,
            _ => "no url found",
        };

        match h2el.next() {
            Some(elref) => {
                println!("H2: {}", &elref.inner_html().to_string());
                println!("Link: {}", &href);
                continue;
            }
            _ => {}
        }

        match h3el.next() {
            Some(elref) => {
                println!("H3: {}", &elref.inner_html().to_string());
                println!("Link: {}", &href);
                continue;
            }
            _ => {}
        }

        match get_text_re.captures_iter(&inner).next() {
            Some(cap) => {
                let replaced = find_replace_re.replace_all(&cap[0], "");
                println!("Regex: {}", &replaced);
                println!("Link: {}", &href);
            }
            _ => {
                println!("Nothing found");
            }
        }
    }
}
