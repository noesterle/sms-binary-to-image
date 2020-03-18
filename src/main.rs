extern crate lib;

use lib::xml_reader;

use std::fs::File;
use std::path::Path;

fn main() {
    //Take in location of sms.xml from user
    //Create Path object.
    //let path = Path::new("sms-20171201173215.xml.bak");
    let path = Path::new("sms-20171201173215.xml.bak.bak");
    //let path = Path::new("sms-test.xml");
    
    //Pass that off to XML mod, which reads through the XML.
    //Extracts text and image information
    //  The image information is then decoded from base64 and written to a file.
    xml_reader::read_xml(&path);
}
