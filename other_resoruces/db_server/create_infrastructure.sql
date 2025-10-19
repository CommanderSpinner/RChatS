
CREATE TABLE "user"(
    uid SERIAL PRIMARY KEY,
    user_name VARCHAR(100) UNIQUE NOT NULL,
    hashed_password TEXT NOT NULL
);

CREATE TABLE "type"(
    type VARCHAR(100) PRIMARY KEY UNIQUE NOT NULL,
    file_location TEXT NOT NULL,
);

CREATE TABLE chat(
    cid SERIAL PRIMARY KEY,
    
);

CREATE TABLE user_chats(
    
);

CREATE TABLE message(
    mid SERIAL PRIMARY KEY,
    created_at TIMESTAMP DEFAULT NOW(),
    uid INT NOT NULL REFERENCES "user"(uid) ON DELETE CASCADE,
    type VARCHAR(100) NOT NULL REFERENCES "type"(type) ON DELETE SET NULL
);
