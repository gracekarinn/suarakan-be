-- Drop old foreign keys to `admins`
ALTER TABLE publications DROP CONSTRAINT IF EXISTS publications_adminid_fkey;
ALTER TABLE updates DROP CONSTRAINT IF EXISTS updates_adminid_fkey;

-- Change column type from Int4 to Int8 (matching authentication_user.id)
ALTER TABLE publications ALTER COLUMN adminid TYPE BIGINT;
ALTER TABLE updates ALTER COLUMN adminid TYPE BIGINT;

-- Add new foreign keys to `authentication_user`
ALTER TABLE publications ADD CONSTRAINT publications_adminid_fkey
    FOREIGN KEY (adminid) REFERENCES authentication_user(id);

ALTER TABLE updates ADD CONSTRAINT updates_adminid_fkey
    FOREIGN KEY (adminid) REFERENCES authentication_user(id);
