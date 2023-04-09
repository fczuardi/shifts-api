CREATE TABLE "Facility" (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT false
);

INSERT INTO "Facility"
  (id, is_active, name) VALUES
  ('4','False', 'b3b0bd75669fc5110fdae9048f9300738e13f51c');

INSERT INTO "Facility"
  (id, is_active, name) VALUES
  ('5','True', '94866511b2a4b9a26e3634ec828ba9d65559840f');
