//! Download captcha image to test precision

use std::{fs::File, io::Write};
use regex::Regex;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Downloading captcha images...");

    for i in 0..100 {
        let img = download_captcha().await?;

        let path = format!("examples/dataset/{}.jpg", i);

        File::create(path)?.write_all(&img)?;
    }    

    println!("Done!");

    Ok(())
}


async fn download_captcha() -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let text = reqwest::get("https://www.amazon.com/errors/validateCaptcha")
        .await?
        .text()
        .await?;

    let re = Regex::new(r#"<img src="((.*).jpg)">"#)?;
    let cap = re.captures(&text).unwrap();
    let url = cap.get(1).unwrap().as_str();

    let img = reqwest::get(url).await?.bytes().await?;
    
    Ok(img.to_vec())
}