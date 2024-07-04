use serde::{Deserialize, Serialize};

use sea_orm::{
    prelude::*,
    ActiveValue::{NotSet, Set},
};

use crate::models::thread::{ActiveModel, Model};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, Default)]
pub struct Thread {
    pub title: String,
    pub tag: String,
    pub author: i64,
    pub reviewer: i64,
    pub updated: ChronoDateTimeUtc,
    pub extra: Option<String>,
    pub content: String,
}

impl From<Thread> for ActiveModel {
    fn from(info: Thread) -> Self {
        Self {
            id: NotSet,
            tag: Set(info.tag),
            title: Set(info.title),
            author: Set(info.author),
            reviewer: Set(info.reviewer),
            updated: Set(chrono::Utc::now()),
            extra: Set(info.extra),
            content: Set(info.content),
        }
    }
}

impl From<Model> for Thread {
    fn from(val: Model) -> Self {
        Thread {
            title: val.title,
            tag: val.tag,
            author: val.author,
            reviewer: val.reviewer,
            updated: val.updated,
            extra: val.extra,
            content: val.content,
        }
    }
}
