use chrono::{DateTime, Utc};
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

#[derive(Debug, Serialize, Deserialize)]
pub struct ExpiredDomain {
    pub domain_name: String,
    pub expiration_date: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessResult {
    pub unregistered_domains: Vec<String>,
    pub expired_domains: Vec<ExpiredDomain>,
}
