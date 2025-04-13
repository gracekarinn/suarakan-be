-- Drop foreign keys to `authentication_user`
ALTER TABLE publications DROP CONSTRAINT IF EXISTS publications_adminid_fkey;
ALTER TABLE updates DROP CONSTRAINT IF EXISTS updates_adminid_fkey;

-- Change column type back to Int4
ALTER TABLE publications ALTER COLUMN adminid TYPE INTEGER;
ALTER TABLE updates ALTER COLUMN adminid TYPE INTEGER;

-- Re-add foreign keys to `admins`
ALTER TABLE publications ADD CONSTRAINT publications_adminid_fkey
    FOREIGN KEY (adminid) REFERENCES admins(adminid);

ALTER TABLE updates ADD CONSTRAINT updates_adminid_fkey
    FOREIGN KEY (adminid) REFERENCES admins(adminid);
