//! Download captcha image to test precision

use std::{error, fs};

use regex::Regex;

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    println!("Downloading captcha images...");

    let img_regex = Regex::new(r#"<img src="((.*).jpg)">"#)?;

    for i in 0..100 {
        let text =
            reqwest::get("https://www.amazon.com/errors/validateCaptcha")
                .await?
                .text()
                .await?;

        let url = img_regex.captures(&text).unwrap().get(1).unwrap().as_str();

        fs::write(
            format!("examples/dataset/{i}.jpg"),
            reqwest::get(url).await?.bytes().await?,
        )?;
    }

    println!("Done!");

    Ok(())
}
