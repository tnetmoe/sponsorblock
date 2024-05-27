// SPDX-License-Identifier: LGPL-3.0-only
//! GET `/api/status/:value`
//! https://wiki.sponsor.ajay.app/w/API_Docs#GET_/api/status/:value
use serde::{Serialize, Deserialize};

/// payload
#[derive(Serialize, Deserialize)]
pub struct StatusResponse {
    uptime: i64,
    commit: String,
    db: i64,
    #[serde(rename = "startTime")]
    start_time: i64,
    #[serde(rename = "processTime")]
    process_time: i64,
    #[serde(rename = "redisProcessTime")]
    redis_process_time: i64,
    loadavg: [f64; 2],
    #[serde(rename = "statusRequests")]
    status_requests: i64,
    hostname: String
}