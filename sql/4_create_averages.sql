DROP TABLE IF EXISTS retention_weights;
CREATE TABLE IF NOT EXISTS retention_weights
(
    rec    INTEGER NOT NULL,
    weight REAL    NOT NULL,
    FOREIGN KEY (rec) REFERENCES records (id)
);

DROP INDEX IF EXISTS idx_retention_weights;
CREATE INDEX idx_retention_weights
    ON retention_weights (rec, weight);

DROP TABLE IF EXISTS average_retention_times_per_file;
CREATE TABLE IF NOT EXISTS average_retention_times_per_file
(
    sequence           INTEGER NOT NULL,
    file               INTEGER NOT NULL,
    weighted_retention REAL    NOT NULL,
    weighted_score     REAL    NOT NULL,
    average_retention  REAL    NOT NULL,
    average_score      REAL    NOT NULL,
    count              INTEGER NOT NULL,
    FOREIGN KEY (sequence) REFERENCES sequences (id),
    FOREIGN KEY (file) REFERENCES files (id)
);

DROP TABLE IF EXISTS average_retention_times;
CREATE TABLE IF NOT EXISTS average_retention_times
(
    sequence            INTEGER NOT NULL,
    weighted2_retention REAL    NOT NULL,
    weighted2_score     REAL    NOT NULL,
    count               INTEGER NOT NULL,
    count_all           INTEGER NOT NULL,
    PRIMARY KEY (sequence),
    FOREIGN KEY (sequence) REFERENCES sequences (id)
);

INSERT INTO retention_weights(rec, weight)
SELECT r.id, CASE WHEN ss.max_score = 0 THEN 0 ELSE POWER(r.score / ss.max_score, 2) END
FROM records r
         JOIN score_stats ss on ss.sequence = r.sequence AND ss.file = r.file;

INSERT INTO average_retention_times_per_file(sequence, file, weighted_retention, weighted_score, average_retention,
                                             average_score, count)
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
       avg(r.retention_time)                                         as average_retention,
       avg(r.score)                                                  as average_score,
       count(r.id)                                                   as count
FROM records r
         JOIN weights_query w ON r.id = w.id
GROUP BY r.sequence, r.file;

INSERT INTO average_retention_times(sequence, weighted2_retention, weighted2_score, count, count_all)
WITH max_query(sequence, max_score) AS (SELECT a.sequence, MAX(a.weighted_score)
                                        FROM average_retention_times_per_file a
                                        GROUP BY a.sequence),
     weights_query (sequence, file, retention, score, weight, count) AS (SELECT a.sequence,
                                                                                a.file,
                                                                                a.weighted_retention,
                                                                                a.weighted_score,
                                                                                CASE
                                                                                    WHEN mq.max_score = 0 THEN 0
                                                                                    ELSE POWER(a.weighted_score / mq.max_score, 2) END,
                                                                                a.count
                                                                         FROM average_retention_times_per_file a
                                                                                  JOIN max_query mq on a.sequence = mq.sequence)
SELECT wq.sequence,
       CASE
           WHEN sum(wq.weight) = 0 THEN 0
           ELSE sum(wq.retention * wq.weight) / sum(wq.weight) END                             as weighted2_retention,
       CASE WHEN sum(wq.weight) = 0 THEN 0 ELSE sum(wq.score * wq.weight) / sum(wq.weight) END as weigted2_score,
       count(wq.sequence)                                                                      as count,
       sum(wq.count)                                                                           as count_all
FROM weights_query wq
GROUP BY wq.sequence;

SELECT COUNT(*)
FROM average_retention_times