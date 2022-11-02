DROP INDEX IF EXISTS idx_seq_file;
CREATE INDEX idx_seq_file
    ON records (sequence, file);

DROP INDEX IF EXISTS idx_seq_file_retention;
CREATE INDEX idx_seq_file_retention
    ON records (sequence, file, retention_time);

DROP INDEX IF EXISTS idx_seq_file_score;
CREATE INDEX idx_seq_file_score
    ON records (sequence, file, score);