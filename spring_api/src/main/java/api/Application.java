package api;

import java.util.ArrayList;
import java.util.Random;
import java.time.LocalDateTime;
import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

@SpringBootApplication
@RestController
public class Application {

	private static String summaries[] = { "Freezing", "Bracing", "Chilly", "Cool", "Mild", "Warm", "Balmy", "Hot",
			"Sweltering",
			"Scorching" };

	@RequestMapping("/weather")
	public static ArrayList<WeatherForecast> getWeatherForecast() {
		ArrayList<WeatherForecast> alist = new ArrayList<WeatherForecast>();
		Random random = new Random();

		for (int a = 0; a < 6; a++) {
			int c = random.ints(0, 55).findFirst().getAsInt();
			int e = random.ints(0, 9).findFirst().getAsInt();
			Float f = (float) (32 + (int) (c / 0.5556));
			LocalDateTime dt = LocalDateTime.now();
			WeatherForecast wf = new WeatherForecast(dt, c, f, summaries[e]);
			alist.add(a, wf);
		}
		return alist;

	}

	public static void main(String[] args) {
		SpringApplication.run(Application.class, args);
	}

}
