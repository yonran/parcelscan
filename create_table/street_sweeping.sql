-- Street Sweeping Schedule
-- Source: https://data.sfgov.org/City-Infrastructure/Street-Sweeping-Schedule/yhqp-riqs
-- Raw schema: https://data.sfgov.org/api/views/yhqp-riqs.json
--
-- A. SUMMARY
-- Mechanical street sweeping and street cleaning schedule managed by San Francisco Public Works.
--
-- B. HOW THE DATASET IS CREATED
-- This dataset is created by extracting all street sweeping schedule data from a Department of
-- Public Works database, it is then geocoded to add common identifiers such as Centerline Network
-- Number ("CNN") then published to the open data portal.
--
-- C. UPDATE PROCESS
-- This dataset will be updated on an 'as needed' basis, when sweeping schedules change.
--
-- D. HOW TO USE THIS DATASET
-- Use this dataset to understand, track, or analyze street sweeping in San Francisco.
--
-- KEY ATTRIBUTES:
-- - CNN = Centerline Network Number (unique street segment ID)
-- - Week1-Week5 = Binary flags (1/0) indicating which weeks of the month sweeping occurs
-- - Line = LINESTRING geometry (WKT format) for mapping
--
-- NOTES:
-- - Sweeping frequency per month = Week1 + Week2 + Week3 + Week4 + Week5
-- - BlockSide can be NULL (839 out of 37878 records)
-- - Join to parcels: Use CNN for street segment matching

-- Load spatial extension for geometry functions
INSTALL spatial;
LOAD spatial;

-- Centralize the source file name for reuse in map metadata.
CREATE MACRO IF NOT EXISTS street_sweeping_source_file() AS 'data/yhqp-riqs.csv';
CREATE MACRO IF NOT EXISTS street_sweeping_metadata_file() AS 'data/yhqp-riqs-meta.json';

CREATE OR REPLACE VIEW street_sweeping AS
SELECT
    * REPLACE (
        ST_GeomFromText(Line) AS Line
    )
FROM read_csv(street_sweeping_source_file(),
    header=true,
    columns={
        'CNN': 'VARCHAR NOT NULL',
        'Corridor': 'VARCHAR NOT NULL',
        'Limits': 'VARCHAR NOT NULL',
        'CNNRightLeft': 'VARCHAR NOT NULL',
        'BlockSide': 'VARCHAR',
        'FullName': 'VARCHAR NOT NULL',
        'WeekDay': 'VARCHAR NOT NULL',
        'FromHour': 'INTEGER NOT NULL',
        'ToHour': 'INTEGER NOT NULL',
        'Week1': 'INTEGER NOT NULL',
        'Week2': 'INTEGER NOT NULL',
        'Week3': 'INTEGER NOT NULL',
        'Week4': 'INTEGER NOT NULL',
        'Week5': 'INTEGER NOT NULL',
        'Holidays': 'INTEGER NOT NULL',
        'BlockSweepID': 'VARCHAR NOT NULL',
        'Line': 'VARCHAR NOT NULL'
    }
);

-- -- Test: Verify data loaded correctly
-- SELECT
--     COUNT(*) AS total_segments,
--     COUNT(DISTINCT CNN) AS distinct_streets,
--     SUM(CASE WHEN BlockSide IS NULL THEN 1 ELSE 0 END) AS null_blockside,
--     -- Verify NOT NULL columns have no nulls
--     SUM(CASE WHEN CNN IS NULL THEN 1 ELSE 0 END) AS null_cnn,
--     SUM(CASE WHEN Line IS NULL THEN 1 ELSE 0 END) AS null_line
-- FROM street_sweeping;
