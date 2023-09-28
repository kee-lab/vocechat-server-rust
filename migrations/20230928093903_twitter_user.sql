-- Add migration script here
CREATE TABLE twitter_user
(
    uid                 integer primary key autoincrement not null,
    twitter_id                  integer not null,
    username            text collate nocase not null,
    profile_image_url   text
)