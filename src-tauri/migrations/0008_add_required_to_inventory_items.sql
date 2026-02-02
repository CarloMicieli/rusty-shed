-- Add required quantity column to track_inventory_items table
-- This allows users to set target quantities for track planning

ALTER TABLE track_inventory_items ADD COLUMN required INTEGER DEFAULT 0 NOT NULL;

-- Index for querying items with shortage (stock < required)
CREATE INDEX idx_track_inventory_items_shortage 
ON track_inventory_items(inventory_id, track_id) 
WHERE quantity < required;
