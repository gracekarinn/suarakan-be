CREATE TABLE admins (
    adminid INTEGER PRIMARY KEY REFERENCES users(userid)
);
