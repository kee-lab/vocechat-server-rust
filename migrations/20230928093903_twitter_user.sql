-- Add migration script here
CREATE TABLE twitter_user
(
    uid                 integer primary key not null,
    twitter_id                  integer not null,
    username            text collate nocase not null,
    profile_image_url   text
);

create index twitter_user_key_uid on twitter_user (uid);

create unique index twitter_user_key_twitter_id on twitter_user (twitter_id);

