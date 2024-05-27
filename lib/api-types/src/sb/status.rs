// SPDX-License-Identifier: LGPL-3.0-only
//! GET `/api/status/:value`
//! https://wiki.sponsor.ajay.app/w/API_Docs#GET_/api/status/:value
use serde::{Serialize, Deserialize};

/// payload
#[derive(Serialize, Deserialize)]
pub struct StatusResponse {
    pub uptime: i64,
    pub commit: String,
    pub db: i64,
    #[serde(rename = "startTime")]
    pub start_time: i64,
    #[serde(rename = "processTime")]
    pub process_time: i64,
    #[serde(rename = "redisProcessTime")]
    pub redis_process_time: i64,
    pub loadavg: [f64; 2],
    #[serde(rename = "statusRequests")]
    pub status_requests: i64,
    pub hostname: String
}