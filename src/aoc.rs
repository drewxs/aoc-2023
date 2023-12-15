use std::error::Error;
use std::fs;

use reqwest::blocking::Client;

const BASE_URL: &str = "https://adventofcode.com";

pub struct AOC {
    year: u16,
    token: String,
    client: Client,
}

impl AOC {
    pub fn new(year: u16, token: String) -> Self {
        let client = Client::new();
        Self {
            year,
            token,
            client,
        }
    }

    pub fn get_input(&self, day: u8) -> Result<String, Box<dyn Error>> {
        let cache = fs::read_to_string(format!("cache/day_{:02}.txt", day));
        if let Ok(data) = cache {
            return Ok(data);
        }

        let res = self
            .client
            .get(&format!("{}/{}/day/{}/input", BASE_URL, self.year, day))
            .header("Cookie", format!("session={}", self.token))
            .send()?;

        let data = res.text()?;

        fs::create_dir_all("cache")?;
        fs::write(format!("cache/day_{:02}.txt", day), &data)?;

        Ok(data)
    }
}
