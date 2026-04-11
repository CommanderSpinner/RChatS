-- this sql code creates the db, just run it in postgres and it should be ready to go

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



-- test users not intended for prod

-- 1. Create a test user
INSERT INTO "user" (user_name, hashed_password) 
VALUES ('test1', 'test123');

-- 2. Create a second user (needed because your chat table requires exactly 2 users)
INSERT INTO "user" (user_name, hashed_password) 
VALUES ('test2', 'test123');

-- 3. Create a chat between these two users
INSERT INTO chat (chat_name, user_ids) 
VALUES ('Test Chat', ARRAY[1, 2]);

-- 4. Create a message within that chat
INSERT INTO message (cid, uid, url, content) 
VALUES (1, 1, '-', 'test msg');