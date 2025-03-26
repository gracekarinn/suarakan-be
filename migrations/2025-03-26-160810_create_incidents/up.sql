CREATE TABLE incidents (
    incidentid SERIAL PRIMARY KEY,
    location VARCHAR(40) NOT NULL,
    time TIMESTAMP NOT NULL,
    description TEXT,
    victimneeds TEXT,
    pasteffort TEXT
);
