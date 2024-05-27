// SPDX-License-Identifier: LGPL-3.0-only
//! https://wiki.sponsor.ajay.app/w/API_Docs
// In the order as currently listed on the wiki.
pub mod skip_segments;
pub mod vote_on_sponsor_time;
pub mod viewed_video_sponsor_time;
pub mod user_info;
pub mod user_stats;
pub mod get_views_for_user;
pub mod get_saved_time_for_user;
pub mod set_username;
pub mod get_username;
pub mod segment_info;
pub mod user_id;
pub mod lock_categories;
pub mod lock_reason;
pub mod search_segments;
pub mod status;
pub mod get_top_users;
pub mod get_total_stats;
pub mod get_days_saved_formatted;
// vip
pub mod is_user_vip;
pub mod shadow_ban_user;
pub mod warn_user;
pub mod clear_cache;
pub mod purge_all_segments;
pub mod segment_shift;
pub mod add_user_as_temp_vip;
pub mod feature;
// admin
pub mod add_user_as_vip;
// legacy
pub mod get_video_sponsor_times;
pub mod post_video_sponsor_times;