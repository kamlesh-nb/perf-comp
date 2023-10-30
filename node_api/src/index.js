
var rn = require('random-number');

const express = require('express')

const app = express();
const port = 3000;

let summaries = [
    "Freezing",
    "Bracing",
    "Chilly",
    "Cool",
    "Mild",
    "Warm",
    "Balmy",
    "Hot",
    "Sweltering",
    "Scorching"
];




app.get('/weather', (req, res) => {
 
    let forecasts = [];

    for (let index = 1; index < 6; index++) {
        let c = rn.generator({
            min: -20
            , max: 55
            , integer: true
        });
        let _c = c();
        let f = (32 + (_c / 0.5556));
        let e = rn.generator({
            min: 0
            , max: 9
            , integer: true
        });

        let forecast = {
            date: Date.now(),
            temperatureC: _c,
            temperatureF: f,
            summary: summaries[e()]
        };
        forecasts.push(forecast);
    }


    res.send(JSON.stringify(forecasts));

});

app.listen(port, () => console.log(`Weather Forecast app listening on port ${port}!`));



