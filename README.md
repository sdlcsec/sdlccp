# SDLC Control Plane API

This is a repo for designing and prototyping a data model and API for an SDLC control plane. The intent of this control plane is to help manage software from development through to publishing. It is intended to provide a centralized platform for handling namespaces, policies, and other essential components that facilitate efficient and secure software development practices.

This project leverages gRPC for high-performance communication and uses Protocol Buffers (Protobufs) for interface definitions, ensuring consistency and interoperability across different services and languages. The RESTful API is exposed via grpc-gateway, allowing HTTP/JSON clients to interact seamlessly with the gRPC services.

## How to use this repo

The protobufs under `proto/` are the source of truth for the API. Everything else like the openapi and Rust example code are based on generated config/code.

## Running the code

The code available is intended to be just some example code and is in no way a production ready SDLC control plane.

### Installation

Prerequisites:

- `protoc` for building the protos into the Rust, Go, OpenAPI, etc. code and config.
- Buf for protobuf build configuration
- Rust/Cargo for running the grpc server.
- Go for running the grpc-gateway

### Building/Running

#### Generating code stubs

For generating the gRPC gateway code and OpenAPI schema:

```
cd proto/
buf generate
```

For building and running the Rust gRPC server:

```
cargo run
```

For building and running the grpc-gateway for HTTP proxying to the Rust gRPC server:

```
cd go-grpc-gateway
go run
```

### Developing

The general workflow for developing is:

1. Add or update a protobuf
2. Update the `api/mod.rs` file with any new proto imports.
3. Update the rust code
4. Generate the Go code and OpenAPI schema
5. Update the `go-grpc-gateway/main.go` file if needed, e.g. if you added a new service