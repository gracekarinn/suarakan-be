CREATE TABLE victims (
    victimid SERIAL PRIMARY KEY,
    fullname VARCHAR(100) NOT NULL,
    nik VARCHAR(20),
    email VARCHAR(50),
    domicileaddress TEXT,
    phonenum VARCHAR(20),
    occupation VARCHAR(50),
    sex VARCHAR(2),
    dateofbirth DATE,
    placeofbirth VARCHAR(50),
    officialaddress TEXT,
    educationlevel VARCHAR(50),
    faxnum VARCHAR(50),
    marriagestatus VARCHAR(50),
    marriageage INTEGER,
    isuploaded BOOLEAN DEFAULT FALSE,
    disability VARCHAR(50)
);