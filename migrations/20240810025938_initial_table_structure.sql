-- Add migration script here
 CREATE TABLE users (
    user_id SERIAL PRIMARY KEY,
    username VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    profile_picture TEXT,
    bio TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE posts (
    post_id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    content_text TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(user_id)
);

CREATE TABLE media (
    media_id SERIAL PRIMARY KEY,
    post_id INTEGER NOT NULL,
    media_type VARCHAR(50) NOT NULL,
    media_url TEXT NOT NULL,
    FOREIGN KEY (post_id) REFERENCES posts(post_id)
);

CREATE TABLE recipes (
    recipe_id SERIAL PRIMARY KEY,
    post_id INTEGER NOT NULL,
    title VARCHAR(255) NOT NULL,
    instructions TEXT NOT NULL,
    FOREIGN KEY (post_id) REFERENCES posts(post_id)
);

CREATE TABLE ingredients (
    ingredient_id SERIAL PRIMARY KEY,
    recipe_id INTEGER NOT NULL,
    name VARCHAR(255) NOT NULL,
    quantity VARCHAR(255) NOT NULL,
    FOREIGN KEY (recipe_id) REFERENCES recipes(recipe_id)
);

CREATE TABLE preferences (
    preference_id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    coffee_type VARCHAR(255),
    flavor_profile VARCHAR(255),
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(user_id)
);

CREATE TABLE authentication (
    auth_id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    session_token TEXT NOT NULL,
    expires_at TIMESTAMP NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(user_id)
);
