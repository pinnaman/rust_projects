CREATE database stats_db;

\c stats_db

DROP table string_stats;
CREATE TABLE string_stats (
  index serial,
  uid text,
  number_cnt text,
  alpha_cnt text,
  vowel_cnt text,
  ins_date DATE NOT NULL DEFAULT NOW()
);