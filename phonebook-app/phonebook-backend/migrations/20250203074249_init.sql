-- Add migration script here
CREATE TABLE person (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL,
    number VARCHAR(20) NOT NULL
);
