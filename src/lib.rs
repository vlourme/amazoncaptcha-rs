/*!
Amazon Captcha Solver

This library has been highly inspired by:
- a-maliarov/amazoncaptcha (Python)
- gopkg-dev/amazoncaptcha (Go)

Some methods are re-used from these libraries,
the dataset is also re-used from gopkg but
converted to uncompressed bincode format which makes
it much faster to load.

# Example

```
use amazon_captcha_rs::Solver;

let image = image::open("examples/dataset/aatmag.jpg").unwrap();

let solver = Solver::new().unwrap();
let response = solver.resolve_image(&image);

assert_eq!(response, "aatmag");
```
*/

use std::collections::HashMap;

use image::{imageops, DynamicImage, GenericImage, GrayImage};

/// Solver implementation
pub struct Solver {
    training_data: HashMap<String, char>,
}

impl Solver {
    /**
    Creates a new [`Solver`] using the training data from
    `dataset.bin` (included in the crate)

    # Errors

    Refer to [`bincode::Error`]
    */
    pub fn new() -> Result<Self, bincode::Error> {
        Ok(Solver {
            training_data: bincode::deserialize(include_bytes!(
                "../dataset.bin"
            ))?,
        })
    }

    /**
    Resolves a captcha image using training data

    # Example

    ```
    use amazon_captcha_rs::Solver;

    let image = image::open("examples/dataset/cxkgmg.jpg").unwrap();

    let solver = Solver::new().unwrap();
    let response = solver.resolve_image(&image);

    assert_eq!(response, "cxkgmg");
    ```
    */
    pub fn resolve_image(&self, image: &DynamicImage) -> String {
        let mut letters = extract_letters(image);

        if letters.len() == 7 {
            letters[6] = merge_images(&letters[6], &letters[0]);
            letters.remove(0);
        }

        let mut resolved = String::new();

        for img in letters {
            let binary = img
                .pixels()
                .map(|pixel| if pixel.0[0] <= 1 { '1' } else { '0' })
                .collect::<String>();

            resolved.push(
                *self
                    .training_data
                    .get(&binary)
                    .unwrap_or_else(|| self.most_similar_letter(&binary)),
            );
        }

        resolved.to_lowercase()
    }

    /// Returns most similar letter
    fn most_similar_letter(&self, letter: &str) -> &char {
        let mut max_letter = &' ';
        let mut max_score = usize::MIN;

        for (key, value) in &self.training_data {
            let score = letter
                .chars()
                .zip(key.chars())
                .filter(|(a, b)| a == b)
                .count();

            if score > max_score {
                max_score = score;
                max_letter = value;
            }
        }

        max_letter
    }
}

/// Merge two images side by side
fn merge_images(img1: &GrayImage, img2: &GrayImage) -> GrayImage {
    let (width1, height1) = img1.dimensions();
    let (width2, height2) = img2.dimensions();

    let mut merged_image =
        GrayImage::new(width1 + width2, height1.max(height2));

    merged_image.copy_from(img1, 0, 0).unwrap();
    merged_image.copy_from(img2, width1, 0).unwrap();

    merged_image
}

/// Extracts letters from an image
fn extract_letters(img: &DynamicImage) -> Vec<GrayImage> {
    let img = imageops::grayscale(img);
    let (width, height) = img.dimensions();

    let mut rects = Vec::new();
    let mut start = None;

    for x in 0..width {
        if (0..height).any(|y| img.get_pixel(x, y)[0] <= 1) {
            if start.is_none() {
                start = Some(x);
            }
        } else if let Some(point) = start {
            rects.push((point, x));
            start = None;
        }
    }

    if let Some(point) = start {
        rects.push((point, width));
    }

    rects
        .iter()
        .map(|(x1, x2)| {
            imageops::crop_imm(&img, *x1, 0, x2 - x1, height).to_image()
        })
        .collect()
}
