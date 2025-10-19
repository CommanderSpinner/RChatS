
CREATE TABLE user(
    nid SERIAL PRIMARY KEY,
    user_name VARCHAR(100) UNIQUE NOT NULL,
    hashed_password TEXT NOT NULL
)
