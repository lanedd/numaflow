/*
 * Numaflow
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: latest
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PersistenceStrategy : PersistenceStrategy defines the strategy of persistence



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PersistenceStrategy {
    /// Available access modes such as ReadWriteOnce, ReadWriteMany https://kubernetes.io/docs/concepts/storage/persistent-volumes/#access-modes
    #[serde(rename = "accessMode", skip_serializing_if = "Option::is_none")]
    pub access_mode: Option<String>,
    /// Name of the StorageClass required by the claim. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#class-1
    #[serde(rename = "storageClassName", skip_serializing_if = "Option::is_none")]
    pub storage_class_name: Option<String>,
    #[serde(rename = "volumeSize", skip_serializing_if = "Option::is_none")]
    pub volume_size: Option<k8s_openapi::apimachinery::pkg::api::resource::Quantity>,
}

impl PersistenceStrategy {
    /// PersistenceStrategy defines the strategy of persistence
    pub fn new() -> PersistenceStrategy {
        PersistenceStrategy {
            access_mode: None,
            storage_class_name: None,
            volume_size: None,
        }
    }
}


