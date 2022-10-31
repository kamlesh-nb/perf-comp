use chrono::{DateTime, Local, Duration};
use tide::http::mime;
use tide::prelude::*;
use tide::{Request, Response};
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


#[derive(Debug, Serialize, Deserialize)]
struct WeatherForecast {
    pub date: DateTime<Local>,
    pub temperature_c: i32,
    pub temperature_f: f64,
    pub summary: String,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/weatherforecast").get(get_weather_forecast);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
 
async fn get_weather_forecast(_req: Request<()>) -> tide::Result<impl Into<Response>> {

    let mut forecast: Vec<WeatherForecast> = Vec::new();

    for i in 1..6 {
        let c = rand::thread_rng().gen_range(-20..55);
        let f = f64::from(32) + (f64::from(c) / 0.5556);
        let e = rand::thread_rng().gen_range(0..9);
        let k = i as i64;

        forecast.insert(
            i - 1,
            WeatherForecast {
                date: chrono::Local::now() + Duration::days(k) ,
                temperature_c: c,
                temperature_f: f,
                summary: String::from(SUMMARIES.get(e).unwrap().clone())
            },
        );
       
    }

    Ok(Response::builder(200)
    .body(serde_json::to_string(&forecast)?)
    .content_type(mime::JSON))

}
