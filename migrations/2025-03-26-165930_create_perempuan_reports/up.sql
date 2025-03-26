CREATE TABLE perempuan_reports (
    reportid INTEGER PRIMARY KEY REFERENCES reports(reportid),
    updateid INTEGER REFERENCES updates(updateid)
);

