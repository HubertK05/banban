CREATE TABLE columns (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name TEXT NOT NULL,
    ordinal INT NOT NULL
);

CREATE TABLE activities (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name TEXT NOT NULL,
    body TEXT,
    column_id INTEGER,
    ordinal INT NOT NULL,
    FOREIGN KEY (column_id) REFERENCES columns(id) ON DELETE CASCADE
);

CREATE TABLE categories (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name TEXT NOT NULL,
    ordinal INT NOT NULL
);

CREATE TABLE category_tags (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    tag_name TEXT NOT NULL,
    category_id INTEGER,
    ordinal INT NOT NULL,
    UNIQUE (tag_name, category_id),
    FOREIGN KEY (category_id) REFERENCES categories(id) ON DELETE CASCADE
);

CREATE TABLE activity_tags (
    activity_id INTEGER NOT NULL,
    category_tag_id INTEGER NOT NULL,
    PRIMARY KEY (activity_id, category_tag_id),
    FOREIGN KEY (activity_id) REFERENCES activities(id) ON DELETE CASCADE,
    FOREIGN KEY (category_tag_id) REFERENCES category_tags(id) ON DELETE CASCADE
);

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

INSERT INTO "category_tags" ("id", "tag_name", "category_id", "ordinal")
VALUES
    (1, 'Tiny', 1, 0),
    (2, 'Small', 1, 1),
    (3, 'Medium', 1, 2),
    (4, 'Large', 1, 3),
    (5, 'X-Large', 1, 4),
    (6, 'Low', 2, 0),
    (7, 'Medium', 2, 1),
    (8, 'High', 2, 2),
    (9, 'Urgent', 2, 3);

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
    