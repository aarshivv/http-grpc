# gRPC Over HTTP with Envoy

This repository is a sample demonstrating the necessary components to make a gRPC request from the browser.

## Envoy  
Envoy provides two main ways for a client (browser) to communicate with a gRPC server:  

### 1. HTTP/gRPC Bridge  
In the **bridge approach**, the client needs the `.proto` file because **protobuf serialization is done on the client side**. The client serializes the request using protobuf and sends it over HTTP/2, while Envoy ensures compatibility by handling **header and trailer conversions** that are not natively supported in HTTP.  

Envoy does not perform message conversion; instead, it ensures that gRPC's trailing headers are properly handled when translating between HTTP and gRPC. This means that the client and server both understand gRPC, but Envoy helps bridge the gap between the HTTP-based client and the gRPC server.  

In this repository, we are using the **bridge approach**, and **Envoy does not need the `.proto` file at all**—only the client and server require it. The client directly serializes the request using protobuf before sending it.  

### 2. gRPC-JSON Transcoding  
In the **transcoding approach**, the `.proto` file is required only by **Envoy**, and the client does not need it. The client can send **normal HTTP/1.1 or HTTP/2 JSON requests**, and Envoy automatically translates them into gRPC calls based on the HTTP annotations defined in the `.proto` file.  

This approach is useful when you want to expose gRPC services as RESTful APIs without requiring clients to handle gRPC-specific details. However, it requires modifying the `.proto` file to include HTTP annotations for the transcoding rules.  

### Implementation in This Repository  
In this repository, we are using the **HTTP/gRPC bridge approach**, where:  
- The client needs the `.proto` file and performs **protobuf serialization** before sending the request.  
- Envoy handles **header and trailer management** to ensure compatibility with gRPC.  
- Envoy **does not need the `.proto` file** in this setup, as it is only responsible for routing and proxying traffic.

The setup involves Kubernetes with Istio as a service mesh. Since Istio uses Envoy as its data plane proxy, we will be applying the Envoy configuration using an EnvoyFilter Custom Resource (CRD).

To make HTTP/2 call instead of HTTP/1.1, we will need certificate, for demonstrating purpose you can generate the certificate as follows:
```
░▒▓ ~ openssl req -x509 -nodes -days 365 -newkey rsa:2048 \
  -keyout tls.key \
  -out tls.crt \
  -subj "/CN=test.example.com" \
  -addext "subjectAltName = DNS:test.example.com" 
```

To use this certificate, we will have to add this as k8s secret:
```
░▒▓ ~ kubectl create secret tls grpc-web-tls-secret --cert=tls.crt --key=tls.key -n istio-system
```

I am using minikube for the demonstration, you can add `test.example.com` in the `/etc/hosts` to resolve it to the external IP of minikube.

In the config, we are referring the above secret.

To generate corresponding js file from proto, go to proto dir and run:
```
░▒▓ ~ protoc -I=. task_service.proto \
  --js_out=import_style=commonjs,binary:. \
  --grpc-web_out=import_style=commonjs,mode=grpcwebtext:.
```

The same proto dir is placed in client as well as server dir to ease the import. 

You can run the server by going to server dir and executing: ```cargo run```

You can run the client by going to client dir and executing: ```npm start```

Since Envoy handles HTTP-to-gRPC translation, you cannot start the gRPC server locally and call it directly from the browser. To verify that the server is working correctly, use `grpcurl`:
```
░▒▓ ~ grpcurl -plaintext -d '{
  "task_type": "some_value",
  "message": "Execute this task",
  "attributes": {"key1": "value1", "key2": "value2"}
}' 0.0.0.0:6000 taskservice.TaskService/ProcessTask
```

The client side code is only in: [App.js](client/src/App.js)

The primary purpose of this repository is to share the `EnvoyFilter` CRD, which you can modify and adapt to your requirements. 
