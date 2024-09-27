mod api;
mod services;

use api::namespace_service_server::NamespaceServiceServer;
use services::namespace::inmemory::InMemoryNamespaceService;
use tonic::transport::Server;
use tonic_web::GrpcWebLayer;
use tower_http::cors::CorsLayer;

pub mod proto {
    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("namespace_service");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let namespace_service = InMemoryNamespaceService::default();
    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
        .build_v1()?;

    println!("NamespaceService listening on {}", addr);

    Server::builder()
        .add_service(NamespaceServiceServer::new(namespace_service))
        .add_service(reflection_service)
        .serve(addr)
        .await?;

    Ok(())
}
