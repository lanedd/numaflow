/*
Copyright 2022 The Numaproj Authors.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

// Code generated by Openapi Generator. DO NOT EDIT.

/// Backoff : Backoff holds parameters applied to a Backoff retry strategy.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Backoff {
    #[serde(rename = "cap", skip_serializing_if = "Option::is_none")]
    pub cap: Option<kube::core::Duration>,
    #[serde(rename = "duration", skip_serializing_if = "Option::is_none")]
    pub duration: Option<kube::core::Duration>,
    /// Duration is multiplied by factor each iteration, if factor is not zero and the limits imposed by Steps and Cap have not been reached. Should not be negative. The jitter does not contribute to the updates to the duration parameter.
    #[serde(rename = "factor", skip_serializing_if = "Option::is_none")]
    pub factor: Option<i32>,
    /// The sleep at each iteration is the duration plus an additional amount chosen uniformly at random from the interval between zero and `jitter*duration`.
    #[serde(rename = "jitter", skip_serializing_if = "Option::is_none")]
    pub jitter: Option<i32>,
    /// The remaining number of iterations in which the duration parameter may change (but progress can be stopped earlier by hitting the cap). If not positive, the duration is not changed. Used for exponential backoff in combination with Factor and Cap.
    #[serde(rename = "steps", skip_serializing_if = "Option::is_none")]
    pub steps: Option<i32>,
}

impl Backoff {
    /// Backoff holds parameters applied to a Backoff retry strategy.
    pub fn new() -> Backoff {
        Backoff {
            cap: None,
            duration: None,
            factor: None,
            jitter: None,
            steps: None,
        }
    }
}
