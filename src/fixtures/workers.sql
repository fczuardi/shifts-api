CREATE TYPE "Profession" AS ENUM ('CNA', 'LVN', 'RN');

CREATE TABLE "Worker" (
    id serial PRIMARY KEY,
    name text NOT NULL,
    is_active boolean NOT NULL DEFAULT false,
    profession "Profession" NOT NULL
);

INSERT INTO "Worker"
  (id, name, is_active, profession) VALUES
  (4, 'Active Worker', true, 'CNA');

INSERT INTO "Worker"
  (id, name, is_active, profession) VALUES
  (5, 'Inactive Worker', false, 'LVN');
