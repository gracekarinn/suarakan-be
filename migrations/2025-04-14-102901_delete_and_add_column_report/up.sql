ALTER TABLE reports DROP COLUMN reporteroccupation;
ALTER TABLE reports ADD COLUMN reporterid INTEGER NOT NULL REFERENCES reporters(reporterid);