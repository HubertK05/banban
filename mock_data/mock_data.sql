INSERT INTO "columns" ("id", "name", "ordinal")
VALUES
    (1, 'New', 0),
    (2, 'In progress', 1),
    (3, 'Done', 2);

INSERT INTO "activities" ("id", "name", "body", "column_id", "ordinal")
VALUES
    (1, 'Profit', 'The $$$ will start rolling in', 1, 0),
    (2, 'Avoid burnout', 'Remember kids, burnout is unhealthy', 1, 1),
    (3, 'Hustle more', 'Stay focused, like a sigma', 2, 0),
    (4, 'Have an idea', 'Come up with a killer idea', 3, 0),
    (5, 'Get motivated', 'Watch motivational videos at 3am', 3, 1);

INSERT INTO "categories" ("id", "name", "ordinal")
VALUES
    (1, 'Size', 0),
    (2, 'Priority', 1);

INSERT INTO "category_tags" ("id", "tag_name", "category_id", "color", "ordinal")
VALUES
    (1, '🦔 Tiny', 1, 46079, 0),
    (2, '🐇 Small', 1, 510208, 1),
    (3, '🐂 Medium', 1, 14275072, 2),
    (4, '🦑 Large', 1, 16746248, 3),
    (5, '🐋 X-Large', 1, 16711680, 4),
    (6, '🏝 Low', 2, 207, 0),
    (7, '🏕 Medium', 2, 1429248, 1),
    (8, '🏔 High', 2, 13600512, 2),
    (9, '🌋 Urgent', 2, 16716820, 3),
    (10, '💼 Work', null, 15235358, 0),
    (11, '🍲 Cooking', null, 16766023, 1);

INSERT INTO "activity_tags" ("activity_id", "category_tag_id")
VALUES
    (1, 6),
    (2, 9),
    (2, 5),
    (3, 8),
    (3, 5),
    (4, 6),
    (4, 2),
    (5, 8),
    (5, 1);
