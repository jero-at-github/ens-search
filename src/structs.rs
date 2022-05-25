use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryVariables {
    pub ids: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Query {
    pub query: String,
    pub variables: QueryVariables,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseDomain {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseRegistrations {
    pub id: String,
    pub registrationDate: String,
    pub expiryDate: String,
    pub domain: ResponseDomain,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseData {
    pub registrations: Vec<ResponseRegistrations>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub data: ResponseData,
}
