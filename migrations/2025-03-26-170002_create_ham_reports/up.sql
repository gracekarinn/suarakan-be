CREATE TABLE ham_reports (
    reportid INTEGER PRIMARY KEY REFERENCES reports(reportid),
    updateid INTEGER REFERENCES updates(updateid)
);