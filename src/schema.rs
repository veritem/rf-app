table! {
    cards (id) {
        id -> Uuid,
        owner_names -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    transactions (id) {
        id -> Int4,
        initial_balance -> Float8,
        transport_fare -> Float8,
        card_id -> Uuid,
    }
}

joinable!(transactions -> cards (card_id));

allow_tables_to_appear_in_same_query!(
    cards,
    transactions,
);
