ALTER TABLE reports DROP CONSTRAINT IF EXISTS reports_reporterid_fkey;

ALTER TABLE reports ADD CONSTRAINT reports_reporterid_fkey
    FOREIGN KEY (reporterid) REFERENCES authentication_user(id);