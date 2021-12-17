# Rust web scrape tutorial

The code in this repository represents a simple and basic example of how to scrape a website using Rust and the `scraper` crate.

The program downloads the front page of finance.yahoo.com as raw HTML and then uses `scraper` to select certain elements in the HTML containing article titles and links to articles. The data gathered is then serialized to a JSON file.

I have an article up on my blog describing the building of this program: (How to scrape websites with Rust: basic example)[https://tms-dev-blog.com/how-to-scrape-websites-with-rust-basic-example].

## Set up and run

The project can simply be run using `cargo run --release`, no additional configuration is needed.
