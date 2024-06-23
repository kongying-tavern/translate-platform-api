// @generated automatically by Diesel CLI.

diesel::table! {
    sys_user (id) {
        version -> Int4,
        create_by -> Nullable<Varchar>,
        create_time -> Nullable<Timestamp>,
        update_by -> Nullable<Varchar>,
        update_time -> Nullable<Timestamp>,
        del_flag -> Bool,
        id -> Varchar,
        #[max_length = 32]
        username -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        role -> Int4,
        #[max_length = 32]
        timezone -> Varchar,
        #[max_length = 32]
        locale -> Varchar,
    }
}
