CREATE TABLE accused (
    accusedid SERIAL PRIMARY KEY,
    fullname VARCHAR(50) NOT NULL,
    email VARCHAR(50),
    domicileaddress TEXT,
    phonenum VARCHAR(50),
    occupation VARCHAR(50),
    sex VARCHAR(50),
    dateofbirth DATE,
    placeofbirth VARCHAR(50),
    educationlevel VARCHAR(50),
    relationship VARCHAR(50)
);