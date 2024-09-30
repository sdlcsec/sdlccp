# Examples


## gRPC

List namespaces

```shell
grpcurl -plaintext -d '{}' localhost:50051 sdlc_control_plane.v1alpha1.NamespaceService/ListNamespaces
```

## REST

Create new namespace:

```shell
curl -X POST -k http://localhost:8081/v1alpha1/namespaces -d '{"name": "acme/go/foo/bar", "display_name": "Acme Foo Bar", "owner": "john@example.com"}'
```

## Swagger

Go to `http://localhost:8081/swagger/namespace` or whatever port you're using.