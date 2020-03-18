extern crate lib;

use lib::xml_reader;

use std::fs::File;
use std::path::Path;

fn main() {
    //println!("Hello, world!");
    //binary_to_image::test();
    //Take in location of sms.xml from user
    //Create File object.
    //let file = File::open("sms-20171201173215.xml").unwrap();
    //let path = Path::new("sms-20171201173215.xml.bak");
    let path = Path::new("sms-20171201173215.xml.bak.bak");
    //let path = Path::new("sms-test.xml");
    //Pass that off to XML mod
    xml_reader::read_xml(&path);
    //  Get relevant tags, ones that contain images.
    //  Pass those onto the binary_to_image mod
    //      Find out how to create proper image format.
}
