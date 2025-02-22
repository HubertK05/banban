INSERT INTO "columns" ("id", "name", "ordinal") VALUES (1, 'Col1', 0), (2, 'Col2', 1);
INSERT INTO "activities" ("id", "name", "body", "column_id", "ordinal")
VALUES
    (1, '1', '', 1, 0),
    (2, '2', '', 1, 1),
    (3, '3', '', 1, 2),
    (4, '4', '', 2, 0),
    (5, '5', '', 2, 1),
    (6, '6', '', 2, 2);
