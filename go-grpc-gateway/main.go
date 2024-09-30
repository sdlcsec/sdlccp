package main

import (
	"context"
	"log"
	"net/http"

	"github.com/grpc-ecosystem/grpc-gateway/v2/runtime"
	httpSwagger "github.com/swaggo/http-swagger"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"

	pb "github.com/sdlcsec/sdlccp/proto/gen/go/sdlc_control_plane/v1alpha1"
)

func main() {
	ctx := context.Background()
	ctx, cancel := context.WithCancel(ctx)
	defer cancel()

	mux := runtime.NewServeMux(
		// Below line is required to unescape the namespace URL path that is used by the namespace service
		runtime.WithUnescapingMode(runtime.UnescapingModeAllExceptReserved),
	)
	opts := []grpc.DialOption{grpc.WithTransportCredentials(insecure.NewCredentials())}
	err := pb.RegisterNamespaceServiceHandlerFromEndpoint(ctx, mux, "localhost:50051", opts)
	if err != nil {
		log.Fatalf("Failed to register gateway: %v", err)
	}
	// Serve the namespace OpenAPI file
	http.HandleFunc("/namespaces/openapi.json", func(w http.ResponseWriter, r *http.Request) {
		http.ServeFile(w, r, "../proto/gen/openapiv2/sdlc_control_plane/v1alpha1/namespace_service.swagger.json")
	})

	// Serve the policy OpenAPI file
	http.HandleFunc("/policies/openapi.json", func(w http.ResponseWriter, r *http.Request) {
		http.ServeFile(w, r, "../proto/gen/openapiv2/sdlc_control_plane/v1alpha1/policy_service.swagger.json")
	})

	// Serve Swagger UI for Namespace Service
	http.Handle("/swagger/namespace/", httpSwagger.Handler(
		httpSwagger.URL("http://localhost:8081/openapi/namespace.json"),
	))

	// Serve Swagger UI for Policy Service
	http.Handle("/swagger/policy/", httpSwagger.Handler(
		httpSwagger.URL("http://localhost:8081/openapi/policy.json"),
	))

	// Serve the gateway
	http.Handle("/", mux)

	log.Println("gRPC-Gateway listening on :8081")
	http.ListenAndServe(":8081", nil)
}
