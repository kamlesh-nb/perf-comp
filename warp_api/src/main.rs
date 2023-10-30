use tokio::runtime::Builder;
use warp::Filter;
use chrono::{DateTime, Utc, Local, Duration};
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

#[derive(serde::Serialize)]
struct TestStruct {
    pub time: DateTime<Utc>,
    pub name: String,
}

fn main() {
    let logical_cpu_cores = num_cpus::get();

    let runtime = Builder::new_multi_thread()
        .enable_all()
        .worker_threads(logical_cpu_cores)
        .build()
        .unwrap();

    // // Execute the future, blocking the current thread until completion
    runtime.block_on(async {
        let route = warp::path("weather")
            .and(warp::path::end())
            .map(|| warp::reply::json(&get_weather_forecast()));

        warp::serve(route).run(([0, 0, 0, 0], 5041)).await;

    });


}

fn get_weather_forecast() -> Vec<WeatherForecast> {

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
        forecast
}
