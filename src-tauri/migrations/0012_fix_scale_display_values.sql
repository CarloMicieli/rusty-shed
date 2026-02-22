-- Fix scale values that were incorrectly stored as Display form (e.g. "H0 (1:87)")
-- due to a bug in update_scale() which used scale.to_string() instead of scale.as_code().
-- The SCREAMING_SNAKE_CASE forms used here match what SQLx derives via heck 0.4.
UPDATE railway_models
SET scale = CASE
    WHEN scale LIKE 'H0m %' THEN 'H0M'
    WHEN scale LIKE 'H0e %' THEN 'H0E'
    WHEN scale LIKE 'H0 %'  THEN 'H0'
    WHEN scale LIKE 'N %'   THEN 'N'
    WHEN scale LIKE 'TT %'  THEN 'TT'
    WHEN scale LIKE 'Z %'   THEN 'Z'
    WHEN scale LIKE 'G %'   THEN 'G'
    WHEN scale LIKE '1 %'   THEN 'SCALE1'
    WHEN scale LIKE '00 %'  THEN 'SCALE00'
    WHEN scale LIKE '0 %'   THEN 'SCALE0'
    ELSE scale
END
WHERE scale LIKE '% (%';
