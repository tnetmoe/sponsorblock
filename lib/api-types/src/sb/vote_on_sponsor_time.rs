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
    uuid: String,
    #[serde(rename = "userID")]
    user_id: String,
    r#type: i64
}

#[derive(Serialize, Deserialize)]
pub struct CategoryVote {
    #[serde(rename = "UUID")]
    uuid: String,
    #[serde(rename = "userID")]
    user_id: String,
    category: String
}