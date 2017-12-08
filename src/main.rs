extern crate lib;

use lib::binary_to_image;

use std::fs::File;

fn main() {
    println!("Hello, world!");
    binary_to_image::test();
    //Take in location of sms.xml from user
    //Create File object.
    //Pass that off to XML mod
    //  Get relevant tags, ones that contain images.
    //  Pass those onto the binary_to_image mod
    //      Find out how to create proper image format.
}
