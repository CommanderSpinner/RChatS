CREATE TABLE "user" (
    uid SERIAL PRIMARY KEY,
    user_name VARCHAR(100) UNIQUE NOT NULL,
    hashed_password TEXT NOT NULL
);

CREATE TABLE "type" (
    type VARCHAR(100) PRIMARY KEY,
    file_location TEXT NOT NULL
);

CREATE TABLE chat (
    cid SERIAL PRIMARY KEY,
    chat_name VARCHAR(100),
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE user_chats (
    uid INT NOT NULL REFERENCES "user"(uid) ON DELETE CASCADE,
    cid INT NOT NULL REFERENCES chat(cid) ON DELETE CASCADE,
    joined_at TIMESTAMP DEFAULT NOW(),
    PRIMARY KEY (uid, cid)
);

CREATE TABLE message (
    mid SERIAL PRIMARY KEY,
    cid INT NOT NULL REFERENCES chat(cid) ON DELETE CASCADE,
    uid INT NOT NULL REFERENCES "user"(uid) ON DELETE CASCADE,
    type VARCHAR(100) REFERENCES "type"(type) ON DELETE SET NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);

INSERT INTO "type" (type, file_location) VALUES
('image', 'files/image'),
('audio', 'files/audio'),
('video', 'files/video');