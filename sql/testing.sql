CREATE INDEX idx_seq_file
    ON records (sequence, file);

DROP TABLE IF EXISTS score_stats;

CREATE TABLE score_stats
(
    sequence  INTEGER NOT NULL,
    file      INTEGER NOT NULL,
    entries   INTEGER NOT NULL,
    min_score REAL    NOT NULL,
    max_score REAL    NOT NULL,
    avg_score REAL    NOT NULL,
    FOREIGN KEY (sequence) REFERENCES sequences (id),
    FOREIGN KEY (file) REFERENCES files (id)
);

INSERT INTO score_stats (sequence, file, entries, min_score, max_score, avg_score)
SELECT r.sequence, r.file, COUNT(r.id), MIN(r.score), MAX(r.score), AVG(r.score)
FROM records r
GROUP BY r.sequence, r.file;

SELECT *
FROM score_stats;

SELECT r.id, r.retention_time, r.score, (r.score / ss.max_score) as weight, ss.max_score, ss.avg_score
FROM records r
         JOIN score_stats ss on ss.sequence = r.sequence AND ss.file = r.file
WHERE r.sequence = 1
  AND r.file = 2;

-- CREATE OR REPLACE FUNCTION wavg_accum (float4, float4, float4) RETURNS float4 AS
--     $$
--         SELECT $1 * $2
--     $$ LANGUAGE 'sql' STRICT;
--
-- CREATE AGGREGATE wavg (float4, float4)
-- (
--     STYPE = float4,
--     SFUNC = wavg_accum
-- );

WITH weights_query (id, weight) AS (SELECT r.id, (r.score / ss.max_score)
                                    FROM records r
                                             JOIN score_stats ss on ss.sequence = r.sequence AND ss.file = r.file)
SELECT r.id, r.retention_time, r.score, w.weight
FROM records r
         JOIN weights_query w ON r.id = w.id
WHERE r.sequence = 1
  AND r.file = 2;

WITH weights_query (id, weight) AS (SELECT r.id, (r.score / ss.max_score)
                                    FROM records r
                                             JOIN score_stats ss on ss.sequence = r.sequence AND ss.file = r.file)
SELECT sum(r.retention_time * w.weight) / sum(w.weight) as weighted_average,
       avg(r.retention_time)                            as average,
       count(r.id)
FROM records r
         JOIN weights_query w ON r.id = w.id
GROUP BY r.sequence, r.file
LIMIT 2;

WITH weights_query (id, weight) AS (SELECT rw.rec, rw.weight FROM retention_weights rw)
SELECT r.sequence,
       r.file,
       CASE
           WHEN sum(w.weight) = 0
               THEN 0
           ELSE sum(r.retention_time * w.weight) / sum(w.weight) END as weighted_retention,
       CASE
           WHEN sum(w.weight) = 0
               THEN 0
           ELSE sum(r.score * w.weight) / sum(w.weight) END          as weighted_score,
       avg(r.retention_time)                                         as average,
       avg(r.score)                                                  as average_score,
       count(r.id)                                                   as count
FROM records r
         JOIN weights_query w
              ON r.id = w.id
WHERE r.sequence = 1
  AND r.file = 2
GROUP BY r.sequence, r.file;

SELECT s.id                                                          as seq_id,
       s.sequence                                                    as seq_full,
       a.weighted2_retention                                         as retention_time,
       a.weighted2_score                                             as score,
       CASE WHEN e.sequence IS NULL THEN true ELSE false END         as is_ox,
       CASE WHEN e.sequence IS NULL THEN 0 ELSE e.weight END         as mass,
       CASE WHEN e.sequence IS NULL THEN 0 ELSE e.kyte_doolittle END as hs_kd,
       CASE WHEN e.sequence IS NULL THEN 0 ELSE e.hopp_woods END     as hs_hw,
       CASE WHEN e.sequence IS NULL THEN 0 ELSE e.cornette END       as hs_c
FROM average_retention_times a
         JOIN sequences s on a.sequence = s.id
         LEFT JOIN extras e on a.sequence = e.sequence
OFFSET 0 LIMIT 100;

SELECT r.id                                                          as rec_id,
       r.sequence                                                    as seq_id,
       r.file                                                        as file_id,
       r.retention_time                                              as retention_time,
       r.score                                                       as score,
       CASE WHEN e.sequence IS NULL THEN true ELSE false END         as is_ox,
       CASE WHEN e.sequence IS NULL THEN 0 ELSE e.weight END         as mass,
       CASE WHEN e.sequence IS NULL THEN 0 ELSE e.kyte_doolittle END as hs_kd,
       CASE WHEN e.sequence IS NULL THEN 0 ELSE e.hopp_woods END     as hs_hw,
       CASE WHEN e.sequence IS NULL THEN 0 ELSE e.cornette END       as hs_c
FROM records r
         LEFT JOIN extras e on r.sequence = e.sequence
ORDER BY r.id
OFFSET 0 LIMIT 100;

SELECT r.id                                                          as rec_id,
       r.sequence                                                    as seq_id,
       r.file                                                        as file_id,
       r.retention_time                                              as retention_time,
       r.score                                                       as score,
       CASE WHEN e.sequence IS NULL THEN true ELSE false END         as is_ox,
       CASE WHEN e.sequence IS NULL THEN 0 ELSE e.weight END         as mass,
       CASE WHEN e.sequence IS NULL THEN 0 ELSE e.kyte_doolittle END as hs_kd,
       CASE WHEN e.sequence IS NULL THEN 0 ELSE e.hopp_woods END     as hs_hw,
       CASE WHEN e.sequence IS NULL THEN 0 ELSE e.cornette END       as hs_c
FROM records r
         LEFT JOIN extras e on r.sequence = e.sequence
ORDER BY r.id
OFFSET 0 LIMIT 100;



SELECT a.sequence                                                    as seq_id,
       a.file                                                        as file_id,
       a.weighted_retention                                          as retention_time,
       a.weighted_score                                              as score,
       a.count                                                       as count,
       CASE WHEN e.sequence IS NULL THEN true ELSE false END         as is_ox,
       CASE WHEN e.sequence IS NULL THEN 0 ELSE e.weight END         as mass,
       CASE WHEN e.sequence IS NULL THEN 0 ELSE e.kyte_doolittle END as hs_kd,
       CASE WHEN e.sequence IS NULL THEN 0 ELSE e.hopp_woods END     as hs_hw,
       CASE WHEN e.sequence IS NULL THEN 0 ELSE e.cornette END       as hs_c
FROM average_retention_times_per_file a
         LEFT JOIN extras e on a.sequence = e.sequence
ORDER BY a.sequence, a.file
OFFSET 0 LIMIT 100;

SELECT COUNT(*) FROM average_retention_times_per_file;

SELECT MAX(LENGTH(s.sequence)) FROM sequences s;

SELECT
    s.sequence,
    REPLACE(s.sequence, '(ox)', 'X'),
    r.weighted2_retention
FROM sequences s
JOIN average_retention_times r ON s.id = r.sequence
WHERE s.sequence ILIKE '%(ox)%';

SELECT s.id, s.sequence, e.weight, e.kyte_doolittle, e.cornette, e.hopp_woods
FROM sequences s
JOIN extras e on s.id = e.sequence
LIMIT 10;

SELECT count(a.sequence)
FROM average_retention_times a
JOIN extras e on a.sequence = e.sequence
WHERE a.weighted2_score > 200