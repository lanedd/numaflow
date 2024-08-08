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
pub struct UdSource {
    #[serde(rename = "container")]
    pub container: Box<crate::models::Container>,
}

impl UdSource {
    pub fn new(container: crate::models::Container) -> UdSource {
        UdSource {
            container: Box::new(container),
        }
    }
}