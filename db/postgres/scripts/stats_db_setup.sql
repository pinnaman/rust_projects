CREATE database stats_db;

\c stats_db

DROP table string_stats;
CREATE TABLE string_stats (
  index serial,
  uid text,
  number_cnt smallint,
  alpha_cnt smallint,
  vowel_cnt smallint,
  ins_date DATE NOT NULL DEFAULT NOW()
);