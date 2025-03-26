CREATE TABLE publications (
    publicationid SERIAL PRIMARY KEY,
    title VARCHAR(50) NOT NULL,
    createdat TIMESTAMP NOT NULL DEFAULT NOW(),
    updatedat TIMESTAMP,
    description TEXT,
    filelink VARCHAR(50),
    adminid INTEGER REFERENCES admins(adminid)
);