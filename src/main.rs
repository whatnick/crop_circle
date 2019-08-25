use std::env;
use url::{Url,ParseError};

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Please supply image to apply crop-circle algorithm to");
    println!("{:?}", args);
    
    // HACK: With no inputs this causes a panic
    let image_path = &args[1];
    
    let parsed = match Url::parse(&image_path) {
        Ok(parsed) => parsed,
        Err(_) => panic!("Error parsing image path"),
    };

    assert!(parsed.scheme() == "https");

    match parsed.scheme() {
        "s3" => println!("Handling S3 Image"),
        "http"|"https" => println!("Handling Web Image"),
        "file" => println!("Handling Filesystem Image"),
        _ => panic!("Scheme not understood"),
    }
}
