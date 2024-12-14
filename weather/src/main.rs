extern crate dotenv;

use dotenv::dotenv;

use colored::*;
use serde::Deserialize;
use std::io;

// Struct to deserialize the JSON response from the OpenWeatherMap API
#[derive(Deserialize, Debug)]
struct WeatherResponse {
    weather: Vec<Weather>,
    main: Main,
    wind: Wind,
    name: String,
}

// Struct to represent weather description
#[derive(Deserialize, Debug)]
struct Weather {
    description: String,
}

// Struct to represent the main weather parameters
#[derive(Deserialize, Debug)]
struct Main {
    temp: f64, // deg C
    humidity: f64,
    pressure: f64, // atm.
}

// Struct to represent wind information
#[derive(Deserialize, Debug)]
struct Wind {
    speed: f64,
}

// function to get the weather information from OpenWeatherMap API
fn get_weather_info(
    city: &str,
    country_code: &str,
    api_key: &str,
) -> Result<WeatherResponse, reqwest::Error> {
    let url: String = format!(
        "http://api.openweathermap.org/data/2.5/weather?q={},{}&units=metric&appid={}",
        city, country_code, api_key
    );

    let response = reqwest::blocking::get(&url)?;
    let response_json: WeatherResponse = response.json::<WeatherResponse>()?;

    Ok(response_json)
}

// function to get emoji based on temperature
fn get_temp_emoji(temperature: f64) -> &'static str {
    if temperature < 0.0 {
        "❄️"
    } else if temperature >= 0.0 && temperature < 10.0 {
        "❄️"
    } else if temperature >= 10.0 && temperature < 20.0 {
        "⛅"
    } else if temperature >= 20.0 && temperature < 30.0 {
        "🌤️"
    } else {
        "🔥"
    }
}

// function to display the weather information
fn display_weather_info(response: &WeatherResponse) {
    // Extract the weather information from the response
    let description: &String = &response.weather[0].description;
    let temperature: f64 = response.main.temp;
    let humidity: f64 = response.main.humidity;
    let pressure: f64 = response.main.pressure;
    let wind_speed: f64 = response.wind.speed;

    // formatting weather information into a string
    let weather_text: String = format!(
        "Weather in {}: {} {}
        > Temperature: {:.1} °C,
        > Humidity: {:.1}%,
        > Pressure: {:.1} hPa,
        > Wind Speed: {:.1} m/s",
        response.name,
        description,
        get_temp_emoji(temperature),
        temperature,
        humidity,
        pressure,
        wind_speed,
    );

    // coloring the weather text based on weather conditions
    let weather_text_colored: ColoredString = match description.as_str() {
        "clear sky" => weather_text.bright_yellow(),
        "few clouds" | "scattered clouds" | "broken clouds" => weather_text.bright_blue(),
        "overcast clouds" | "mist" | "haze" | "smoke" | "sand" | "dust" | "fog" | "squalls" => {
            weather_text.dimmed()
        }
        "shower rain" | "rain" | "thunderstorm" | "snow" => weather_text.bright_cyan(),
        _ => weather_text.normal(),
    };

    // print the colored weather information
    println!("{}", weather_text_colored);
}

fn main() {
    dotenv().ok();

    // OpenWeatherMap API Key
    let open_weather_api_key =
        std::env::var("OPEN_WEATHER_API_KEY").expect("OPEN_WEATHER_API_KEY must be set");

    println!("{}", "Welcome to Weather Station!".bright_yellow());

    loop {
        // reading city
        println!("{}", "PLease enter the name of the city:".bright_green());
        let mut city = String::new();
        io::stdin()
            .read_line(&mut city)
            .expect("Failed to read input!");
        let city: &str = city.trim();

        // reading country
        println!(
            "{}",
            "Please enter the country code (e.g., US for United States):".bright_green()
        );
        let mut country_code = String::new();
        io::stdin()
            .read_line(&mut country_code)
            .expect("Failed to read input!");
        let country_code = country_code.trim();

        // calling the function to fetch weather information
        match get_weather_info(city, country_code, open_weather_api_key.as_str()) {
            Ok(response) => {
                display_weather_info(&response); // displaying weather information
            }
            Err(err) => {
                eprintln!("Error: {}", err); // printing error message in case of failure
            }
        }

        println!(
            "{}",
            "Do you want to search for weather in another city? (yes/no):".bright_green()
        ); // prompting user to continue or exit
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read input"); // reading user input for continuation
        let input = input.trim().to_lowercase();

        if input != "yes" {
            println!("Thank you for using your software!");
            break; // exiting the loop if the user doesn't want to continue
        }
    }
}
