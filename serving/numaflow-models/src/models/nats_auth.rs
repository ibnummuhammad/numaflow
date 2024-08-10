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

/// NatsAuth : NatsAuth defines how to authenticate the nats access

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NatsAuth {
    #[serde(rename = "basic", skip_serializing_if = "Option::is_none")]
    pub basic: Option<Box<crate::models::BasicAuth>>,
    #[serde(rename = "nkey", skip_serializing_if = "Option::is_none")]
    pub nkey: Option<k8s_openapi::api::core::v1::SecretKeySelector>,
    #[serde(rename = "token", skip_serializing_if = "Option::is_none")]
    pub token: Option<k8s_openapi::api::core::v1::SecretKeySelector>,
}

impl NatsAuth {
    /// NatsAuth defines how to authenticate the nats access
    pub fn new() -> NatsAuth {
        NatsAuth {
            basic: None,
            nkey: None,
            token: None,
        }
    }
}