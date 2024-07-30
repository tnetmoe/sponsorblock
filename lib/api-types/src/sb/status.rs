// SPDX-License-Identifier: LGPL-3.0-only
//! GET `/api/status/:value`
//! https://wiki.sponsor.ajay.app/w/API_Docs#GET_/api/status/:value
use serde::{Serialize, Deserialize};

/// payload
#[derive(Serialize, Deserialize)]
pub struct StatusResponseBody {
    pub uptime: Option<i64>,
    pub commit: Option<String>,
    pub db: Option<i64>,
    #[serde(rename = "startTime")]
    pub start_time: Option<i64>,
    #[serde(rename = "processTime")]
    pub process_time: Option<i64>,
    #[serde(rename = "redisProcessTime")]
    pub redis_process_time: Option<i64>,
    pub loadavg: Option<[f64; 2]>,
    #[serde(rename = "statusRequests")]
    pub status_requests: Option<i64>,
    pub hostname: Option<String>
}