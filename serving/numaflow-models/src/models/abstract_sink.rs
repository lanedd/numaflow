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
pub struct AbstractSink {
    #[serde(rename = "blackhole", skip_serializing_if = "Option::is_none")]
    pub blackhole: Option<Box<crate::models::Blackhole>>,
    #[serde(rename = "kafka", skip_serializing_if = "Option::is_none")]
    pub kafka: Option<Box<crate::models::KafkaSink>>,
    #[serde(rename = "log", skip_serializing_if = "Option::is_none")]
    pub log: Option<Box<crate::models::Log>>,
    #[serde(rename = "udsink", skip_serializing_if = "Option::is_none")]
    pub udsink: Option<Box<crate::models::UdSink>>,
}

impl AbstractSink {
    pub fn new() -> AbstractSink {
        AbstractSink {
            blackhole: None,
            kafka: None,
            log: None,
            udsink: None,
        }
    }
}