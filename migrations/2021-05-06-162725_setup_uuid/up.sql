-- Your SQL goes here

DROP TABLE IF EXISTS "transactions";


CREATE TABLE cards(
  id uuid DEFAULT uuid_generate_v4 (),
  owner_names VARCHAR(2020) NOT NULL,
  created_at TIMESTAMP DEFAULT Now() NOT NULL,
  PRIMARY KEY(id)
);


CREATE TABLE transactions(
    id serial PRIMARY KEY,
    initial_balance FLOAT NOT NULL,
    transport_fare FLOAT NOT NULL,
    card_id uuid NOT NULL,
    CONSTRAINT fk_card
        FOREIGN KEY(card_id)
            REFERENCES cards(id)
            ON DELETE CASCADE
);
