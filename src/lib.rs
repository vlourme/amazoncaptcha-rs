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
//! let image = image::open("testdata/caggpa.jpg").unwrap();
//! 
//! let solver = new_solver();
//! let response = solver.resolve_image(&image).unwrap();
//! 
//! assert_eq!(response, "caggpa");
//! ```

use image::{DynamicImage, GenericImageView};
use std::collections::HashMap;
use std::error::Error;

/// Maximum length of a letter in pixels
const MAX_LENGTH: i32 = 32;

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
    /// let image = image::open("testdata/caggpa.jpg").unwrap();
    /// 
    /// let solver = new_solver();
    /// let response = solver.resolve_image(&image).unwrap();
    /// 
    /// assert_eq!(response, "caggpa");
    /// ```
    pub fn resolve_image(&self, image: &DynamicImage) -> Option<String> {
        let letters = self.extract_letters(image);
        let mut result = String::new();

        for letter_img in letters {
            let mut img = letter_img.grayscale();
            let img = img.as_mut_luma8()?;

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

    /// Extract letters from an image
    /// 
    /// This method will extract letters from an image and return them as a vector of images.
    fn extract_letters(&self, image: &DynamicImage) -> Vec<DynamicImage> {
        let (width, heigth) = image.dimensions();
        let mut black_cols = vec![false; width as usize];

        for x in 0..width {
            for y in 0..heigth {
                if image.get_pixel(x, y).0[0] == 0 {
                    black_cols[x as usize] = true;
                    break;
                }
            }
        }

        let mut letters: Vec<DynamicImage> = Vec::new();

        let mut start = -1;

        for x in 0..width {
            if black_cols[x as usize] {
                if start == -1 {
                    start = x as i32;
                }
            } else if start != -1 {
                let end = x as i32 - 1;

                if end - start < MAX_LENGTH {
                    let letter =
                        image.crop_imm(start as u32, 0, end as u32 + 1 - start as u32, heigth);

                    letters.push(letter);
                }

                start = -1;
            }
        }

        letters
    }
}
