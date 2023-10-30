use ntex::web::{self, App, Error, HttpResponse};
use chrono::{DateTime, Local, Duration};
use rand::Rng;

static SUMMARIES: [&str; 10] = [
    "Freezing",
    "Bracing",
    "Chilly",
    "Cool",
    "Mild",
    "Warm",
    "Balmy",
    "Hot",
    "Sweltering",
    "Scorching",
    ];


#[derive(serde::Serialize)]
struct WeatherForecast {
    pub date: DateTime<Local>,
    pub temperature_c: i32,
    pub temperature_f: f64,
    pub summary: String,
}

async fn get_weather_forecast() -> Result<HttpResponse, Error> {

    let mut forecast = Vec::with_capacity(5);
    for i in 1..6 {
        let c = rand::thread_rng().gen_range(-20..55);
        let f = f64::from(32) + (f64::from(c) / 0.5556);
        let e = rand::thread_rng().gen_range(0..9);
        let k = i as i64;
        
        forecast.push(
            WeatherForecast {
                date: chrono::Local::now() + Duration::days(k) ,
                temperature_c: c,
                temperature_f: f,
                summary: String::from(SUMMARIES.get(e).unwrap().clone())
            },
        );
       
    }
    
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(&forecast))
}


#[ntex::main]
async fn main() -> std::io::Result<()> {
    let _logical_cpus = num_cpus::get();
    let address = "0.0.0.0:5041";
    println!("Running at {address}");

    web::server(|| App::new().service((web::resource("/weather").route(web::get().to(get_weather_forecast)),)))
        .bind(address)?
        .workers(_logical_cpus)
        .run()
        .await
}