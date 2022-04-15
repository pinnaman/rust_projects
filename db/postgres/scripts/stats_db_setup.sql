CREATE database stats_db;

\c stats_db

DROP table string_stats;
CREATE TABLE string_stats (
  index serial,
  uid text,
  ins_date DATE NOT NULL DEFAULT CURRENT_DATE
);