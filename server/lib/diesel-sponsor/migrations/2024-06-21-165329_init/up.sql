-- Initial migration

-- vipUsers
CREATE TABLE IF NOT EXISTS "vipUsers"(
    "userID" TEXT NOT NULL PRIMARY KEY
);

-- sponsorTimes
CREATE TABLE IF NOT EXISTS "sponsorTimes"(
    "videoID" TEXT NOT NULL,
    "startTime" REAL NOT NULL,
    "endTime" REAL NOT NULL,
    "votes" INTEGER NOT NULL,
    "locked" INTEGER NOT NULL default 0,
    "incorrectVotes" INTEGER NOT NULL default 1,
    "UUID" TEXT NOT NULL UNIQUE PRIMARY KEY,
    "userID" TEXT NOT NULL,
    "timeSubmitted" INTEGER NOT NULL,
    "views" INTEGER NOT NULL,
    "category" TEXT NOT NULL default 'sponsor',
    "actionType" TEXT NOT NULL default 'skip',
    "service" TEXT NOT NULL default 'YouTube',
    "videoDuration" INTEGER NOT NULL default 0,
    "hidden" INTEGER NOT NULL default 0,
    "reputation" INTEGER NOT NULL default 0,
    "shadowHidden" INTEGER NOT NULL,
    "hashedVideoID" TEXT NOT NULL,
    "userAgent" TEXT NOT NULL,
    "description" TEXT NOT NULL
);

-- userNames
CREATE TABLE IF NOT EXISTS "userNames"(
    "userID" TEXT NOT NULL PRIMARY KEY,
    "userName" TEXT NOT NULL,
    "locked" INTEGER NOT NULL default 0
);

-- categoryVotes
CREATE TABLE IF NOT EXISTS "categoryVotes"(
    "UUID" TEXT NOT NULL,
    "category" TEXT NOT NULL,
    "votes" INTEGER NOT NULL default 0,
    "id" SERIAL PRIMARY KEY
);

-- lockCategories
CREATE TABLE IF NOT EXISTS "lockCategories"(
    "videoID" TEXT NOT NULL,
    "userID" TEXT NOT NULL,
    "actionType" TEXT NOT NULL default 0,
    "category" TEXT NOT NULL,
    "hashedVideoID" TEXT NOT NULL,
    "reason" TEXT NOT NULL,
    "service" TEXT NOT NULL default 'YouTube',
    "id" SERIAL PRIMARY KEY
);

-- warnings
CREATE TABLE IF NOT EXISTS "warnings"(
    "userID" TEXT NOT NULL,
    "issueTime" INTEGER NOT NULL,
    "issuerUserID" TEXT NOT NULL,
    "enabled" INTEGER NOT NULL,
    "reason" TEXT NOT NULL,
    "type" INTEGER default 0,
    PRIMARY KEY ("userID", "issueTime")
);

-- shadowBannedUsers
CREATE TABLE IF NOT EXISTS "shadowBannedUsers"(
    "userID" TEXT NOT NULL PRIMARY KEY
);

-- videoInfo
CREATE TABLE IF NOT EXISTS "videoInfo"(
    "videoID" TEXT NOT NULL PRIMARY KEY,
    "channelID" TEXT NOT NULL,
    "title" TEXT NOT NULL,
    "published" REAL NOT NULL
);

-- unlistedVideos
CREATE TABLE IF NOT EXISTS "unlistedVideos"(
    "videoID" TEXT NOT NULL,
    "year" INTEGER NOT NULL,
    "views" INTEGER NOT NULL,
    "channelID" TEXT NOT NULL,
    "timeSubmitted" INTEGER NOT NULL,
    "service" TEXT NOT NULL default 'YouTube',
    "id" SERIAL PRIMARY KEY
);

-- config
CREATE TABLE IF NOT EXISTS "config"(
    "key" TEXT NOT NULL PRIMARY KEY,
    "value" TEXT NOT NULL
);

-- archivedSponsorTimes
CREATE TABLE IF NOT EXISTS "archivedSponsorTimes"(
    "videoID" TEXT NOT NULL,
    "startTime" REAL NOT NULL,
    "endTime" REAL NOT NULL,
    "votes" INTEGER NOT NULL,
    "locked" INTEGER NOT NULL default 0,
    "incorrectVotes" INTEGER NOT NULL default 1,
    "UUID" TEXT NOT NULL UNIQUE PRIMARY KEY,
    "userID" TEXT NOT NULL,
    "timeSubmitted" INTEGER NOT NULL,
    "views" INTEGER NOT NULL,
    "category" TEXT NOT NULL default 'sponsor',
    "actionType" TEXT NOT NULL default 'skip',
    "service" TEXT NOT NULL default 'YouTube',
    "videoDuration" INTEGER NOT NULL default 0,
    "hidden" INTEGER NOT NULL default 0,
    "reputation" INTEGER NOT NULL default 0,
    "shadowHidden" INTEGER NOT NULL,
    "hashedVideoID" TEXT NOT NULL,
    "userAgent" TEXT NOT NULL,
    "description" TEXT NOT NULL
);

-- ratings
CREATE TABLE IF NOT EXISTS "ratings"(
    "videoID" TEXT NOT NULL,
    "service" TEXT NOT NULL default 'YouTube',
    "type" INTEGER NOT NULL,
    "count" INTEGER NOT NULL,
    "hashedVideoID" TEXT NOT NULL,
    "id" SERIAL PRIMARY KEY
);

-- userFeatures
CREATE TABLE IF NOT EXISTS "userFeatures"(
    "userID" TEXT NOT NULL,
    "feature" INTEGER NOT NULL,
    "issuerUserID" TEXT NOT NULL,
    "timeSubmitted" INTEGER NOT NULL,
    PRIMARY KEY ("userID", "feature")
);

-- shadowBannedIPs
CREATE TABLE IF NOT EXISTS "shadowBannedIPs"(
    "hashedIP" TEXT NOT NULL PRIMARY KEY
);

-- titles
CREATE TABLE IF NOT EXISTS "titles"(
    "videoID" TEXT NOT NULL,
    "title" TEXT NOT NULL,
    "original" INTEGER default 0,
    "userID" TEXT NOT NULL,
    "service" TEXT NOT NULL,
    "hashedVideoID" TEXT NOT NULL,
    "timeSubmitted" INTEGER NOT NULL,
    "UUID" TEXT NOT NULL PRIMARY KEY
);

-- titleVotes
CREATE TABLE IF NOT EXISTS "titleVotes"(
    "UUID" TEXT NOT NULL PRIMARY KEY,
    "votes" INTEGER NOT NULL default 0,
    "locked" INTEGER NOT NULL default 0,
    "shadowHidden" INTEGER NOT NULL default 0,
    "verification" INTEGER default 0,
    "downvotes" INTEGER NOT NULL default 0,
    "removed" INTEGER NOT NULL default 0,
    FOREIGN KEY ("UUID") REFERENCES "titles"("UUID")
);

-- thumbnails
CREATE TABLE IF NOT EXISTS "thumbnails"(
    "original" INTEGER default 0,
    "userID" TEXT NOT NULL,
    "service" TEXT NOT NULL,
    "hashedVideoID" TEXT NOT NULL,
    "timeSubmitted" INTEGER NOT NULL,
    "UUID" TEXT NOT NULL PRIMARY KEY
);

-- thumbnailTimestamps
CREATE TABLE IF NOT EXISTS "thumbnailTimestamps"(
    "UUID" TEXT PRIMARY KEY,
    "timestamp" INTEGER,
    FOREIGN KEY ("UUID") REFERENCES "thumbnails"("UUID")
);

-- thumbnailVotes
CREATE TABLE IF NOT EXISTS "thumbnailVotes"(
    "UUID" TEXT PRIMARY KEY,
    "votes" INTEGER,
    "locked" INTEGER,
    "shadowHidden" INTEGER,
    "downvotes" INTEGER,
    "removed" INTEGER,
    FOREIGN KEY ("UUID") REFERENCES "thumbnails"("UUID")
);

-- Set version
INSERT INTO "config" ("key", "value") VALUES ('version', 1);