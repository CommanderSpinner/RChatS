
CREATE TABLE user(
    nid SERIAL PRIMARY KEY,
    user_name VARCHAR(100) UNIQUE NOT NULL,
    hashed_password TEXT NOT NULL
);

CREATE TABLE type(
    type VARCHAR(100) PRIMARY KEY UNIQUE NOT NULL,
    file_location TEXT NOT NULL,
)

CREATE TABLE message(
    mid SERIAL PRIMARY KEY,
    created_at TIMESTAMP DEFAULT NOW(),
    nid INT REFERENCES user(nid),
    type VARCHAR(100) REFERENCES type(type),
)