mod api;
mod services;

use api::{namespace_service_server::NamespaceServiceServer, policy_service_server::PolicyServiceServer};
use services::{namespace::inmemory::InMemoryNamespaceService, policy::inmemory::InMemoryPolicyService};
use tonic::transport::Server;

pub mod proto {
    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("namespace_service");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let namespace_service = InMemoryNamespaceService::default();
    let policy_service = InMemoryPolicyService::default();
    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
        .build_v1()?;

    println!("SDLC Control Plane gRPC listening on {}", addr);

    Server::builder()
        .add_service(NamespaceServiceServer::new(namespace_service))
        .add_service(PolicyServiceServer::new(policy_service))
        .add_service(reflection_service)
        .serve(addr)
        .await?;

    Ok(())
}
