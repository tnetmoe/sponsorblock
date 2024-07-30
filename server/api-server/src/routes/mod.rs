// SPDX-License-Identifier: AGPL-3.0-only
mod add_user_as_temp_vip;
mod add_user_as_vip;
mod branding;
mod clear_cache;
mod feature;
mod get_days_saved_formatted;
mod get_saved_time_for_user;
mod get_top_users;
mod get_total_stats;
mod get_username;
mod get_views_for_user;
mod is_user_vip;
mod lock_categories;
mod lock_reason;
mod purge_all_segments;
mod search_segment;
mod segment_info;
mod segment_shift;
mod set_username;
mod shadow_ban_user;
mod skip_segments;
mod status;
mod user_id;
mod user_info;
mod user_stats;
mod v1;
mod viewed_video_sponsor_time;
mod vote_on_sponsor_time;
mod warn_user;
use axum::{
    routing::{get, post},
    Router,
};

pub fn routes() -> Router {
    Router::new()
        .nest("/v1", v1::routes())
        .nest("/skipSegments", skip_segments::routes())
        .nest("/branding", branding::routes())
        .nest("/lockCategories", lock_categories::routes())
        .nest("/status", status::routes())
        .route("/voteOnSponsorTime", post(vote_on_sponsor_time::post_vote))
        .route(
            "/viewedVideoSponsorTime",
            post(viewed_video_sponsor_time::post_view),
        )
        .route("/userInfo", get(user_info::get_user_info))
        .route("/userStats", get(user_stats::get_user_stats))
        .route("/getViewsForUser", get(get_views_for_user::get_user_views))
        .route(
            "/getSavedTimeForUser",
            get(get_saved_time_for_user::get_user_saved_time),
        )
        .route("/setUsername", post(set_username::post_set_username))
        .route("/getUsername", get(get_username::get_username))
        .route("/segmentInfo", get(segment_info::get_segment_info))
        .route("/userID", get(user_id::get_user_name_search))
        .route("/lockReason", get(lock_reason::get_lock_reason))
        .route("/searchSegments", get(search_segment::get_search_segments))
        .route("/getTopUsers", get(get_top_users::get_top_users))
        .route("/getTotalStats", get(get_total_stats::get_total_stats))
        .route(
            "/getDaysSavedFormatted",
            get(get_days_saved_formatted::get_time_saved_stats),
        )
        .route("/isUserVIP", get(is_user_vip::get_is_user_vip))
        .route(
            "/shadowBanUser",
            post(shadow_ban_user::post_shadow_ban_user),
        )
        .route("/warnUser", post(warn_user::post_warn_user))
        .route("/clearCache", post(clear_cache::post_clear_cache))
        .route(
            "/purgeAllSegments",
            post(purge_all_segments::post_purge_all_segments),
        )
        .route("/segmentShift", post(segment_shift::post_segment_shift))
        .route(
            "/addUserAsTempVIP",
            post(add_user_as_temp_vip::post_add_user_as_temp_vip),
        )
        .route("/feature", post(feature::post_feature))
        .route("/addUserAsVIP", post(add_user_as_vip::post_add_user_as_vip))
}
