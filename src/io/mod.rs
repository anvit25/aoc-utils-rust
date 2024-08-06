use reqwest::blocking;
use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

pub fn download_input(day: u8, year: u32) -> Result<String, Box<dyn Error>> {
    // read from the AOC_SESSION_COOKIE.PVT file

    if Path::exists(Path::new(&format!("inputs/{}.txt", day))) {
        return Ok(format!("Input for Day {day} already exists"));
    }

    let session_cookie = fs::read_to_string("AOC_SESSION_COOKIE.pvt");
    let session_cookie = match session_cookie {
        Ok(cookie) => cookie,
        Err(_) => {
            return Err("AOC_SESSION_COOKIE.pvt file not found".into());
        }
    };
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let client = blocking::Client::new();
    let res = client
        .get(url)
        .header("Cookie", format!("session={}", session_cookie))
        .send()?
        .text()?;

    std::fs::create_dir_all("inputs")?;
    let file = File::create(format!("inputs/{}.txt", day));
    let mut file = match file {
        Ok(file) => file,
        Err(_) => {
            return Err(format!("Error creating file for Day {day}").into());
        }
    };
    file.write_all(res.as_bytes())?;
    Ok(format!("Input for Day {day} downloaded successfully."))
}

pub fn lines_from_file(filename: &str) -> Vec<String> {
    std::fs::read_to_string(filename)
        .expect(format!("Cant read {filename}").as_str())
        .lines()
        .map(|s| s.to_string())
        .collect()
}