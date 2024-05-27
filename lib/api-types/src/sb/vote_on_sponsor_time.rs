// SPDX-License-Identifier: LGPL-3.0-only
//! POST `/api/voteOnSponsorTime`
//! https://wiki.sponsor.ajay.app/w/API_Docs#POST_/api/voteOnSponsorTime
use serde::{Serialize, Deserialize};

/// url params
#[derive(Serialize, Deserialize)]
pub enum VoteOnSponsorTimeRequest {
    Normal(NormalVote),
    Category(CategoryVote)
}

#[derive(Serialize, Deserialize)]
pub struct NormalVote {
    #[serde(rename = "UUID")]
    pub uuid: String,
    #[serde(rename = "userID")]
    pub user_id: String,
    // serialized to "type"
    pub r#type: i64
}

#[derive(Serialize, Deserialize)]
pub struct CategoryVote {
    #[serde(rename = "UUID")]
    pub uuid: String,
    #[serde(rename = "userID")]
    pub user_id: String,
    pub category: String
}