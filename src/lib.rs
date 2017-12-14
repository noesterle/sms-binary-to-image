pub mod binary_to_image {
    pub fn test() {
        println!("Hello Image");
    }
}

pub mod xml_reader {
    extern crate quick_xml;

    use std::fs::File;
    use std::path::Path;
    use std::io::BufReader;
    //use self::xml::reader::{EventReader, XmlEvent};
    use self::quick_xml::reader::Reader;
    use self::quick_xml::events::Event;


    pub fn read_xml(path: &Path) {
        println!("Reading xml");
        //let buf = BufReader::new(file);
        let xml = r#"<tag1 att1 = "test">
                        <tag2><!--Test comment-->Test</tag2>
                            <tag2>
                                Test 2
                            </tag2>
                     </tag1>"#;

        //let mut reader = Reader::from_str(xml);
        let mut reader = Reader::from_file(path).unwrap();
        reader.trim_text(true);

        let mut count = 0;
        let mut txt:Vec<String> = Vec::new();
        let mut buf:Vec<u8> = Vec::new();

        // The `Reader` does not implement `Iterator` because it outputs borrowed data (`Cow`s)
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => {
                        println!("{:?}",String::from_utf8(e.name().to_vec()).unwrap()); //Prints name of tag.
                        //println!("{:?}",e.name());
                        //match e.name() {
                        //    //b"sms" => println!("sms"),
                        //    //b"smses" => println!("smses"),
                        //    b"tag1" => println!("tag1"),
                        //    b"tag2" => println!("tag2"),
                        //    _ => continue,
                        //}
                },
                Ok(Event::Eof) => break, // exits the loop when reaching end of file
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                Ok(Event::Text(e)) => {
                    txt.push(e.unescape_and_decode(&reader).unwrap()); //Gets text between tags.
                    //println!("{:?}",String::from_utf8(e.name().to_vec()).unwrap());
                },
                Ok(Event::End(e)) => {
                    println!("End");

                },
                Ok(Event::Comment(e)) => {
                    println!("Comment");
                },
                Ok(Event::Empty(e)) => {
                    println!("Empty");

                },
                Ok(Event::CData(e)) => {
                    println!("CData");

                },
                Ok(Event::Decl(e)) => {
                    println!("Decl");

                },
                Ok(Event::PI(e)) => {
                    println!("PI");

                },
                Ok(Event::DocType(e)) => {
                    println!("DocType");

                }
                //_ => (),// There are several other `Event`s we do not consider here

            }
            count += 1;

            // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
            buf.clear();
        }
        for item in txt.iter() {
            println!("{}",item);
        }
        println!("Count: {}",count);
    }
}
