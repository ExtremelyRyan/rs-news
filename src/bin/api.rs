#![warn(dead_code)]
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Build the client using the builder pattern
    let client = reqwest::Client::builder()
        .build()?;

    // Perform the actual execution of the network request
    let res = client
        .get("https://httpbin.org/ip")
        .send()
        .await?;

    // Parse the response body as Json in this case
    let ip = res
        .json::<HashMap<String, String>>()
        .await?;

    println!("{:?}", ip);
    Ok(())
}

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct _Ip {
    _origin: String
}

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     // Build the client using the builder pattern
//     let client = reqwest::Client::builder()
//         .build()?;

//     // Perform the actual execution of the network request
//     let res = client
//         .get("https://httpbin.org/ip")
//         .send()
//         .await?;

//     // Parse the response body as Json in this case
//     let ip = res
//         .json::<Ip>()
//         .await?;

//     println!("{:?}", ip);
//     Ok(())
// }