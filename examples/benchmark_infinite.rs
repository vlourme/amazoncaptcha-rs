//! Benchmark running on an infinite loop.

use std::{error, time::Instant};

use amazon_captcha_rs::Solver;
use image::EncodableLayout;
use regex::Regex;
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let solver = Solver::new()?;

    let code_regex = Regex::new(r#"name="amzn" value="(.*?)" \/>"#)?;
    let img_regex = Regex::new(r#"<img src="((.*).jpg)">"#)?;

    let (mut resolved, mut total) = (0u32, 0u32);
    let mut times = Vec::new();

    loop {
        total += 1;

        let text =
            reqwest::get("https://www.amazon.com/errors/validateCaptcha")
                .await?
                .text()
                .await?;

        let code = code_regex.captures(&text).unwrap().get(1).unwrap().as_str();

        let url = img_regex.captures(&text).unwrap().get(1).unwrap().as_str();
        let img = image::load_from_memory(
            reqwest::get(url).await?.bytes().await?.as_bytes(),
        )?;

        let now = Instant::now();
        let result = solver.resolve_image(&img);
        times.push(now.elapsed().as_millis());

        if Client::new()
            .get("https://www.amazon.com/errors/validateCaptcha")
            .query(&[
                ("amzn", code),
                ("amzn-r", "/"),
                ("field-keywords", &result),
            ])
            .send()
            .await?
            .url()
            .to_string()
            == "https://www.amazon.com/"
        {
            resolved += 1;
        }

        if total % 10 == 0 {
            println!(
                "Resolved: {resolved}/{total}\nPrecision: {:.2}%\nAverage Time: {:.2}ms",
                resolved as f32 / total as f32 * 100.0,
                times.iter().sum::<u128>() as f32 / times.len() as f32
            );
        }
    }
}
