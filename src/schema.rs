// @generated automatically by Diesel CLI.

diesel::table! {
    accused (accusedid) {
        accusedid -> Int4,
        #[max_length = 50]
        fullname -> Varchar,
        #[max_length = 50]
        email -> Nullable<Varchar>,
        domicileaddress -> Nullable<Text>,
        #[max_length = 50]
        phonenum -> Nullable<Varchar>,
        #[max_length = 50]
        occupation -> Nullable<Varchar>,
        #[max_length = 50]
        sex -> Nullable<Varchar>,
        dateofbirth -> Nullable<Date>,
        #[max_length = 50]
        placeofbirth -> Nullable<Varchar>,
        #[max_length = 50]
        educationlevel -> Nullable<Varchar>,
        #[max_length = 50]
        relationship -> Nullable<Varchar>,
    }
}

diesel::table! {
    admins (adminid) {
        adminid -> Int4,
    }
}

diesel::table! {
    auth_group (id) {
        id -> Int4,
        #[max_length = 150]
        name -> Varchar,
    }
}

diesel::table! {
    auth_group_permissions (id) {
        id -> Int8,
        group_id -> Int4,
        permission_id -> Int4,
    }
}

diesel::table! {
    auth_permission (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        content_type_id -> Int4,
        #[max_length = 100]
        codename -> Varchar,
    }
}

diesel::table! {
    authentication_user (id) {
        id -> Int8,
        #[max_length = 128]
        password -> Varchar,
        last_login -> Nullable<Timestamptz>,
        is_superuser -> Bool,
        #[max_length = 150]
        first_name -> Varchar,
        #[max_length = 150]
        last_name -> Varchar,
        is_staff -> Bool,
        is_active -> Bool,
        date_joined -> Timestamptz,
        #[max_length = 254]
        email -> Varchar,
        #[max_length = 10]
        user_type -> Varchar,
        is_email_verified -> Bool,
        #[max_length = 255]
        full_name -> Varchar,
        #[max_length = 15]
        phone_number -> Varchar,
    }
}

diesel::table! {
    authentication_user_groups (id) {
        id -> Int8,
        user_id -> Int8,
        group_id -> Int4,
    }
}

diesel::table! {
    authentication_user_user_permissions (id) {
        id -> Int8,
        user_id -> Int8,
        permission_id -> Int4,
    }
}

diesel::table! {
    authentication_userprofile (id) {
        id -> Int8,
        reporter_id -> Uuid,
        #[max_length = 255]
        occupation -> Varchar,
        date_of_birth -> Nullable<Date>,
        official_address -> Text,
        #[max_length = 20]
        fax_number -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        user_id -> Int8,
    }
}

diesel::table! {
    django_admin_log (id) {
        id -> Int4,
        action_time -> Timestamptz,
        object_id -> Nullable<Text>,
        #[max_length = 200]
        object_repr -> Varchar,
        action_flag -> Int2,
        change_message -> Text,
        content_type_id -> Nullable<Int4>,
        user_id -> Int8,
    }
}

diesel::table! {
    django_content_type (id) {
        id -> Int4,
        #[max_length = 100]
        app_label -> Varchar,
        #[max_length = 100]
        model -> Varchar,
    }
}

diesel::table! {
    django_migrations (id) {
        id -> Int8,
        #[max_length = 255]
        app -> Varchar,
        #[max_length = 255]
        name -> Varchar,
        applied -> Timestamptz,
    }
}

diesel::table! {
    django_session (session_key) {
        #[max_length = 40]
        session_key -> Varchar,
        session_data -> Text,
        expire_date -> Timestamptz,
    }
}

diesel::table! {
    ham_reports (reportid) {
        reportid -> Int4,
        updateid -> Nullable<Int4>,
    }
}

diesel::table! {
    incidents (incidentid) {
        incidentid -> Int4,
        #[max_length = 40]
        location -> Varchar,
        time -> Timestamp,
        description -> Nullable<Text>,
        victimneeds -> Nullable<Text>,
        pasteffort -> Nullable<Text>,
    }
}

diesel::table! {
    perempuan_reports (reportid) {
        reportid -> Int4,
        updateid -> Nullable<Int4>,
    }
}

diesel::table! {
    proofs (proofid) {
        proofid -> Int4,
        #[max_length = 50]
        link -> Varchar,
    }
}

diesel::table! {
    publications (publicationid) {
        publicationid -> Int4,
        #[max_length = 50]
        title -> Varchar,
        createdat -> Timestamp,
        updatedat -> Nullable<Timestamp>,
        description -> Nullable<Text>,
        #[max_length = 50]
        filelink -> Nullable<Varchar>,
        adminid -> Nullable<Int8>,
    }
}

diesel::table! {
    reporters (reporterid) {
        reporterid -> Int4,
        #[max_length = 20]
        phonenum -> Nullable<Varchar>,
        #[max_length = 25]
        occupation -> Nullable<Varchar>,
        dateofbirth -> Nullable<Date>,
        officialaddress -> Nullable<Text>,
        #[max_length = 20]
        faxnum -> Nullable<Varchar>,
        #[max_length = 50]
        relationship -> Nullable<Varchar>,
    }
}

diesel::table! {
    reports (reportid) {
        reportid -> Int4,
        reporterid -> Int4,
        createdat -> Timestamp,
        updatedat -> Nullable<Timestamp>,
        proofid -> Nullable<Int4>,
        incidentid -> Nullable<Int4>,
        victimid -> Nullable<Int4>,
        accusedid -> Nullable<Int4>,
        updateid -> Int4,
    }
}

diesel::table! {
    ui_reports (reportid) {
        reportid -> Int4,
        updateid -> Nullable<Int4>,
    }
}

diesel::table! {
    updates (updateid) {
        updateid -> Int4,
        dataid -> Int4,
        createdat -> Timestamp,
        updatedat -> Nullable<Timestamp>,
        remarks -> Nullable<Text>,
        #[max_length = 50]
        proof -> Nullable<Varchar>,
        #[max_length = 50]
        status -> Nullable<Varchar>,
        adminid -> Nullable<Int4>,
    }
}

diesel::table! {
    users (userid) {
        userid -> Int4,
        #[max_length = 50]
        fullname -> Varchar,
        #[max_length = 50]
        email -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        #[max_length = 10]
        role -> Varchar,
    }
}

diesel::table! {
    victims (victimid) {
        victimid -> Int4,
        #[max_length = 100]
        fullname -> Varchar,
        #[max_length = 20]
        nik -> Nullable<Varchar>,
        #[max_length = 50]
        email -> Nullable<Varchar>,
        domicileaddress -> Nullable<Text>,
        #[max_length = 20]
        phonenum -> Nullable<Varchar>,
        #[max_length = 50]
        occupation -> Nullable<Varchar>,
        #[max_length = 2]
        sex -> Nullable<Varchar>,
        dateofbirth -> Nullable<Date>,
        #[max_length = 50]
        placeofbirth -> Nullable<Varchar>,
        officialaddress -> Nullable<Text>,
        #[max_length = 50]
        educationlevel -> Nullable<Varchar>,
        #[max_length = 50]
        faxnum -> Nullable<Varchar>,
        #[max_length = 50]
        marriagestatus -> Nullable<Varchar>,
        marriageage -> Nullable<Int4>,
        isuploaded -> Nullable<Bool>,
        #[max_length = 50]
        disability -> Nullable<Varchar>,
    }
}

diesel::joinable!(admins -> users (adminid));
diesel::joinable!(auth_group_permissions -> auth_group (group_id));
diesel::joinable!(auth_group_permissions -> auth_permission (permission_id));
diesel::joinable!(auth_permission -> django_content_type (content_type_id));
diesel::joinable!(authentication_user_groups -> auth_group (group_id));
diesel::joinable!(authentication_user_groups -> authentication_user (user_id));
diesel::joinable!(authentication_user_user_permissions -> auth_permission (permission_id));
diesel::joinable!(authentication_user_user_permissions -> authentication_user (user_id));
diesel::joinable!(authentication_userprofile -> authentication_user (user_id));
diesel::joinable!(django_admin_log -> authentication_user (user_id));
diesel::joinable!(django_admin_log -> django_content_type (content_type_id));
diesel::joinable!(ham_reports -> reports (reportid));
diesel::joinable!(ham_reports -> updates (updateid));
diesel::joinable!(perempuan_reports -> reports (reportid));
diesel::joinable!(perempuan_reports -> updates (updateid));
diesel::joinable!(publications -> authentication_user (adminid));
diesel::joinable!(reporters -> users (reporterid));
diesel::joinable!(reports -> accused (accusedid));
diesel::joinable!(reports -> incidents (incidentid));
diesel::joinable!(reports -> proofs (proofid));
diesel::joinable!(reports -> reporters (reporterid));
diesel::joinable!(reports -> victims (victimid));
diesel::joinable!(ui_reports -> reports (reportid));
diesel::joinable!(ui_reports -> updates (updateid));
diesel::joinable!(updates -> admins (adminid));

diesel::allow_tables_to_appear_in_same_query!(
    accused,
    admins,
    auth_group,
    auth_group_permissions,
    auth_permission,
    authentication_user,
    authentication_user_groups,
    authentication_user_user_permissions,
    authentication_userprofile,
    django_admin_log,
    django_content_type,
    django_migrations,
    django_session,
    ham_reports,
    incidents,
    perempuan_reports,
    proofs,
    publications,
    reporters,
    reports,
    ui_reports,
    updates,
    users,
    victims,
);
