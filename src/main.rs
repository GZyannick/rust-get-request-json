use reqwest::{self, header::HeaderMap};
use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

struct RequestGetter {
    content: Vec<u8>,
    headers: HeaderMap,
}

impl RequestGetter {
    fn new(url: &str) -> Self {
        let mut body = reqwest::blocking::get(url).unwrap();
        let mut content: Vec<u8> = Vec::new();
        body.read_to_end(&mut content).unwrap();
        Self {
            headers: body.headers().to_owned(),
            content,
        }
    }

    fn create_json_file(&self, path: &PathBuf) -> std::io::Result<()> {
        let systime = SystemTime::now();
        let current_time_as_millis = systime.duration_since(UNIX_EPOCH).unwrap().as_millis();

        let mut file_path = path.clone();
        let mut file_name = current_time_as_millis.to_string();
        file_name.push_str(".json");
        file_path.push(file_name);

        let mut f = File::create(file_path).unwrap();
        let _ = f.write_all(&self.content);

        Ok(())
    }
}

struct ArgsReader {
    url: String,
    path: PathBuf,
}
impl ArgsReader {
    fn get_args() -> Self {
        let url = match std::env::args().nth(1) {
            Some(url) => url,
            None => panic!("Expect url"),
        };

        if !url.contains("http") && !url.contains("://") {
            panic!("Cannot find a valid url");
        }

        let path = match std::env::args().nth(2) {
            Some(path) => PathBuf::from(path),
            None => PathBuf::from("./"),
        };

        Self { url, path }
    }
}

use std::time::{SystemTime, UNIX_EPOCH};
fn main() -> std::io::Result<()> {
    let args = ArgsReader::get_args();
    let request = RequestGetter::new(&args.url);
    if request
        .headers
        .get("content-type")
        .unwrap()
        .to_str()
        .unwrap()
        == "application/json"
    {
        let _ = request.create_json_file(&args.path);
    } else {
        println!(
            "headers: {:#?}, content: {:#?}",
            request.headers,
            String::from_utf8(request.content)
        );
    }
    Ok(())
}
