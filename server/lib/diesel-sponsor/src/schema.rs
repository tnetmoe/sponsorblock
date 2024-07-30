// SPDX-License-Identifier: AGPL-3.0-only

diesel::table! {
    archived_sponsor_times (uuid) {
        #[sql_name = "videoID"]
        video_id -> Text,
        #[sql_name = "startTime"]
        start_time -> Float4,
        #[sql_name = "endTime"]
        end_time -> Float4,
        votes -> Int4,
        locked -> Int4,
        #[sql_name = "incorrectVotes"]
        incorrect_votes -> Int4,
        #[sql_name = "UUID"]
        uuid -> Text,
        #[sql_name = "userID"]
        user_id -> Text,
        #[sql_name = "timeSubmitted"]
        time_submitted -> Int4,
        views -> Int4,
        category -> Text,
        #[sql_name = "actionType"]
        action_type -> Text,
        service -> Text,
        #[sql_name = "videoDuration"]
        video_duration -> Int4,
        hidden -> Int4,
        reputation -> Int4,
        #[sql_name = "shadowHidden"]
        shadow_hidden -> Int4,
        #[sql_name = "hashedVideoID"]
        hashed_video_id -> Text,
        #[sql_name = "userAgent"]
        user_agent -> Text,
        description -> Text,
    }
}

diesel::table! {
    category_votes (id) {
        #[sql_name = "UUID"]
        uuid -> Text,
        category -> Text,
        votes -> Int4,
        id -> Int4,
    }
}

diesel::table! {
    config (key) {
        key -> Text,
        value -> Text,
    }
}

diesel::table! {
    lock_categories (id) {
        #[sql_name = "videoID"]
        video_id -> Text,
        #[sql_name = "userID"]
        user_id -> Text,
        #[sql_name = "actionType"]
        action_type -> Text,
        category -> Text,
        #[sql_name = "hashedVideoID"]
        hashed_video_id -> Text,
        reason -> Text,
        service -> Text,
        id -> Int4,
    }
}

diesel::table! {
    ratings (id) {
        #[sql_name = "videoID"]
        video_id -> Text,
        service -> Text,
        #[sql_name = "type"]
        type_ -> Int4,
        count -> Int4,
        #[sql_name = "hashedVideoID"]
        hashed_video_id -> Text,
        id -> Int4,
    }
}

diesel::table! {
    shadow_banned_ips (hashed_id) {
        #[sql_name = "hashedIP"]
        hashed_id -> Text,
    }
}

diesel::table! {
    shadow_banned_users (user_id) {
        #[sql_name = "userID"]
        user_id -> Text,
    }
}

diesel::table! {
    sponsor_times (uuid) {
        #[sql_name = "videoID"]
        video_id -> Text,
        #[sql_name = "startTime"]
        start_time -> Float4,
        #[sql_name = "endTime"]
        end_time -> Float4,
        votes -> Int4,
        locked -> Int4,
        #[sql_name = "incorrectVotes"]
        incorrect_votes -> Int4,
        #[sql_name = "UUID"]
        uuid -> Text,
        #[sql_name = "userID"]
        user_id -> Text,
        #[sql_name = "timeSubmitted"]
        time_submitted -> Int4,
        views -> Int4,
        category -> Text,
        #[sql_name = "actionType"]
        action_type -> Text,
        service -> Text,
        #[sql_name = "videoDuration"]
        video_duration -> Int4,
        hidden -> Int4,
        reputation -> Int4,
        #[sql_name = "shadowHidden"]
        shadow_hidden -> Int4,
        #[sql_name = "hashedVideoID"]
        hashed_video_id -> Text,
        #[sql_name = "userAgent"]
        user_agent -> Text,
        description -> Text,
    }
}

diesel::table! {
    thumbnail_timestamps (uuid) {
        #[sql_name = "UUID"]
        uuid -> Text,
        timestamp -> Nullable<Int4>,
    }
}

diesel::table! {
    thumbnail_votes (uuid) {
        #[sql_name = "UUID"]
        uuid -> Text,
        votes -> Nullable<Int4>,
        locked -> Nullable<Int4>,
        #[sql_name = "shadowHidden"]
        shadow_hidden -> Nullable<Int4>,
        downvotes -> Nullable<Int4>,
        removed -> Nullable<Int4>,
    }
}

diesel::table! {
    thumbnails (uuid) {
        original -> Nullable<Int4>,
        #[sql_name = "userID"]
        user_id -> Text,
        service -> Text,
        #[sql_name = "hashedVideoID"]
        hashed_video_id -> Text,
        #[sql_name = "timeSubmitted"]
        time_submitted -> Int4,
        #[sql_name = "UUID"]
        uuid -> Text,
    }
}

diesel::table! {
    title_votes (uuid) {
        #[sql_name = "UUID"]
        uuid -> Text,
        votes -> Int4,
        locked -> Int4,
        #[sql_name = "shadowHidden"]
        shadow_hidden -> Int4,
        verification -> Nullable<Int4>,
        downvotes -> Int4,
        removed -> Int4,
    }
}

diesel::table! {
    titles (uuid) {
        #[sql_name = "videoID"]
        video_id -> Text,
        title -> Text,
        original -> Nullable<Int4>,
        #[sql_name = "userID"]
        user_id -> Text,
        service -> Text,
        #[sql_name = "hashedVideoID"]
        hashed_video_id -> Text,
        #[sql_name = "timeSubmitted"]
        time_submitted -> Int4,
        #[sql_name = "UUID"]
        uuid -> Text,
    }
}

diesel::table! {
    unlisted_videos (id) {
        #[sql_name = "videoID"]
        video_id -> Text,
        year -> Int4,
        views -> Int4,
        #[sql_name = "channelID"]
        channel_id -> Text,
        #[sql_name = "timeSubmitted"]
        time_submitted -> Int4,
        service -> Text,
        id -> Int4,
    }
}

diesel::table! {
    user_features (user_id, feature) {
        #[sql_name = "userID"]
        user_id -> Text,
        feature -> Int4,
        #[sql_name = "issuerUserID"]
        issuer_user_id -> Text,
        #[sql_name = "timeSubmitted"]
        time_submitted -> Int4,
    }
}

diesel::table! {
    user_names (user_id) {
        #[sql_name = "userID"]
        user_id -> Text,
        #[sql_name = "userName"]
        user_name -> Text,
        locked -> Int4,
    }
}

diesel::table! {
    video_info (video_id) {
        #[sql_name = "videoID"]
        video_id -> Text,
        #[sql_name = "channelID"]
        channel_id -> Text,
        title -> Text,
        published -> Float4,
    }
}

diesel::table! {
    vip_users (user_id) {
        #[sql_name = "userID"]
        user_id -> Text,
    }
}

diesel::table! {
    warnings (user_id, issue_time) {
        #[sql_name = "userID"]
        user_id -> Text,
        #[sql_name = "issueTime"]
        issue_time -> Int4,
        #[sql_name = "issuerUserID"]
        issuer_user_id -> Text,
        enabled -> Int4,
        reason -> Text,
        #[sql_name = "type"]
        type_ -> Nullable<Int4>,
    }
}

diesel::joinable!(thumbnail_timestamps -> thumbnails (uuid));
diesel::joinable!(thumbnail_votes -> thumbnails (uuid));
diesel::joinable!(title_votes -> titles (uuid));

diesel::allow_tables_to_appear_in_same_query!(
    archived_sponsor_times,
    category_votes,
    config,
    lock_categories,
    ratings,
    shadow_banned_ips,
    shadow_banned_users,
    sponsor_times,
    thumbnail_timestamps,
    thumbnail_votes,
    thumbnails,
    title_votes,
    titles,
    unlisted_videos,
    user_features,
    user_names,
    video_info,
    vip_users,
    warnings,
);
