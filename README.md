# HTTP TO GRPC (involve envoy)

This repository is a sample demonstrating the necessary components to make a gRPC request from the browser.

The setup involves Kubernetes with Istio as a service mesh. Since Istio uses Envoy as its data plane proxy, we will be applying the Envoy configuration using an EnvoyFilter Custom Resource (CRD).

To make HTTP/2 call instead of HTTP/1.1, we will need certificate, for demonstrating purpose you can generate the certificate as follows:
```
░▒▓ ~ openssl req -x509 -nodes -days 365 -newkey rsa:2048 \
  -keyout tls.key \
  -out tls.crt \
  -subj "/CN=localhost" \
  -addext "subjectAltName = DNS:localhost" 
```

To use this certificate, we will have to add this as k8s secret:
```
░▒▓ ~ kubectl create secret tls grpc-web-tls-secret --cert=tls.crt --key=tls.key -n istio-system
```

In the config, we are referring the above secret.

To generate corresponding js file from proto, go to proto dir and run:
```
protoc -I=. task_service.proto \
  --js_out=import_style=commonjs,binary:. \
  --grpc-web_out=import_style=commonjs,mode=grpcwebtext:.
```