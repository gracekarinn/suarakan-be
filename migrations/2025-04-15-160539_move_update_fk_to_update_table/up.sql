ALTER TABLE updates ADD COLUMN reportid INTEGER NOT NULL;

ALTER TABLE updates
ADD CONSTRAINT updates_reportid_fkey
FOREIGN KEY (reportid) REFERENCES reports(reportid);