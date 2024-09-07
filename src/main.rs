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
    
    let html_generator = HtmlGenerator::from_ascii_image(ascii_image);
    html_generator.generate_output().expect("Failed to generate HTML file.");
}


struct Config {
    image_path: String,
}

impl Config {
    pub fn from_args(args: &[String]) -> std::io::Result<Self> {
        if args.len() == 2{
            let image_path = args.get(1).unwrap().clone();
            
            return Ok(Self{
                image_path,
            });
        }
        
        Err(std::io::Error::new(
            ErrorKind::InvalidInput, 
            "User provided the wrong amount of arguments."
        ))
    }
}
