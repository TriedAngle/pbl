DROP INDEX IF EXISTS idx_seq_file;

CREATE INDEX idx_seq_file
    ON records (sequence, file);
