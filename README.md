dotnet core
dotnet run --property:Configuration=Release
wrk -c100 -d60s https://localhost:7028/weatherforecast
cpu: 100%
memory: 100mb

Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     5.79ms    4.49ms 151.76ms   95.26%
    Req/Sec     7.57k     1.25k    9.83k    81.04%
  877438 requests in 1.00m, 549.18MB read
Requests/sec:  14610.82
Transfer/sec:      9.14MB


rust - tide
wrk -c100 -d60s https://localhost:8080/weatherforecast
cpu: 100%
memory: 6mb