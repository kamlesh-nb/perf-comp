dotnet core
dotnet run --property:Configuration=Release
oha http://localhost:5041/weatherforecast -z 60s
cpu: 100%
memory: 100mb

Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     5.79ms    4.49ms 151.76ms   95.26%
    Req/Sec     7.57k     1.25k    9.83k    81.04%
  877438 requests in 1.00m, 549.18MB read
Requests/sec:  14610.82
Transfer/sec:      9.14MB


rust - tide
oha http://localhost:8080/weatherforecast -z 60s
cpu: 100%
memory: 6mb


rust - warp
oha http://localhost:8081/weatherforecast -z 60s

kubectl -n kubernetes-dashboard describe secret $(kubectl -n kubernetes-dashboard get secret |grep default-token | awk '{print $1}')
