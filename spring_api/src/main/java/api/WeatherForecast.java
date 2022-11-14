package api;

import java.time.LocalDateTime;

public class WeatherForecast {

    public LocalDateTime Date;
    public int TemperatureC;
    public Float TemperatureF;
    public String Summary;

    public WeatherForecast(
        LocalDateTime Date,
        int TemperatureC,
        Float TemperatureF,
        String Summary
    ) {
        this.Date = Date;
        this.TemperatureC = TemperatureC;
        this.TemperatureF = TemperatureF;
        this.Summary = Summary;
    }
    
}
