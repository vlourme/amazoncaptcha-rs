//! Benchmark running on an infinite loop.

use amazon_captcha_rs::Solver;
use image::EncodableLayout;
use regex::Regex;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let solver = amazon_captcha_rs::new_solver();

    let mut resolved = 0;
    let mut total = 0;

    loop {
        let ok = resolve_captcha(&solver).await?;
        
        total += 1;
        if ok {
            resolved += 1;
        }

        if total % 10 == 0 {
            println!("Resolved: {}/{}", resolved, total);
            println!("Precision: {:.2}%", resolved as f32 / total as f32 * 100.0);
        }
    }
}


async fn resolve_captcha(solver: &Solver) -> Result<bool, Box<dyn std::error::Error>> {
    let text = reqwest::get("https://www.amazon.com/errors/validateCaptcha")
        .await?
        .text()
        .await?;

    
    let img_regex = Regex::new(r#"<img src="((.*).jpg)">"#)?;
    let cap = img_regex.captures(&text).unwrap();
    let url = cap.get(1).unwrap().as_str();

    let code_regex = Regex::new(r#"name="amzn" value="(.*?)" \/>"#).unwrap();
    let code = code_regex.captures(&text).unwrap().get(1).unwrap().as_str();

    let img = reqwest::get(url).await?.bytes().await?;
    let img = image::load_from_memory(img.as_bytes())?;

    let Some(result) = solver.resolve_image(&img) else {
        return Ok(false);
    };

    let response = reqwest::Client::new()
        .get("https://www.amazon.com/errors/validateCaptcha")
        .query(&[
            ("amzn", code),
            ("amzn-r", "/"),
            ("field-keywords", &result),
        ])
        .send()
        .await?;

    Ok(response.url().to_string() == "https://www.amazon.com/")
}