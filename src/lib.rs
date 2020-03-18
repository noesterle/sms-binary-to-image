pub mod io_handler {
    extern crate base64;

    use self::base64::decode;
    use std::fs;
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;
    use std::path::PathBuf;

    static image_writing: bool = true;

    pub fn test() {
        println!("Hello Image");
    }

    pub fn create_image_dir() {
        fs::create_dir_all("images").expect("unable to create images directory");
    }

    pub fn convert_b64_to_image(b64: String) -> Vec<u8>{
        //print!("DECODING");
        return decode(b64).unwrap();
    }

    pub fn write_image_to_disk(img_path: &Path, img_contents: Vec<u8>) {
        //print!("WRITING");
        if(image_writing) {
            let mut ofile = File::create(&img_path).expect(&format!("Unable to open file: {}", img_path.display()));
            ofile.write_all(&img_contents);
            println!("{}",&img_path.display());
        }
    }

    pub fn find_new_filename(name: String) -> std::path::PathBuf {
        let mut buf = PathBuf::from(format!("images/{}",&name));
        let mut counter = 1;
        let buf = loop {
            if !(buf.as_path().exists()) {
                break buf;
            }
            buf = PathBuf::from(format!("images/{}{}", counter,&name));
            //println!("{} exists, generating new path.", temp_path.display());
            counter = counter + 1;
        };
        return buf;


        //let mut temp_path_str = format!("images/{}",&name);
        ////let mut temp_path = Path::new(&format!("images/{}",&name));
        //let mut temp_path = Path::new(&temp_path_str);
        //let mut counter = 1;
        //let img_path = loop {
        //   if !(temp_path.exists()){
        //        break temp_path;
        //    }
        //    temp_path_str = format!("images/{}{}",counter,&name);
        //    temp_path = Path::new(&temp_path_str);
        //    //println!("{} exists, generating new path.", temp_path.display());
        //    counter=counter+1;
        //};
    }
}

pub mod xml_reader {
    extern crate quick_xml;
    extern crate base64;

    use std::fs::File;
    use std::io::Write;
    use std::path::Path;
    use std::io::BufReader;
    //use self::xml::reader::{EventReader, XmlEvent};
    use self::quick_xml::reader::Reader;
    use self::quick_xml::events::Event;
    use std::iter;
    use io_handler::{convert_b64_to_image, write_image_to_disk, create_image_dir, find_new_filename};


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
        let debug_printing = false;
        //This crate does not allow me to retrieve specific attributes by calling the 'key' to get
        //the respective 'value'.
        //Will either need to figure out the pattern of what tags always have way attrs and what
        //the index is, or find a new crate that will allow key/value querying.
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    let attr = e.attributes();
                    //Print results
                    if (debug_printing){
                        for _ in 0..depth {
                            print!("{}",tabs)
                        }
                        println!("{:?} attrs: {:?}",String::from_utf8(e.name().to_vec()).unwrap(),
                            attr.fold(String::new(), |acc, a| acc.add(&a.unwrap().unescape_and_decode_value(&reader).unwrap().add("   ")))
                            //attr.count()
                            ); //Prints name of tag.
                    }
                    depth += 1;
                },
                Ok(Event::Eof) => break, // exits the loop when reaching end of file
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                Ok(Event::Text(e)) => {
                    if(debug_printing){
                        for _ in 0..depth {
                            print!("{}",tabs)
                        }
                    }
                    txt.push(e.unescape_and_decode(&reader).unwrap()); //Gets text between tags.
                    //println!("{:?}",String::from_utf8(e.name().to_vec()).unwrap());
                },
                Ok(Event::End(e)) => {
                    depth -= 1;
                    if(debug_printing){
                        for _ in 0..depth {
                            print!("{}",tabs)
                        }
                        println!("End");
                    }

                },
                Ok(Event::Comment(e)) => {
                    if(debug_printing) {
                        for _ in 0..depth {
                            print!("{}",tabs)
                        }
                        println!("Comment");
                    }
                },
                Ok(Event::Empty(e)) => {
                    //This is where smses are found.
                    //  Address is always the other persons number.
                    //  Date is the epoch*1000
                    //  Type='2' means sent, Type='1' means received
                    //  readable_time is the time the message was sent/received
                    //  Body is the message itself.
                    //This is where parts are found.
                    //  ct is the type (image/jpeg, text/plain, video/mp4, image/png)
                    //  data is the info.
                    //  name is the filename.
                    let attr = e.attributes();
                    let vec_attrs = attr.clone().map(|a| a.unwrap().unescape_and_decode_value(&reader).unwrap()).collect::<Vec<String>>();
                    if(debug_printing){
                       for _ in 0..depth {
                            print!("{}",tabs)
                        }
                        print!("Empty: ");
                        //println!("{:?}",String::from_utf8(e.name().to_vec()).unwrap()); //Prints name of tag.
                        println!("{:?} attrs: {:?}",String::from_utf8(e.name().to_vec()).unwrap(),
                            attr.fold(String::new(), |acc, a| acc.add(&a.unwrap().unescape_and_decode_value(&reader).unwrap().add("   ")))
                            //attr.count()
                            ); //Prints name of tag.
                    
                        for _ in 0..depth {
                            print!("{}",tabs)
                        }
                    }
                    let message_type = String::from_utf8(e.name().to_vec()).unwrap();
                    if(message_type == "sms"){
                        if(debug_printing){
                            println!("SMS");
                        }
                        parse_sms(&vec_attrs);
                    }
                    else if(message_type == "mms"){
                        if(debug_printing){
                            println!("MMS NOT IMPL");
                        }
                    }
                    else if(message_type == "part"){
                        if(debug_printing){
                            println!("PART");
                        }
                        parse_part(&vec_attrs);
                    }
                    else if(message_type == "addr"){

                    }
                    else if(message_type == "application/smil"){

                    }
                    else{
                        println!("--- FOUND UNKNOWN TAG TYPE: '{}' ---", message_type);
                    }
                },
                Ok(Event::CData(e)) => {
                    if(debug_printing){
                        println!("CData");
                    }

                },
                Ok(Event::Decl(e)) => {
                    if(debug_printing){
                        println!("Decl");
                    }
                },
                Ok(Event::PI(e)) => {
                    if(debug_printing){
                        println!("PI");
                    }
                },
                Ok(Event::DocType(e)) => {
                    if(debug_printing){
                        println!("DocType");
                    }
                }
                //_ => (),// There are several other `Event`s we do not consider here

            }
            count += 1;

            // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
            buf.clear();
        }
        for item in txt.iter() {
            println!("TXT {}",item);
        }
        println!("Count: {}",count);
    }

    fn parse_sms(vec_sms: &Vec<String>){
        let send = "1";
        let receive = "2";
        let send_or_receive = &vec_sms[3];
        let text = &vec_sms[5];
        let readable_date = &vec_sms[13];
        let contact_name = &vec_sms[14];
        
        if(send_or_receive == send) {
            println!("Received '{}' from {} at {}", text, contact_name, readable_date);
        }
        else if(send_or_receive == receive) {
            println!("Sent '{}' to {} at {}.",text, contact_name, readable_date);
        }

    }

    fn parse_part(vec_part: &Vec<String>){
        use std::fs;

        let jpg = "image/jpeg";
        let png = "image/png";
        let txt = "text/plain";
        let smil = "application/smil";
        let null = "null";

        let msg_type = &vec_part[1];
        let name = if &vec_part[2] == null {
            vec_part[7].clone()
        }
        else {
            vec_part[2].clone()
        };

        let img_path = find_new_filename(name);

        //println!("{}",msg_type);
        create_image_dir();
//        fs::create_dir_all("images").expect("unable to create images directory");

        if(msg_type == jpg){
            //println!("FOUND JPEG TO WRITE TO FILE");
            let mut jpg_contents = &vec_part[3];
            if jpg_contents == null && vec_part.len() >= 10 {
                jpg_contents = &vec_part[11];
            }

            let bytes = convert_b64_to_image(jpg_contents.to_string());
            write_image_to_disk(img_path.as_path(), bytes);
            //let mut ofile = File::create(&img_path).expect(&format!("Unable to open file: {}", img_path.display()));
            //ofile.write_all(jpg_contents.as_bytes());
            //println!("{}",&img_path.display());
        }
        else if(msg_type == png){
            //println!("FOUND PNG TO WRITE TO FILE");
            let png_contents = &vec_part[11];

            let bytes = convert_b64_to_image(png_contents.to_string());
            write_image_to_disk(img_path.as_path(), bytes);
            //let mut ofile = File::create(&img_path).expect(&format!("Unable to open file: {}", img_path.display()));
            //ofile.write_all(png_contents.as_bytes());
            //println!("{}",&img_path.display());
        }
        else if(msg_type == txt){
            //println!("FOUND TEXT OF AN IMAGE");
            println!("{}",&vec_part[10]);
        }
        else if(msg_type == smil){

        }
        else {
            println!("--- FOUND UNKNOWN MESSAGE TYPE: '{}' ---", msg_type);
        }
    }
}
