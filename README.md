# Rust Amazon Captcha Solver
A attempt to resolve Amazon.com captchas without using Tesseract OCR. Highly inspired by [gopkg-dev/amazoncaptcha](https://github.com/gopkg-dev/amazoncaptcha) and [a-maliarov/amazoncaptcha](https://github.com/a-maliarov/amazoncaptcha). We reuse the dataset from the Go library but in a uncompressed bincode format.

We simplified the resolving process as much as possible, resulting in a less than 200 LoC library. Concerning speed, on a M1 Mac in release build, loading the library takes ~30ms (dataset loading) and resolving a captcha takes ~40ms.

## Functionnal schema
<img src="media/schema.png" align="center" />

> Amazon captcha are pretty repetitive, letter are always less than 32 pixel, there are not millions of combinaisons possible. The `dataset.bin` contains most of possibilities, when a letter is not matching exactly, we use a similarity comparison function.

## Example
```rs
use amazon_captcha_rs::new_solver; 

let image = image::open("testdata/caggpa.jpg").unwrap();

let solver = new_solver();
let response = solver.resolve_image(&image).unwrap();

assert_eq!(response, "caggpa");
```

## Precision
We downloaded and resolved 100 captcha in the `examples` directory to test the precision, actually it's not perfect, mostly due to some images being cropped uncorrectly:

```
Solved: 64/99
Precision: 64.65%
```