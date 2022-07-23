use clap::Parser;
use reqwest::Url;
use exitfailure::ExitFailure;
use serde_derive::{Deserialize, Serialize};
extern crate chrono;
use chrono::prelude::*;

#[derive(Parser)]
struct Cli {
   city: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Forecast {
    coord: Coord,
    weather: Weather,
    base: String,
    main: Temps,
    visibility: i32,
    wind: Wind,
    clouds: Cloud,
    dt: i32,
    sys: Sys,
    id: i32,
    name: String,
    cod: i32,

}

#[derive(Serialize, Deserialize, Debug)]
struct Coord {
    lon: f64,
    lat: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Weather {
    details: Details,
}

#[derive(Serialize, Deserialize, Debug)]
struct Details {
    id: i32,
    main: String,
    description: String,
    icon: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Temps {
    temp: f64,
    pressure: f64,
    humidity: f64,
    temp_min: f64,
    temp_max: f64,
}

impl Temps {
    fn celsius(farenheit: &f64) -> f64{
        farenheit - 273.15
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Wind {
    speed: f64,
    deg: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Cloud {
    all: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Sys {
    r#type: f64,
    id: i32,
    country: String,
    sunrise: i64,
    sunset: i64,
}

impl Sys {
    fn datetime(timestamp: i64) -> DateTime<Utc> {
        let naive = NaiveDateTime::from_timestamp(timestamp, 0);
        let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
        return datetime
    }
}

impl Forecast {
    async fn get(city: &str, api_key: &str) -> Result<Self, ExitFailure> {
        let url: String = format!("https://api.openweathermap.org/data/2.5/weather?q={}&appid={}", city, api_key); 
        let url: Url = Url::parse(&*url)?;
        let resp = reqwest::get(url)
            .await?
            .json::<Forecast>()
            .await?;
        Ok(resp)
    }
}

fn remove_whitespace(s: &mut String) {
    s.retain(|c| !c.is_whitespace());
}

fn write_api_key(s: &String) {
    let path = get_key_path();
    std::fs::write(path, s).expect("Unable to write to file");
}

fn input_api_key() -> String {
    println!("You need to visit https://home.openweathermap.org/users/sign_in to create an account and generate a free api key");
    println!("Input api key: ");
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    remove_whitespace(&mut line);
    write_api_key(&line);

    return line
}

fn get_key_path() -> String {
    let path = std::env::current_exe().unwrap();
    let mystr = path.to_str().unwrap();
    let mut key_path = str::replace(mystr, "weather.exe", "key.txt");
    loop {
        if key_path.chars().nth(0).unwrap() == 'C' {
            break;
        }
        key_path.remove(0);
    }
    return key_path
}

fn get_api_key() -> String {
    let key_path = get_key_path();
    let contents = std::fs::read_to_string(key_path).expect("Unable to read file");
    if contents.len() == 0 {
        let contents = input_api_key();
        return contents
    }

    return contents
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let args = Cli::parse();
    let api_key = get_api_key();
    let result: Forecast = Forecast::get(&args.city, &api_key).await?;
    println!("Description: {}\nTemp: {:.2}\nTemp_max: {:.2}\nTemp_min: {:.2}\nWind_speed: {}\nWind_deg: {}\nSunrise: {}\nSunset: {}", result.weather.details.description, Temps::celsius(&result.main.temp), Temps::celsius(&result.main.temp_max), Temps::celsius(&result.main.temp_min), result.wind.speed, result.wind.deg, Sys::datetime(result.sys.sunrise).format("%H:%M:%S"), Sys::datetime(result.sys.sunset).format("%H:%M:%S"));
    Ok(())
}
