//! Amazon Captcha Solver
//!
//! This library has been highly inspired by:
//! - a-maliarov/amazoncaptcha (Python)
//! - gopkg-dev/amazoncaptcha (Go)
//!
//! Some methods are re-used from these libraries,
//! the dataset is also re-used from gopkg but
//! converted to uncompressed bincode format which makes
//! it much faster to load.
//!
//! # Example
//!
//! ```
//! use amazon_captcha_rs::new_solver;
//!
//! let image = image::open("examples/dataset/aatmag.jpg").unwrap();
//!
//! let solver = new_solver();
//! let response = solver.resolve_image(&image).unwrap();
//!
//! assert_eq!(response, "aatmag");
//! ```

use image::{imageops, DynamicImage, GenericImage, GrayImage};
use std::collections::HashMap;
use std::error::Error;

/// Solver implementation
///
/// Use `new_solver` to create a new instance
pub struct Solver {
    training_data: HashMap<String, char>,
}

/// Create a new solver instance
///
/// This method will load the training data.
pub fn new_solver() -> Solver {
    Solver {
        training_data: Solver::load_training_data().unwrap(),
    }
}

impl Solver {
    /// Load training data from dataset.bin
    ///
    /// This method will load the training data from the dataset.bin file
    /// which is included in the crate.
    fn load_training_data() -> Result<HashMap<String, char>, Box<dyn Error>> {
        let bytes = include_bytes!("../dataset.bin");
        let training_data: HashMap<String, char> = bincode::deserialize_from(&bytes[..])?;

        Ok(training_data)
    }

    /// Calculate the similarity between two binary strings
    ///
    /// This method is used when the exact binary string is not found in the training data.
    fn bin_similarity(a: &str, b: &str) -> f32 {
        let mut score = 0.0;

        for (a, b) in a.chars().zip(b.chars()) {
            if a == b {
                score += 1.0;
            }
        }

        score / a.len() as f32
    }

    /// Resolve a captcha image
    ///
    /// This method will try to resolve the captcha image and return the result as a string.
    ///
    /// # Example
    ///
    /// ```
    /// use amazon_captcha_rs::new_solver;
    ///
    /// let image = image::open("examples/dataset/cxkgmg.jpg").unwrap();
    ///
    /// let solver = new_solver();
    /// let response = solver.resolve_image(&image).unwrap();
    ///
    /// assert_eq!(response, "cxkgmg");
    /// ```
    pub fn resolve_image(&self, image: &DynamicImage) -> Option<String> {
        let mut letters = self.extract_letters(image);
        let mut result = String::new();

        if letters.len() == 7 {
            let last_letter = self.merge_images(vec![
                letters.last().unwrap().clone(),
                letters.first().unwrap().clone(),
            ]);

            letters.remove(0);
            letters[5] = last_letter;
        }

        for img in letters {
            let mut binary = String::new();

            for pixel in img.pixels() {
                if pixel.0[0] <= 1 {
                    binary.push('1');
                } else {
                    binary.push('0');
                }
            }

            let mut matching: Option<char> = None;
            self.training_data.iter().for_each(|(key, value)| {
                if key == &binary {
                    matching = Some(*value)
                }
            });

            if let Some(letter) = matching {
                result.push(letter);
            } else {
                let mut scores: HashMap<char, f32> = HashMap::new();
                self.training_data.iter().for_each(|(key, value)| {
                    let score = scores.get(value).unwrap_or(&0.0);
                    scores.insert(*value, score.max(Solver::bin_similarity(&binary, key)));
                });

                let mut max_score = 0.0;
                let mut max_letter: char = ' ';

                for (letter, score) in scores.iter() {
                    if score > &max_score {
                        max_score = *score;
                        max_letter = *letter;
                    }
                }

                result.push(max_letter);
            }
        }

        Some(result.to_lowercase())
    }

    /// Merge multiple images side by side
    fn merge_images(&self, images: Vec<GrayImage>) -> GrayImage {
        let mut width = 0;
        let mut height = 0;

        for image in &images {
            let (w, h) = image.dimensions();

            width += w;
            height = h.max(height);
        }

        let mut result = GrayImage::new(width, height);

        let mut x = 0;

        for image in images {
            let (w, _) = image.dimensions();

            result.copy_from(&image, x, 0).unwrap();

            x += w;
        }

        result
    }

    /// Extract letters from an image
    ///
    /// This method will extract letters from an image and return them as a vector of images.
    fn extract_letters(&self, image: &DynamicImage) -> Vec<GrayImage> {
        let image = imageops::grayscale(image);
        let (width, heigth) = image.dimensions();

        let mut rects: Vec<(u32, u32)> = Vec::new();
        let mut start: Option<u32> = None;

        for x in 0..width {
            let mut black = false;

            for y in 0..heigth {
                if image.get_pixel(x, y)[0] <= 1 {
                    black = true;
                    break;
                }
            }

            if black {
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

        let mut letters: Vec<GrayImage> = Vec::new();

        for (min_x, max_x) in rects {
            letters.push(imageops::crop_imm(&image, min_x, 0, max_x - min_x, heigth).to_image());
        }

        letters
    }
}
