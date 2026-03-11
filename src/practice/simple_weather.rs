/*
criteria
simple Weather stationl
Display all weather reports
Add a new weather reportlll
Display city weathereport
Update weather report
*/

use std::io;

pub fn run() {
    let mut sws = WeatherStation::new();
    loop {
        display_menu();

        match get_user_input_choice() {
            Ok(0) => println!("{}", sws.list_cites()),
            Ok(1) => add_city_weather(&mut sws).unwrap(),
            Ok(2) => display_city_weather(&mut sws).unwrap(),
            Ok(3) => update_city_weather(&mut sws).unwrap(),
            _ => {continue;}
        }
    }
}

fn update_city_weather(sws: &mut WeatherStation) ->Result<(), String> {
    println!("请输入城市:");
    let city = get_user_input();
    let city = city.trim().to_string();

    if sws.find_city(city.as_str()).is_none() {
        println!("No city");
        return Ok(())
    }

    println!("请输入温度:");
    let temperature = get_user_input();
    let temperature = temperature.trim().parse::<u8>().expect("无效的温度");

    println!("请输入湿度:");
    let humidity = get_user_input();
    let humidity = humidity.trim().parse::<u8>().expect("无效的湿度");

    println!("请输入天气:");
    let condition = get_weather_condition()?;

    sws.add_city(CityWeather { 
        city, 
        weather:  Weather::new(temperature, humidity, condition)
    });

    Ok(())
    
}

fn display_city_weather(sws: &mut WeatherStation) ->Result<(), String> {
    println!("请输入城市:");
    let city = get_user_input();
    let city = city.trim().to_string();

    match sws.find_city(city.as_str()) {
        Some(cw) => {
            println!("{}", cw.display());
        },
        None => println!("No city"),
    } 
    Ok(())
}

fn add_city_weather(sws: &mut WeatherStation) -> Result<(), String>{
    println!("请输入城市:");
    let city = get_user_input();
    let city = city.trim().to_string();

    println!("请输入温度:");
    let temperature = get_user_input();
    let temperature = temperature.trim().parse::<u8>().expect("无效的温度");

    println!("请输入湿度:");
    let humidity = get_user_input();
    let humidity = humidity.trim().parse::<u8>().expect("无效的湿度");

    println!("请输入天气:");
    let condition = get_weather_condition()?;

    sws.add_city(CityWeather { 
        city, 
        weather:  Weather::new(temperature, humidity, condition)
    });

    Ok(())

}

fn get_weather_condition() -> Result<WeatherCondition, String> {
    let input = get_user_input();
    match input.trim().to_lowercase().as_str() {
        "sunny" => Ok(WeatherCondition::Sunny),
        "cloudy" => Ok(WeatherCondition::Cloudy),
        "rainy" => Ok(WeatherCondition::Rainy),
        "snowy" => Ok(WeatherCondition::Snowy),
        _ => Err(format!("无效的天气条件: {}. 请输入 sunny/cloudy/rainy/snowy", input.trim())),
    }
}

fn display_menu() {
    const MENU: &str = r#"
========== Simple Weather Station ==========
0. 显示所有天气报告
1. 添加新的天气报告
2. 显示城市天气报告
3. 更新天气报告
===========================================
请输入您的选择 (0-3): "#;
    
    println!("{}", MENU);
}


fn get_user_input_choice() -> Result<u8, String> {
    let input = get_user_input();
    input.trim()
        .parse::<u8>()
        .map_err(|_|format!("无效数字: {}", input))
        .and_then(|n|{
            if n <= 3 {
                Ok(n)
            } else {
                Err(format!("选项 {} 超出范围 (0-3)", n))
            }
        })
}

fn get_user_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("read user input fail");
    input
}

#[derive(Default)]
enum WeatherCondition {
    #[default]
    Sunny,
    Cloudy,
    Rainy,
    Snowy,
}

impl WeatherCondition {
    fn display(&self) -> String {
        match self {
            WeatherCondition::Sunny => "Sunny".to_string(),
            WeatherCondition::Cloudy => "Cloudy".to_string(),
            WeatherCondition::Rainy => "Rainy".to_string(),
            WeatherCondition::Snowy => "Snowy".to_string(),
        }
    }
}

#[derive(Default)]
struct Weather {
    temperature: u8,
    humidity: u8,
    condition: WeatherCondition,
}
impl Weather {
    fn new(temperature: u8, humidity: u8, condition: WeatherCondition) -> Self {
        Self {
            temperature,
            humidity,
            condition,
        }
    }
    fn update_temperature(&mut self, temperature: u8) {
        self.temperature = temperature
    }
    fn update_humidity(&mut self, humidity: u8) {
        self.humidity = humidity
    }
    fn update_condition(&mut self, condition: WeatherCondition) {
        self.condition = condition
    }
    fn disply(&self) -> String {
        format!(
            "temperature: {}°C humidity: {}%, weather condition: {}",
            self.temperature,
            self.humidity,
            self.condition.display(),
        )
    }
}

struct CityWeather {
    city: String,
    weather: Weather,
}

impl CityWeather {
    fn new(city: String, weather: Weather) -> Self {
        Self { city, weather }
    }
    fn update_weather(&mut self, weather: Weather) {
        self.weather = weather
    }
    fn display(&self) -> String {
        // println!("len: {}", self.city.len());
        format!("City:{} {}", self.city, self.weather.disply())
    }
}

struct WeatherStation {
    cities: Vec<CityWeather>,
}

impl WeatherStation {
    fn new() -> Self {
        WeatherStation {
            cities: Vec::with_capacity(10),
        }
    }
    fn add_city(&mut self, cw: CityWeather) {
        match self.cities.iter().position(|x| x.city == cw.city) {
            Some(index) => self.cities[index] = cw,
            None => self.cities.push(cw),
        }
    }
    fn find_city(&self, city: &str) -> Option<&CityWeather> {
        self.cities.iter().find(|cw| cw.city == city)
    }
    fn list_cites(&self) -> String {
        if self.cities.is_empty(){
            return "empty citys".to_string();
        }
        self.cities
            .iter()
            .map(|cw| cw.display())
            .collect::<Vec<_>>()
            .join("\n")
    }
}
