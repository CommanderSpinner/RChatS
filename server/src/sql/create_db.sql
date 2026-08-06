
CREATE TABLE IF NOT EXISTS "user" (
    uid BIGSERIAL PRIMARY KEY,
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

CREATE TABLE IF NOT EXISTS chat (
    cid BIGSERIAL PRIMARY KEY,
    chat_name VARCHAR(100),
    user_ids BIGINT[] NOT NULL UNIQUE CHECK (array_length(user_ids, 1) = 2), 
    created_at TIMESTAMP DEFAULT NOW()
);

/* this might be added later but im not planing on group chats rn
CREATE TABLE IF NOT EXISTS user_chats (
    uid INT NOT NULL REFERENCES "user"(uid) ON DELETE CASCADE,
    cid INT NOT NULL REFERENCES chat(cid) ON DELETE CASCADE,
    PRIMARY KEY (uid, cid)
);
*/

CREATE TABLE IF NOT EXISTS message (
    mid BIGSERIAL PRIMARY KEY,
    cid BIGINT NOT NULL REFERENCES chat(cid) ON DELETE CASCADE,
    uid BIGINT NOT NULL REFERENCES "user"(uid) ON DELETE CASCADE,
    url VARCHAR NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);