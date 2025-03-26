CREATE TABLE updates (
    updateid SERIAL PRIMARY KEY,
    dataid INTEGER NOT NULL,
    createdat TIMESTAMP NOT NULL DEFAULT NOW(),
    updatedat TIMESTAMP,
    remarks TEXT,
    proof VARCHAR(50),
    status VARCHAR(50),
    adminid INTEGER REFERENCES admins(adminid)
);