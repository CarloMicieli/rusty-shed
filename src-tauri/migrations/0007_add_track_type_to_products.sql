-- Add track_type and description columns to track_products table
-- This allows categorization of track pieces by their geometric type
-- and provides a description field for product details

ALTER TABLE track_products ADD COLUMN track_type TEXT;
ALTER TABLE track_products ADD COLUMN description TEXT;

-- Set default value for existing records (STRAIGHT is most common)
UPDATE track_products SET track_type = 'STRAIGHT' WHERE track_type IS NULL;
