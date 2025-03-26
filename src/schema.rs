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
        adminid -> Nullable<Int4>,
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
diesel::joinable!(ham_reports -> reports (reportid));
diesel::joinable!(ham_reports -> updates (updateid));
diesel::joinable!(perempuan_reports -> reports (reportid));
diesel::joinable!(perempuan_reports -> updates (updateid));
diesel::joinable!(publications -> admins (adminid));
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
