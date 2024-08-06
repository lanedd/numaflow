/*
 * Numaflow
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: latest
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Gssapi : GSSAPI represents a SASL GSSAPI config



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Gssapi {
    /// valid inputs - KRB5_USER_AUTH, KRB5_KEYTAB_AUTH  Possible enum values:  - `\"KRB5_KEYTAB_AUTH\"` represents the password method KRB5KeytabAuth = \"KRB5_KEYTAB_AUTH\" = 2  - `\"KRB5_USER_AUTH\"` represents the password method KRB5UserAuth = \"KRB5_USER_AUTH\" = 1
    #[serde(rename = "authType")]
    pub auth_type: AuthType,
    #[serde(rename = "kerberosConfigSecret", skip_serializing_if = "Option::is_none")]
    pub kerberos_config_secret: Option<k8s_openapi::api::core::v1::SecretKeySelector>,
    #[serde(rename = "keytabSecret", skip_serializing_if = "Option::is_none")]
    pub keytab_secret: Option<k8s_openapi::api::core::v1::SecretKeySelector>,
    #[serde(rename = "passwordSecret", skip_serializing_if = "Option::is_none")]
    pub password_secret: Option<k8s_openapi::api::core::v1::SecretKeySelector>,
    #[serde(rename = "realm")]
    pub realm: String,
    #[serde(rename = "serviceName")]
    pub service_name: String,
    #[serde(rename = "usernameSecret")]
    pub username_secret: k8s_openapi::api::core::v1::SecretKeySelector,
}

impl Gssapi {
    /// GSSAPI represents a SASL GSSAPI config
    pub fn new(auth_type: AuthType, realm: String, service_name: String, username_secret: k8s_openapi::api::core::v1::SecretKeySelector) -> Gssapi {
        Gssapi {
            auth_type,
            kerberos_config_secret: None,
            keytab_secret: None,
            password_secret: None,
            realm,
            service_name,
            username_secret,
        }
    }
}

/// valid inputs - KRB5_USER_AUTH, KRB5_KEYTAB_AUTH  Possible enum values:  - `\"KRB5_KEYTAB_AUTH\"` represents the password method KRB5KeytabAuth = \"KRB5_KEYTAB_AUTH\" = 2  - `\"KRB5_USER_AUTH\"` represents the password method KRB5UserAuth = \"KRB5_USER_AUTH\" = 1
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AuthType {
    #[serde(rename = "KRB5_KEYTAB_AUTH")]
    KeytabAuth,
    #[serde(rename = "KRB5_USER_AUTH")]
    UserAuth,
}

impl Default for AuthType {
    fn default() -> AuthType {
        Self::KeytabAuth
    }
}

