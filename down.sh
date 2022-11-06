kubectl delete -f deploy/metrics/

kubectl delete -f deploy/apps/

kubectl delete -f https://raw.githubusercontent.com/kubernetes/ingress-nginx/controller-v1.4.0/deploy/static/provider/cloud/deploy.yaml

kubectl delete -f deploy/ingress/

kubectl delete -f https://raw.githubusercontent.com/kubernetes/dashboard/v2.6.1/aio/deploy/recommended.yaml

kubectl delete -f deploy/dashboard/