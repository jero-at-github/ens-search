use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EnsQueryVariables {
    pub ids: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnsQuery {
    pub query: String,
    pub variables: EnsQueryVariables,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnsDomain {
    pub name: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct EnsRegistrations {
    pub id: String,
    pub registrationDate: String,
    pub expiryDate: String,
    pub domain: EnsDomain,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnsData {
    pub registrations: Vec<EnsRegistrations>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnsResponse {
    pub data: EnsData,
}
