use std::io::{Write, Lines, BufReader, BufRead};
use std::fs::File;
use std::path::Path;
use std::error::Error;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, COOKIE};


pub fn echo(input: &str) {
    println!("{}", input);
}

// Helper function to read lines from a file
pub fn read_lines<P>(filename: P) -> std::io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

// Helper function that downloads the input file and writes it to an input.txt file
// in the directory of dayx
pub fn write_day_input_to_file(day: &str, session: &str) -> Result<String, Box<dyn Error>>
{
    let file_path = format!("day{}/input.txt", day);

    if Path::new(&file_path).exists() {
        return Ok(file_path);
    }

    let url = format!("https://adventofcode.com/2024/day/{}/input", day);
    let session_cookie = format!("session={}", session);
    let client = Client::new(); 
    let mut headers = HeaderMap::new();
    headers.insert(
        COOKIE,
        HeaderValue::from_str(session_cookie.as_str())?,
    );

    let response = client
        .get(url)
        .headers(headers)
        .send()?
        .error_for_status()?
        .text()?;


    let mut file = File::create(&file_path)?;
    file.write_all(response.as_bytes())?;
    Ok(file_path)
}
