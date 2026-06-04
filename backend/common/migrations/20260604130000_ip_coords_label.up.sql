-- Cache the city/state label alongside the coordinates so the finder can
-- pre-fill the "Near" field from an IP geolocation.
ALTER TABLE c_ip_coords ADD COLUMN city text;
ALTER TABLE c_ip_coords ADD COLUMN state text;
