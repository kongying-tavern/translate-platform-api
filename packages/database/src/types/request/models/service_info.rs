use std::collections::HashMap;

use anyhow::Context;
use serde::{Deserialize, Serialize};

use sea_orm::ActiveValue::{NotSet, Set};
use serde_json::json;

use crate::models::thread::{ActiveModel, Model};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, Default)]
pub struct ServiceInfo {
    pub target: String,
    pub system_type: String,

    pub description: String,
    pub projects: HashMap<String, Vec<ServiceInfoItem>>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ServiceInfoItem {
    pub name: String,
    pub description: Option<String>,

    pub related_device: Option<ServiceInfoRelatedDevice>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ServiceInfoRelatedDevice {
    pub id: Option<i64>,
    pub display: String,
}

// VO Subset

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
struct ServiceInfoSubset {
    pub system_type: String,
    pub projects: HashMap<String, Vec<ServiceInfoItem>>,
}

impl From<ServiceInfo> for ServiceInfoSubset {
    fn from(info: ServiceInfo) -> Self {
        Self {
            system_type: info.system_type,
            projects: info.projects,
        }
    }
}

// Converters between VO and DTO

impl From<ServiceInfo> for ActiveModel {
    fn from(info: ServiceInfo) -> Self {
        Self {
            id: NotSet,
            title: Set(info.target.clone()),
            tag: Set(json!(["service-table"]).to_string()),
            author: Set(1),
            reviewer: Set(1),
            updated: Set(chrono::Utc::now()),
            extra: Set(Some(
                serde_json::to_string(&ServiceInfoSubset::from(info.clone()))
                    .context("Failed to serialize ServiceInfo to JSON")
                    .unwrap(),
            )),
            content: Set(info.description),
        }
    }
}

impl From<Model> for ServiceInfo {
    fn from(val: Model) -> Self {
        let extra: ServiceInfoSubset = serde_json::from_str(
            val.extra
                .as_ref()
                .context("Failed to get extra from ServiceInfo")
                .unwrap()
                .as_str(),
        )
        .context("Failed to deserialize extra from ServiceInfo")
        .unwrap();

        ServiceInfo {
            target: val.title,
            system_type: extra.system_type,
            description: val.content,
            projects: extra.projects,
        }
    }
}
