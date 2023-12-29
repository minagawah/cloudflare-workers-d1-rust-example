DROP TABLE IF EXISTS users;
CREATE TABLE users (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  name TEXT,
  code TEXT,
  created_at TEXT DEFAULT NULL
);
INSERT INTO users (name, code, created_at) VALUES (
  'Joe', 
  'Iu82uORO5FSV1iIVXZAKym3JH3gKYKTdJJ3F6iUHkCQCEMsXl6ZVvAVHov0WHIrr',
  'Wed, 27 Dec 2023 02:57:05 +0000'
);
INSERT INTO users (name, code, created_at) VALUES (
  'Anna',
  'FeoJTtaI5KifCKbgNmIr6Cz6wxvs21OcOywSZvkoqEFtdcQGPGTyfHupli6sFH1r',
  'Wed, 27 Dec 2023 02:57:05 +0000'
);
INSERT INTO users (name, code, created_at) VALUES (
  'Paul',
  'KQKs0739LDBrPH8XJpzvt8Ir4k0vcD4qncBMm1Il1OREIsW0gT8C5arwj0YStXFL',
  'Wed, 27 Dec 2023 02:57:05 +0000'
);
