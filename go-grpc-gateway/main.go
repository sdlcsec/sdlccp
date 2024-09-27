package main

import (
	"context"
	"log"
	"net/http"

	"github.com/grpc-ecosystem/grpc-gateway/v2/runtime"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"

	pb "github.com/sdlcsec/sdlccp/proto/gen/go/sdlc_control_plane/v1alpha1"
	httpSwagger "github.com/swaggo/http-swagger/v2"
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
	// Serve the openapi.json file
	http.HandleFunc("/openapi.json", func(w http.ResponseWriter, r *http.Request) {
		http.ServeFile(w, r, "../proto/gen/openapiv2/sdlc_control_plane/v1alpha1/namespace_service.swagger.json")
	})

	// Serve Swagger UI
	http.Handle("/swagger/", httpSwagger.Handler(
		httpSwagger.URL("http://localhost:8081/openapi.json"),
	))

	// Serve the gateway
	http.Handle("/", mux)

	log.Println("gRPC-Gateway listening on :8081")
	http.ListenAndServe(":8081", nil)
}
