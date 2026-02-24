-- this sql code creates the db, just run it in postgres and it should be ready to go

DROP DATABASE IF exists rchats;

SELECT 'CREATE DATABASE rchats'
WHERE NOT EXISTS (
    SELECT FROM pg_database WHERE datname = 'rchats'
)\gexec

CREATE TABLE "user" (
    uid SERIAL PRIMARY KEY,
    user_name VARCHAR(100) UNIQUE NOT NULL,
    hashed_password TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);

/*
CREATE TABLE "type" (
    type VARCHAR(100) PRIMARY KEY,
    file_location TEXT NOT NULL
);
*/

CREATE TABLE chat (
    cid SERIAL PRIMARY KEY,
    chat_name VARCHAR(100),
    user_ids INTEGER[] NOT NULL CHECK (array_length(user_ids, 1) = 2), 
    created_at TIMESTAMP DEFAULT NOW()
);

/* this might be added later but im not planing on group chats rn
CREATE TABLE user_chats (
    uid INT NOT NULL REFERENCES "user"(uid) ON DELETE CASCADE,
    cid INT NOT NULL REFERENCES chat(cid) ON DELETE CASCADE,
    PRIMARY KEY (uid, cid)
);
*/

CREATE TABLE message (
    mid SERIAL PRIMARY KEY,
    cid INT NOT NULL REFERENCES chat(cid) ON DELETE CASCADE,
    uid INT NOT NULL REFERENCES "user"(uid) ON DELETE CASCADE,
    url VARCHAR NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);
