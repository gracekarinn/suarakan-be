CREATE TABLE reports (
    reportid SERIAL PRIMARY KEY,
    reporterid INTEGER NOT NULL REFERENCES reporters(reporterid),
    createdat TIMESTAMP NOT NULL DEFAULT NOW(),
    updatedat TIMESTAMP,
    proofid INTEGER REFERENCES proofs(proofid),
    incidentid INTEGER UNIQUE REFERENCES incidents(incidentid),
    victimid INTEGER UNIQUE REFERENCES victims(victimid),
    accusedid INTEGER UNIQUE REFERENCES accused(accusedid)
);