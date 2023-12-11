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

    pub fn get_input(&self, day: u8) -> String {
        let cache = std::fs::read_to_string(format!("cache/{}.txt", day));
        if let Ok(data) = cache {
            return data;
        }

        let res = self
            .client
            .get(&format!("{}/{}/day/{}/input", BASE_URL, self.year, day))
            .header("Cookie", format!("session={}", self.token))
            .send()
            .unwrap();

        let data = res.text().unwrap();

        std::fs::create_dir("cache").unwrap();
        std::fs::write(format!("cache/{}.txt", day), &data).unwrap();

        data
    }
}
