#![warn(dead_code)]
use std::collections::HashMap;
use serde::Deserialize;
use dotenv;
// use std::{env, collections::HashMap};
use reqwest::{self, header::USER_AGENT};
 
#[derive(Debug, Deserialize)]
struct Response {
    articles: Vec<Article>,
}

#[derive(Debug, Deserialize)]
struct Article {
    source: String,
    author: String,
    title: String,
    desc: String,
    url: String,
    url_to_image: String,
    published_at: String,
    content: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // setup dotenv
    dotenv::from_filename(".env").ok();

    let api_key: String = dotenv::var("NEWS_API").unwrap();
    println!("sample env: {}", api_key);
    // TODO: figure out how to prompt user if api key is empty.

    // generate url with api key, if we have it.
    let url = generate_url(&api_key, "");

    // Name your user agent after your app?
    static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);
    let client = reqwest::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()?;
        println!("{:#?}", client);

    let res = client.get(url).send().await?;
 

    println!("{:#?}", res);
    Ok(())
}

fn generate_url(api_key: &str, query: &str) -> String {
    let mut base_url = "https://newsapi.org/v2/top-headlines?country=us".to_string();
    if !query.is_empty() {
        base_url.push_str(query);
    }
    if !api_key.is_empty() {
        base_url.push_str("&apiKey=");
        base_url.push_str(api_key);
    }

    println!("debug: {}", base_url);

    base_url
}

async fn _test(url: &str) -> Result<(), reqwest::Error> {
    let res = reqwest::get(url).await?.text().await?;
    println!("Status: {}", res);
    Ok(())
}

async fn _test_reqwest() -> Result<(), reqwest::Error> {
    let res = reqwest::get("http://httpbin.org/get").await?.text().await?;
    println!("Status: {}", res);
    Ok(())
}
