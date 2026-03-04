-- Fix purchase_infos rows that were inserted without a purchase_type.
-- All existing rows without a type were created by the 'add to collection'
-- flow which only creates purchased records, so defaulting to 'PURCHASED' is safe.
UPDATE purchase_infos
SET purchase_type = 'PURCHASED'
WHERE purchase_type IS NULL;
