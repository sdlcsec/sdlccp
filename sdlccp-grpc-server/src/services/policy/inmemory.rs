use std::{collections::HashMap, sync::Arc};

use tokio::sync::RwLock;
use tonic::{Request, Response, Status};

use crate::api::{
    policy_service_server::PolicyService, ApplyPolicyRequest, ApplyPolicyResponse, CreatePolicyRequest, DeletePolicyRequest, GetPolicyRequest, ListPoliciesRequest, ListPoliciesResponse, Policy, UpdatePolicyRequest
};

pub struct InMemoryPolicyService {
    namespaces: Arc<RwLock<HashMap<String, Policy>>>,
}

impl InMemoryPolicyService {
    pub fn new() -> Self {
        Self {
            namespaces: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for InMemoryPolicyService {
    fn default() -> Self {
        Self::new()
    }
}

#[tonic::async_trait]
impl PolicyService for InMemoryPolicyService {
    /// Create Policy
    async fn create_policy(
        &self,
        request: Request<CreatePolicyRequest>,
    ) -> std::result::Result<Response<Policy>, Status> {
        let mut namespaces = self.namespaces.write().await;
        match request.into_inner().policy {
            Some(policy) => {
                if namespaces.contains_key(&policy.id) {
                    return Err(Status::already_exists("Policy already exists"));
                }
                namespaces.insert(policy.id.clone(), policy.clone());
                Ok(Response::new(policy))
            }
            None => Err(Status::invalid_argument("Policy not provided")),
        }
    }

    /// Get Policy
    async fn get_policy(
        &self,
        request: Request<GetPolicyRequest>,
    ) -> std::result::Result<Response<Policy>, Status> {
        let namespaces = self.namespaces.read().await;
        match namespaces.get(&request.into_inner().id) {
            Some(policy) => Ok(Response::new(policy.clone())),
            None => Err(Status::not_found("Policy not found")),
        }
    }

    /// Update Policy
    async fn update_policy(
        &self,
        request: Request<UpdatePolicyRequest>,
    ) -> std::result::Result<Response<Policy>, Status> {
        let mut namespaces = self.namespaces.write().await;
        match request.into_inner().policy {
            Some(policy) => {
                if !namespaces.contains_key(&policy.id) {
                    return Err(Status::not_found("Policy not found"));
                }
                namespaces.insert(policy.id.clone(), policy.clone());
                Ok(Response::new(policy))
            }
            None => Err(Status::invalid_argument("Policy not provided")),
        }
    }

    /// Delete Policy
    async fn delete_policy(
        &self,
        request: Request<DeletePolicyRequest>,
    ) -> std::result::Result<Response<()>, Status> {
        let mut namespaces = self.namespaces.write().await;
        match namespaces.remove(&request.into_inner().id) {
            Some(_) => Ok(Response::new(())),
            None => Err(Status::not_found("Policy not found")),
        }
    }

    /// List/Search Policies
    async fn list_policies(
        &self,
        request: Request<ListPoliciesRequest>,
    ) -> std::result::Result<Response<ListPoliciesResponse>, Status> {
        let namespaces = self.namespaces.read().await;
        let policies: Vec<Policy> = namespaces.values().cloned().collect();
        // TODO: Implement search
        Ok(Response::new(ListPoliciesResponse {
            policies,
            // TODO: Implement pagination
            next_page_token: "".to_string(),
        }))
    }

    /// Apply Policy
    async fn apply_policy(
        &self,
        request: Request<ApplyPolicyRequest>,
    ) -> std::result::Result<Response<ApplyPolicyResponse>, Status> {
        // TODO: Implement apply logic
        Err(Status::unimplemented("ApplyPolicy not implemented"))
    }
}