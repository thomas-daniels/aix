CREATE TABLE games (
  $HEADERS,
  movedata BLOB,
  clocks_white USMALLINT[],
  clocks_black USMALLINT[],
  evals SMALLINT[],
  ply_count USMALLINT,
);