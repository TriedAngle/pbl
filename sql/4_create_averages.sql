DROP TABLE IF EXISTS average_retention_times;

CREATE TABLE IF NOT EXISTS average_retention_times
(
    sequence INTEGER NOT NULL,
    file     INTEGER NOT NULL,
    weighted REAL    NOT NULL,
    average  REAL    NOT NULL,
    FOREIGN KEY (sequence) REFERENCES sequences (id),
    FOREIGN KEY (file) REFERENCES files (id)
);

INSERT INTO average_retention_times(sequence, file, weighted, average)
WITH weights_query (id, weight) AS (SELECT r.id, (r.score / ss.max_score)
                                    FROM records r
                                             JOIN score_stats ss on ss.sequence = r.sequence AND ss.file = r.file)
SELECT r.sequence,
       r.file,
       sum(r.retention_time * w.weight) / sum(w.weight) as weighted_average,
       avg(r.retention_time)                            as average
FROM records r
         JOIN weights_query w ON r.id = w.id
GROUP BY r.sequence, r.file
