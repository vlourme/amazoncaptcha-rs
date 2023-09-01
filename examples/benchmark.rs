//! Test the precision of the captcha solver

use std::time::Instant;

extern crate amazon_captcha_rs;

fn main() {
    let start = Instant::now();
    let files = std::fs::read_dir("examples/dataset").unwrap();
    let solver = amazon_captcha_rs::new_solver();

    let mut solved = 0;
    let mut total = 0;
    let mut times = Vec::new();

    files.for_each(|file| {
        let now = Instant::now();
        let file = file.unwrap();
        let path = file.path();

        let expect = path.as_path().file_name()
            .unwrap().to_str().unwrap().split(".").next().unwrap();
        if expect.len() < 6 {
            return;
        }

        total += 1;
        let img = image::open(&path).unwrap();

        let Some(result) = solver.resolve_image(&img) else {
            println!("{:?}: Failed to resolve", &path.as_path());
            return;
        };

        if expect == result {
            solved += 1;
        } else {
            println!("{:?}: Expect '{}', got '{}'", &path.as_path(), expect, result);
        }

        times.push(now.elapsed().as_millis());
    });

    println!("Solved: {}/{}", solved, total);
    println!("Precision: {:.2}%", solved as f32 / total as f32 * 100.0);
    println!("Average time: {:.2}ms", times.iter().sum::<u128>() as f32 / times.len() as f32);
    println!("Total time: {:.2}s", start.elapsed().as_secs_f32());
}