CREATE TABLE reports (
    reportid              SERIAL PRIMARY KEY,
    updateid              INTEGER NOT NULL REFERENCES updates(updateid),

    createdat             TIMESTAMP,
    updatedat             TIMESTAMP,

    -- REPORTER
    reporterfullname      TEXT,
    reporterphonenum      TEXT,
    reporteroccupation    TEXT,
    reporterdateofbirth   DATE,
    reporteraddress       TEXT,
    reporterrelationship  TEXT,

    -- INCIDENT
    incidentlocation      TEXT NOT NULL,
    incidenttime          TIMESTAMP NOT NULL,
    incidentdescription   TEXT,
    incidentvictimneeds   TEXT,
    incidentpasteffort    TEXT,
    incidentproof         TEXT,

    -- VICTIM
    victimfullname        TEXT NOT NULL,
    victimnik             TEXT,
    victimemail           TEXT,
    victimaddress         TEXT,
    victimphonenum        TEXT,
    victimoccupation      TEXT,
    victimsex             TEXT,
    victimdateofbirth     DATE,
    victimplaceofbirth    TEXT,
    victimeducationlevel  TEXT,
    victimmarriagestatus  TEXT,
    victimdisability      TEXT,

    -- ACCUSED
    accusedfullname       TEXT NOT NULL,
    accusedemail          TEXT,
    accusedaddress        TEXT,
    accusedphonenum       TEXT,
    accusedoccupation     TEXT,
    accusedsex            TEXT,
    accuseddateofbirth    DATE,
    accusedplaceofbirth   TEXT,
    accusededucationlevel TEXT,
    accusedrelationship   TEXT,

    -- AUTHORITY
    authority             TEXT NOT NULL
);