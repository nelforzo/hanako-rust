table! {
    categories (id) {
        id -> Unsigned<Bigint>,
        user_id -> Integer,
        name -> Varchar,
        description -> Nullable<Varchar>,
        active_package_id -> Nullable<Varchar>,
        show_in_auto_list -> Bool,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    list_items (id) {
        id -> Unsigned<Bigint>,
        list_id -> Integer,
        category_id -> Nullable<Integer>,
        name -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    lists (id) {
        id -> Unsigned<Bigint>,
        user_id -> Integer,
        name -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    logs (id) {
        id -> Unsigned<Bigint>,
        operation -> Varchar,
        user_id -> Integer,
        category_id -> Integer,
        package_id -> Nullable<Integer>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    packages (id) {
        id -> Unsigned<Bigint>,
        user_id -> Integer,
        category_id -> Integer,
        name -> Varchar,
        description -> Nullable<Varchar>,
        brand -> Nullable<Varchar>,
        comment -> Nullable<Varchar>,
        barcode -> Nullable<Varchar>,
        uuid -> Varchar,
        units_per_package -> Integer,
        units_left -> Nullable<Integer>,
        mililiters_per_package -> Nullable<Integer>,
        expiration_date -> Nullable<Timestamp>,
        opened_date -> Nullable<Timestamp>,
        consume_before_days -> Nullable<Integer>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    ratings (id) {
        id -> Unsigned<Bigint>,
        package_id -> Nullable<Integer>,
        rating -> Integer,
        comment -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    stock_alerts (id) {
        id -> Unsigned<Bigint>,
        user_id -> Integer,
        category_id -> Integer,
        minimum_stock_units -> Integer,
        comment -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    tags (id) {
        id -> Unsigned<Bigint>,
        user_id -> Integer,
        category_id -> Integer,
        body -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    users (id) {
        id -> Unsigned<Bigint>,
        first_name -> Varchar,
        last_name -> Varchar,
        first_name_kana -> Varchar,
        last_name_kana -> Varchar,
        gender -> Nullable<Integer>,
        birthday -> Nullable<Date>,
        mail_address -> Varchar,
        password_hash -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

allow_tables_to_appear_in_same_query!(
    categories,
    list_items,
    lists,
    logs,
    packages,
    ratings,
    stock_alerts,
    tags,
    users,
);
