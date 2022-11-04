docker build -t netcoreapi:0.1.0 -f netcore_api/Dockerfile  .
docker build -t salvoapi:0.1.0  .
docker build -t warpapi:0.1.0  .

wrk -t 6 -c 1000 -d 60s http://localhost:81/weatherforecast
wrk -t 6 -c 1000 -d 60s http://localhost:82/weatherforecast
wrk -t 6 -c 1000 -d 60s http://localhost:83/weatherforecast

kubectl apply -f deploy/netcoreapi/ -n netcore
kubectl apply -f deploy/salvoapi/ -n salvo
kubectl apply -f deploy/warpapi/ -n warp

kubectl logs metrics-server-847d45fd4f-6ddrn -n kube-system

watch -n 2 kubectl get hpa -n netcore 
watch -n 2 kubectl get pods -n netcore 
watch -n 2 kubectl get hpa -n salvo 
watch -n 2 kubectl get pods -n salvo   
watch -n 2 kubectl get hpa -n warp  
watch -n 2 kubectl get pods -n warp  




kubectl run apache-bench -i --tty --rm --image=httpd -- ab -n 500000 -c 1000 http://10.108.224.188:81/weatherforecast 

kubectl run apache-bench -i --tty --rm --image=httpd -- ab -n 500000 -c 1000 http://10.102.181.103:83/weatherforecast
