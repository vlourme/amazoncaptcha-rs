//! Test the precision of the captcha solver

use std::{fs, time::Instant};

use amazon_captcha_rs::Solver;

fn main() {
    let solver = Solver::new().unwrap();

    let (mut resolved, mut total) = (0u32, 0u32);
    let mut times = Vec::new();

    for file in fs::read_dir("examples/dataset").unwrap() {
        let path = file.unwrap().path();
        let expect = path
            .file_name()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default()
            .split('.')
            .next()
            .unwrap_or_default();

        if expect.len() < 6 {
            return;
        }

        total += 1;

        let now = Instant::now();
        let result = solver.resolve_image(&image::open(&path).unwrap());
        times.push(now.elapsed().as_millis());

        if expect == result {
            resolved += 1;
        } else {
            eprintln!("{path:?}: Expected '{expect}', got '{result}'");
        }
    }

    println!(
        "Resolved: {resolved}/{total}\nPrecision: {:.2}%\nAverage Time: {:.2}ms",
        resolved as f32 / total as f32 * 100.0,
        times.iter().sum::<u128>() as f32 / times.len() as f32
    );
}
