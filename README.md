# Rust Amazon Captcha Solver

![crates.io](https://img.shields.io/crates/v/amazon-captcha-rs.svg)

Welcome to the Rust Amazon Captcha Solver, an tool designed to resolve Amazon.com captchas without relying on Tesseract OCR or any external OCR software. Inspired by the exceptional work of [gopkg-dev/amazoncaptcha](https://github.com/gopkg-dev/amazoncaptcha) and [a-maliarov/amazoncaptcha](https://github.com/a-maliarov), this project builds upon their dataset, presenting it in an uncompressed bincode format.

## Key Features

- **Lightning-Fast:** Our solver boasts an impressive speed, resolving captchas in just around 7 milliseconds per image.
- **Pinpoint Accuracy:** Achieve precise captcha resolution with an accuracy rate of up to 98.5%.
- **Simplicity and Maintainability:** With a minimal codebase of only 200 lines of code, this project is easy to maintain and extend.

## How It Works
![Functional Schema](media/schema.png)

Amazon captchas exhibit a repetitive nature, with characters consistently less than 32 pixels in size, resulting in a finite number of possible combinations. We leverage the `dataset.bin` containing most of these possibilities. When a character doesn't match exactly, we employ a similarity comparison function for accurate resolution.

## Usage Example
```rust
use amazon_captcha_rs::Solver; 

let image = image::open("captcha.jpg").unwrap();

let solver = Solver::new().unwrap();
let response = solver.resolve_image(&image);

assert_eq!(response, "caggpa");
```

## Benchmarking
Starting from version 0.2.1, we've introduced a benchmarking feature to assess resolved captchas directly on Amazon. We conducted over 2000 tests with the following results:

```
Resolved: 1990/2020
Precision: 98.51%
```

You can run the benchmark using the following command:
```
cargo run -r --example benchmark_infinite
```

Additionally, we have a second benchmark performed on our proprietary labeled dataset located in `example/dataset`. This dataset contains only 100 images and yielded the following results:

```
Solved: 97/99
Precision: 97.98%
Average resolve time: 6.76ms
Total time: 0.75s
```

Execute this benchmark with the following command:
```
cargo run -r --example benchmark
```

## Changelog

### Version 0.2.1
- Improved documentation
- Added more benchmarking options

### Version 0.2.0
- Enhanced letter extraction method
- Introduced letter merging (used when the first letter is cropped with the last letter)
- Precision bumped up to 97%

### Version 0.1.0
- Initial release with a precision rate of 64%