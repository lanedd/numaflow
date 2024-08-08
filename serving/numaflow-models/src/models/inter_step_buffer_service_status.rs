/*
 * Numaflow
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: latest
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InterStepBufferServiceStatus {
    /// Conditions are the latest available observations of a resource's current state.
    #[serde(rename = "conditions", skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition>>,
    #[serde(rename = "config", skip_serializing_if = "Option::is_none")]
    pub config: Option<Box<crate::models::BufferServiceConfig>>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "observedGeneration", skip_serializing_if = "Option::is_none")]
    pub observed_generation: Option<i64>,
    #[serde(rename = "phase", skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl InterStepBufferServiceStatus {
    pub fn new() -> InterStepBufferServiceStatus {
        InterStepBufferServiceStatus {
            conditions: None,
            config: None,
            message: None,
            observed_generation: None,
            phase: None,
            r#type: None,
        }
    }
}