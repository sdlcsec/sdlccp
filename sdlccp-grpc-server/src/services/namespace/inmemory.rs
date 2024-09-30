use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use tonic::{Request, Response, Status};

use crate::api::{
    namespace_service_server::NamespaceService, ApplyNamespaceRequest, ApplyNamespaceResponse, CreateNamespaceRequest, DeleteNamespaceRequest, GetNamespaceRequest, ListNamespacesRequest, ListNamespacesResponse, Namespace, UpdateNamespaceRequest
};

/// InMemoryNamespaceService is an in-memory implementation of the NamespaceService for testing purposes
pub struct InMemoryNamespaceService {
    namespaces: Arc<RwLock<HashMap<String, Namespace>>>,
}

impl Default for InMemoryNamespaceService {
    fn default() -> Self {
        Self {
            namespaces: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[tonic::async_trait]
impl NamespaceService for InMemoryNamespaceService {
    /// Create Namespace
    async fn create_namespace(
        &self,
        request: Request<CreateNamespaceRequest>,
    ) -> std::result::Result<Response<Namespace>, Status> {
        match request.into_inner().namespace {
            Some(namespace) => {
                let mut namespaces = match self.namespaces.write() {
                    Ok(namespaces) => namespaces,
                    Err(_) => return Err(Status::internal("Failed to acquire write lock")),
                };
                if namespaces.contains_key(&namespace.name) {
                    return Err(Status::already_exists("Namespace already exists"));
                }
                namespaces.insert(namespace.name.clone(), namespace.clone());
                Ok(Response::new(namespace))
            }
            None => Err(Status::invalid_argument("Namespace not provided")),
        }
    }
    /// Get Namespace
    async fn get_namespace(
        &self,
        request: Request<GetNamespaceRequest>,
    ) -> std::result::Result<Response<Namespace>, Status> {
        let name = request.into_inner().name;
        let namespaces = match self.namespaces.read() {
            Ok(namespaces) => namespaces,
            Err(_) => return Err(Status::internal("Failed to acquire read lock")),
        };
        match namespaces.get(&name) {
            Some(namespace) => Ok(Response::new(namespace.clone())),
            None => Err(Status::not_found("Namespace not found")),
        }
    }
    /// Update Namespace
    async fn update_namespace(
        &self,
        request: Request<UpdateNamespaceRequest>,
    ) -> std::result::Result<Response<Namespace>, Status> {
        let update_request = request.into_inner();
        let namespace = match update_request.namespace {
            Some(namespace) => namespace,
            None => return Err(Status::invalid_argument("Namespace not provided")),
        };
        let mut namespaces = match self.namespaces.write() {
            Ok(namespaces) => namespaces,
            Err(_) => return Err(Status::internal("Failed to acquire write lock")),
        };
        if let Some(existing) = namespaces.get_mut(&namespace.name) {
            *existing = namespace.clone();
            Ok(Response::new(namespace))
        } else {
            Err(Status::not_found("Namespace not found"))
        }
    }
    /// Delete Namespace
    async fn delete_namespace(
        &self,
        request: Request<DeleteNamespaceRequest>,
    ) -> std::result::Result<Response<()>, Status> {
        let name = request.into_inner().name;
        let mut namespaces = match self.namespaces.write() {
            Ok(namespaces) => namespaces,
            Err(_) => return Err(Status::internal("Failed to acquire write lock")),
        };
        if namespaces.remove(&name).is_some() {
            Ok(Response::new(()))
        } else {
            Err(Status::not_found("Namespace not found"))
        }
    }
    /// List/Search Namespaces
    async fn list_namespaces(
        &self,
        request: Request<ListNamespacesRequest>,
    ) -> std::result::Result<Response<ListNamespacesResponse>, Status> {
        let req = request.into_inner();
        let namespaces = match self.namespaces.read() {
            Ok(namespaces) => namespaces,
            Err(_) => return Err(Status::internal("Failed to acquire read lock")),
        };
        let filtered_namespaces: Vec<Namespace> = namespaces
            .values()
            .filter(|ns| {
                let mut matches = true;
                if !req.parent.is_empty() {
                    matches &= ns.parent == req.parent;
                }
                if !req.filter.is_empty() {
                    // TODO: Implement search logic
                }
                matches
            })
            .cloned()
            .collect();
        Ok(Response::new(ListNamespacesResponse {
            namespaces: filtered_namespaces,
            next_page_token: "".to_string(),
        }))
    }

    /// Apply Namespace
    async fn apply_namespace(
        &self,
        request: Request<ApplyNamespaceRequest>,
    ) -> std::result::Result<Response<ApplyNamespaceResponse>, Status> {
        // TODO: Implement apply logic
        Err(Status::unimplemented("ApplyNamespace not implemented"))
    }
}

#[cfg(test)]
mod tests {
    use prost_types::FieldMask;

    use super::*;
    use crate::api::{
        CreateNamespaceRequest, DeleteNamespaceRequest, GetNamespaceRequest, ListNamespacesRequest,
        UpdateNamespaceRequest,
    };

    #[tokio::test]
    async fn test_create_namespace() {
        let service = InMemoryNamespaceService::default();
        let request = Request::new(CreateNamespaceRequest {
            namespace: Some(Namespace {
                name: "test".to_string(),
                parent: "".to_string(),
                labels: Default::default(),
                display_name: "test".to_string(),
                owner: "john.smith@example.com".to_string(),
                policies: Vec::new(),
            }),
        });
        let response = service.create_namespace(request).await.unwrap();
        assert_eq!(response.get_ref().name, "test");
    }

    #[tokio::test]
    async fn test_get_namespace() {
        let service = InMemoryNamespaceService::default();
        // Prepopulate the namespace
        let create_request = Request::new(CreateNamespaceRequest {
            namespace: Some(Namespace {
                name: "test".to_string(),
                parent: "".to_string(),
                labels: Default::default(),
                display_name: "test".to_string(),
                owner: "john.smith@example.com".to_string(),
                policies: Vec::new(),
            }),
        });
        service.create_namespace(create_request).await.unwrap();

        // Now, get the namespace
        let request = Request::new(GetNamespaceRequest {
            name: "test".to_string(),
        });
        let response = service.get_namespace(request).await.unwrap();
        assert_eq!(response.get_ref().name, "test");
    }

    #[tokio::test]
    async fn test_update_namespace() {
        let service = InMemoryNamespaceService::default();
        // Prepopulate the namespace
        let create_request = Request::new(CreateNamespaceRequest {
            namespace: Some(Namespace {
                name: "test".to_string(),
                parent: "".to_string(),
                labels: Default::default(),
                display_name: "initial".to_string(),
                owner: "john.smith@example.com".to_string(),
                policies: Vec::new(),
            }),
        });
        service.create_namespace(create_request).await.unwrap();

        // Update the namespace
        let update_request = Request::new(UpdateNamespaceRequest {
            namespace: Some(Namespace {
                name: "test".to_string(),
                parent: "".to_string(),
                labels: Default::default(),
                display_name: "updated".to_string(),
                owner: "john.smith@example.com".to_string(),
                policies: Vec::new(),
            }),
            update_mask: Some(FieldMask {
                paths: vec!["display_name".to_string()],
            }),
        });
        let response = service.update_namespace(update_request).await.unwrap();
        assert_eq!(response.get_ref().name, "test");
        assert_eq!(response.get_ref().display_name, "updated");
    }

    #[tokio::test]
    async fn test_delete_namespace() {
        let service = InMemoryNamespaceService::default();
        // Prepopulate the namespace
        let create_request = Request::new(CreateNamespaceRequest {
            namespace: Some(Namespace {
                name: "test".to_string(),
                parent: "".to_string(),
                labels: Default::default(),
                display_name: "test".to_string(),
                owner: "john.smith@example.com".to_string(),
                policies: Vec::new(),
            }),
        });
        service.create_namespace(create_request).await.unwrap();

        // Delete the namespace
        let request = Request::new(DeleteNamespaceRequest {
            name: "test".to_string(),
        });
        let response = service.delete_namespace(request).await.unwrap();
        assert_eq!(response.get_ref(), &());

        // Verify deletion
        let get_request = Request::new(GetNamespaceRequest {
            name: "test".to_string(),
        });
        let result = service.get_namespace(get_request).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code(), tonic::Code::NotFound);
    }

    #[tokio::test]
    async fn test_list_namespaces() {
        let service = InMemoryNamespaceService::default();
        // Prepopulate the service with multiple namespaces
        for i in 0..5 {
            let create_request = Request::new(CreateNamespaceRequest {
                namespace: Some(Namespace {
                    name: format!("namespace{}", i),
                    parent: "".to_string(),
                    labels: Default::default(),
                    display_name: format!("Namespace {}", i),
                    owner: "john.smith@example.com".to_string(),
                    policies: Vec::new(),
                }),
            });
            service.create_namespace(create_request).await.unwrap();
        }

        // List the namespaces
        let request = Request::new(ListNamespacesRequest {
            parent: "".to_string(),
            filter: "".to_string(),
            page_size: 0,
            page_token: "".to_string(),
        });
        let response = service.list_namespaces(request).await.unwrap();
        assert_eq!(response.get_ref().namespaces.len(), 5);
    }
}