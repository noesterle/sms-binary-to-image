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

        let mut txt:Vec<String> = Vec::new();
        let mut buf:Vec<u8> = Vec::new();

        parse(&mut reader, &mut txt,&mut buf);

    }

    fn parse(mut reader: &mut quick_xml::reader::Reader<BufReader<File>>, mut txt: &mut Vec<String>, mut buf: &mut Vec<u8>) {
        use std::ops::Add;

        let mut count = 0;
        // The `Reader` does not implement `Iterator` because it outputs borrowed data (`Cow`s)
        let tabs = "    ".to_string();
        let mut depth = 0;
        //This crate does not allow me to retrieve specific attributes by calling the 'key' to get
        //the respective 'value'.
        //Will either need to figure out the pattern of what tags always have way attrs and what
        //the index is, or find a new crate that will allow key/value querying.
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    let attr = e.attributes();
                    //Print results
                    for _ in 0..depth {
                        print!("{}",tabs)
                    }
                    println!("{:?} attrs: {:?}",String::from_utf8(e.name().to_vec()).unwrap(),
                        attr.fold(String::new(), |acc, a| acc.add(&a.unwrap().unescape_and_decode_value(&reader).unwrap().add("   ")))
                        //attr.count()
                        ); //Prints name of tag.
                    depth += 1;
                },
                Ok(Event::Eof) => break, // exits the loop when reaching end of file
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                Ok(Event::Text(e)) => {
                    for _ in 0..depth {
                        print!("{}",tabs)
                    }
                    txt.push(e.unescape_and_decode(&reader).unwrap()); //Gets text between tags.
                    //println!("{:?}",String::from_utf8(e.name().to_vec()).unwrap());
                },
                Ok(Event::End(e)) => {
                    depth -= 1;
                    for _ in 0..depth {
                        print!("{}",tabs)
                    }
                    println!("End");

                },
                Ok(Event::Comment(e)) => {
                    for _ in 0..depth {
                        print!("{}",tabs)
                    }
                    println!("Comment");
                },
                Ok(Event::Empty(e)) => {
                    let attr = e.attributes();
                    for _ in 0..depth {
                        print!("{}",tabs)
                    }
                    print!("Empty: ");
                    //println!("{:?}",String::from_utf8(e.name().to_vec()).unwrap()); //Prints name of tag.
                    println!("{:?} attrs: {:?}",String::from_utf8(e.name().to_vec()).unwrap(),
                        attr.fold(String::new(), |acc, a| acc.add(&a.unwrap().unescape_and_decode_value(&reader).unwrap().add("   ")))
                        //attr.count()
                        ); //Prints name of tag.

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
