mod image_wrapper;
mod image_converter;

use std::env::args;
use std::io::ErrorKind;

use image_wrapper::{Image, ImageWrapper};
use image_converter::{ImageConverter, ImageToTextConverter};

fn main() {
    let user_args: Vec<String> = args().collect();
    let config = Config::from_args(user_args).unwrap();
    
    let image_wrapper = ImageWrapper::from_path(&config.image_path).unwrap();
    let mut converter = ImageToTextConverter::from_image_wrapper(image_wrapper);
    
    let ascii_image = converter.convert();
    println!("{ascii_image}");
}


struct Config {
    bin_path: String,
    image_path: String,
}

impl Config {
    pub fn from_args(args: Vec<String>) -> std::io::Result<Self> {
        if args.len() == 2{
            let bin_path = args.first().unwrap().clone();
            let image_path = args.get(1).unwrap().clone();
            
            return Ok(Self{
                bin_path,
                image_path,
            });
        }
        
        Err(
            std::io::Error::new(ErrorKind::InvalidInput, "User provided the wrong amount of arguments.")
        )
    }
}
