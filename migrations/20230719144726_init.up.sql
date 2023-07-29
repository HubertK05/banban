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
    FOREIGN KEY (column_id) REFERENCES columns(id) ON DELETE SET NULL
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
    color INTEGER NOT NULL,
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
