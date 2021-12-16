use reqwest::StatusCode;

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

    println!("HTML: {}", raw_html);
}
