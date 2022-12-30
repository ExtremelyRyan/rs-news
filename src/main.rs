use std::collections::HashMap;

use dotenv;
// use std::{env, collections::HashMap};
use reqwest::{self, header::{USER_AGENT}};

struct Article {
    source: String,
    author: String,
    title: String,
    desc: String,
    url: String,
    urlToImage: String,
    publishedAt: String,
    content: String,
}
 
 #[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{

    // setup dotenv
    dotenv::from_filename(".env").ok();

    let api_key: String= dotenv::var("NEWS_API").unwrap();
    println!("sample env: {}", api_key);
    // TODO: figure out how to prompt user if api key is empty.

    // generate url with api key, if we have it.
    let url = generate_url(&api_key, "");

    let client = reqwest::Client::new();
    let res = client
        .get(url) // 
        .header(USER_AGENT, "http_test") // NewsAPI needs a header, unknown requests not allowed.
        .send() // send the request.
        .await?
        .text()
        .await?; // wait for the results.
        //.json::<HashMap<String, String>>()
        //.await?;  
 
        println!("{:#?}", res);
        Ok(())


    // 
    // 
    //  test(&url).await?;
    // //vtest_reqwest().await?;
    // Ok(())
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
  
    println!("debug: {}",base_url);

    base_url
}

async fn test(url: &str)  -> Result<(), reqwest::Error> {
    let res = reqwest::get(url).await?
    .text().await?;
    println!("Status: {}", res);
    Ok(())
}


async fn test_reqwest()  -> Result<(), reqwest::Error> {
    let res = reqwest::get("http://httpbin.org/get").await?
    .text().await?;
    println!("Status: {}", res);
    Ok(())
}