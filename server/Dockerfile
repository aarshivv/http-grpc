FROM rust:1.80 AS builder

# Creating working directory
WORKDIR /app
# Copying repository code inside image
COPY . .

RUN apt update -y && apt install -y protobuf-compiler
RUN cargo update
RUN cargo build --release

EXPOSE 6000

FROM ubuntu:22.04

# Required packages during runtime
ENV DEBIAN_FRONTEND=noninteractive
WORKDIR /app

EXPOSE 6000

COPY --from=builder /app/target/release/sample-grpc /app/sample-grpc

ENTRYPOINT ["/app/sample-grpc"]
