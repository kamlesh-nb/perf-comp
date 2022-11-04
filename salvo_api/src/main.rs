use salvo::prelude::*;
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

 
#[tokio::main]
async fn main() {
    let router = Router::with_path("weatherforecast").get(get_weather_forecast);
 
    tracing_subscriber::fmt().init();

    tracing::info!("Listening on http://127.0.0.1:5041");

    match TcpListener::try_bind("0.0.0.0:5041") {
        Ok(listener) => Server::new(listener).serve(router).await,
        Err(e) => tracing::error!(error = ?e, "ddd"),
    }

}


#[handler]
async fn get_weather_forecast(_req: &mut Request, res: &mut Response) {

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
    
    res.render(serde_json::to_string(&forecast).unwrap());
}