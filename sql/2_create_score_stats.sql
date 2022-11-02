DROP TABLE IF EXISTS score_stats;

CREATE TABLE IF NOT EXISTS score_stats
(
    sequence  INTEGER NOT NULL,
    file      INTEGER NOT NULL,
    entries   INTEGER NOT NULL,
    min_score REAL    NOT NULL,
    max_score REAL    NOT NULL,
    avg_score REAL    NOT NULL,
    PRIMARY KEY (sequence, file),
    FOREIGN KEY (sequence) REFERENCES sequences (id),
    FOREIGN KEY (file) REFERENCES files (id)
);

INSERT INTO score_stats (sequence, file, entries, min_score, max_score, avg_score)
SELECT r.sequence, r.file, COUNT(r.id), MIN(r.score), MAX(r.score), AVG(r.score)
FROM records r
GROUP BY r.sequence, r.file;

DROP INDEX IF EXISTS idx_score_stats_helper;

CREATE INDEX IF NOT EXISTS idx_score_stats_helper
    ON score_stats (sequence, file, max_score);