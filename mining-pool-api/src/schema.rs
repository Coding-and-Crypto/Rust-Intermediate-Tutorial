table! {
    miners (id) {
        id -> Uuid,
        address -> Uuid,
        nickname -> Text,
        hash_rate -> Int4,
        shares_mined -> Int4,
    }
}

table! {
    wallets (address) {
        address -> Uuid,
        club_name -> Text,
    }
}

joinable!(miners -> wallets (address));

allow_tables_to_appear_in_same_query!(
    miners,
    wallets,
);
