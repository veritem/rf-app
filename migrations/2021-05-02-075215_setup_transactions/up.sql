-- Your SQL goes here


CREATE TABLE transactions(
    id SERIAL PRIMARY KEY,
    initial_balance FLOAT NOT NULL,
    transport_fare FLOAT NOT NULL
)
