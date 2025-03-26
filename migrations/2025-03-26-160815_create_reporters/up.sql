CREATE TABLE reporters (
    reporterid INTEGER PRIMARY KEY REFERENCES users(userid),
    phonenum VARCHAR(20),
    occupation VARCHAR(25),
    dateofbirth DATE,
    officialaddress TEXT,
    faxnum VARCHAR(20),
    relationship VARCHAR(50)
);