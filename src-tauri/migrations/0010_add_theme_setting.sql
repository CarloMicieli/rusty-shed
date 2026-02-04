-- noinspection SqlResolveForFile
-- noinspection SqlNoDataSourceInspectionForFile
-- noinspection SqlResolveInspectionForFile

-- Add theme column to settings table
ALTER TABLE settings ADD COLUMN theme TEXT NOT NULL DEFAULT 'system';
