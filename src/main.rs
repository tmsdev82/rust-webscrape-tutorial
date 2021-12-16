use reqwest::StatusCode;
use scraper::{ElementRef, Html, Selector};

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

    for element in document.select(&article_selector) {
        let inner = element.inner_html().to_string();
        let href = match element.value().attr("href") {
            Some(target_url) => target_url,
            _ => "no url found",
        };

        println!("Title: {}", inner);
        println!("Link: {}", &href);
    }
}
