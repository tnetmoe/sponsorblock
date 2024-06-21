// @generated automatically by Diesel CLI.
#![allow(non_snake_case)]

diesel::table! {
    archivedSponsorTimes (UUID) {
        videoID -> Text,
        startTime -> Float4,
        endTime -> Float4,
        votes -> Int4,
        locked -> Int4,
        incorrectVotes -> Int4,
        UUID -> Text,
        userID -> Text,
        timeSubmitted -> Int4,
        views -> Int4,
        category -> Text,
        actionType -> Text,
        service -> Text,
        videoDuration -> Int4,
        hidden -> Int4,
        reputation -> Int4,
        shadowHidden -> Int4,
        hashedVideoID -> Text,
        userAgent -> Text,
        description -> Text,
    }
}

diesel::table! {
    categoryVotes (id) {
        UUID -> Text,
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
    lockCategories (id) {
        videoID -> Text,
        userID -> Text,
        actionType -> Text,
        category -> Text,
        hashedVideoID -> Text,
        reason -> Text,
        service -> Text,
        id -> Int4,
    }
}

diesel::table! {
    ratings (id) {
        videoID -> Text,
        service -> Text,
        #[sql_name = "type"]
        type_ -> Int4,
        count -> Int4,
        hashedVideoID -> Text,
        id -> Int4,
    }
}

diesel::table! {
    shadowBannedIPs (hashedIP) {
        hashedIP -> Text,
    }
}

diesel::table! {
    shadowBannedUsers (userID) {
        userID -> Text,
    }
}

diesel::table! {
    sponsorTimes (UUID) {
        videoID -> Text,
        startTime -> Float4,
        endTime -> Float4,
        votes -> Int4,
        locked -> Int4,
        incorrectVotes -> Int4,
        UUID -> Text,
        userID -> Text,
        timeSubmitted -> Int4,
        views -> Int4,
        category -> Text,
        actionType -> Text,
        service -> Text,
        videoDuration -> Int4,
        hidden -> Int4,
        reputation -> Int4,
        shadowHidden -> Int4,
        hashedVideoID -> Text,
        userAgent -> Text,
        description -> Text,
    }
}

diesel::table! {
    thumbnailTimestamps (UUID) {
        UUID -> Text,
        timestamp -> Nullable<Int4>,
    }
}

diesel::table! {
    thumbnailVotes (UUID) {
        UUID -> Text,
        votes -> Nullable<Int4>,
        locked -> Nullable<Int4>,
        shadowHidden -> Nullable<Int4>,
        downvotes -> Nullable<Int4>,
        removed -> Nullable<Int4>,
    }
}

diesel::table! {
    thumbnails (UUID) {
        original -> Nullable<Int4>,
        userID -> Text,
        service -> Text,
        hashedVideoID -> Text,
        timeSubmitted -> Int4,
        UUID -> Text,
    }
}

diesel::table! {
    titleVotes (UUID) {
        UUID -> Text,
        votes -> Int4,
        locked -> Int4,
        shadowHidden -> Int4,
        verification -> Nullable<Int4>,
        downvotes -> Int4,
        removed -> Int4,
    }
}

diesel::table! {
    titles (UUID) {
        videoID -> Text,
        title -> Text,
        original -> Nullable<Int4>,
        userID -> Text,
        service -> Text,
        hashedVideoID -> Text,
        timeSubmitted -> Int4,
        UUID -> Text,
    }
}

diesel::table! {
    unlistedVideos (id) {
        videoID -> Text,
        year -> Int4,
        views -> Int4,
        channelID -> Text,
        timeSubmitted -> Int4,
        service -> Text,
        id -> Int4,
    }
}

diesel::table! {
    userFeatures (userID, feature) {
        userID -> Text,
        feature -> Int4,
        issuerUserID -> Text,
        timeSubmitted -> Int4,
    }
}

diesel::table! {
    userNames (userID) {
        userID -> Text,
        userName -> Text,
        locked -> Int4,
    }
}

diesel::table! {
    videoInfo (videoID) {
        videoID -> Text,
        channelID -> Text,
        title -> Text,
        published -> Float4,
    }
}

diesel::table! {
    vipUsers (userID) {
        userID -> Text,
    }
}

diesel::table! {
    warnings (userID, issueTime) {
        userID -> Text,
        issueTime -> Int4,
        issuerUserID -> Text,
        enabled -> Int4,
        reason -> Text,
        #[sql_name = "type"]
        type_ -> Nullable<Int4>,
    }
}

diesel::joinable!(thumbnailTimestamps -> thumbnails (UUID));
diesel::joinable!(thumbnailVotes -> thumbnails (UUID));
diesel::joinable!(titleVotes -> titles (UUID));

diesel::allow_tables_to_appear_in_same_query!(
    archivedSponsorTimes,
    categoryVotes,
    config,
    lockCategories,
    ratings,
    shadowBannedIPs,
    shadowBannedUsers,
    sponsorTimes,
    thumbnailTimestamps,
    thumbnailVotes,
    thumbnails,
    titleVotes,
    titles,
    unlistedVideos,
    userFeatures,
    userNames,
    videoInfo,
    vipUsers,
    warnings,
);
