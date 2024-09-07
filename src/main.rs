mod image_wrapper;
mod image_converter;
mod output_generator;

use std::env::args;
use std::io::ErrorKind;

use image_wrapper::{Image, ImageWrapper};
use image_converter::{ImageConverter, ImageToTextConverter};
use output_generator::{OutputGenerator, HtmlGenerator};

fn main() {
    let user_args: &Vec<String> = &args().collect();
    let config = Config::from_args(user_args).unwrap();
    
    let image_wrapper = ImageWrapper::from_path(&config.image_path).unwrap();
    let mut converter = ImageToTextConverter::from_image_wrapper(image_wrapper);
    
    let ascii_image = converter.convert();
    println!("{ascii_image}");
    
    if config.generate_html {
        let html_generator = HtmlGenerator::from_ascii_image(ascii_image);
        html_generator.generate_output().expect("Failed to generate HTML file.");
    }
}


struct Config {
    image_path: String,
    generate_html: bool,
}

impl Config {
    pub fn from_args(args: &[String]) -> std::io::Result<Self> {
        match args.len() {
            2 => {
                let image_path = args.get(1).unwrap().clone();
                Ok(Self{
                    image_path,
                    generate_html: false,
                })
            },
            3 => {
                let image_path = args.get(1).unwrap().clone();
                let mut config = Self { image_path, generate_html: false };
                
                if let Some(arg) = args.get(2) {
                    if arg == "--html" {
                        config.generate_html = true;
                    }
                }
                
                Ok(config)
            },
            _ => {
                Err(std::io::Error::new(
                    ErrorKind::InvalidInput,
                    "User provided the wrong amount of arguments."
                ))
            }
        }
    }
}
