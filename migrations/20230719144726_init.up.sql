CREATE TABLE columns (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name TEXT NOT NULL
);

CREATE TABLE activities (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name TEXT NOT NULL,
    body TEXT,
    column_id INTEGER,
    FOREIGN KEY (column_id) REFERENCES columns(id)
);

CREATE TABLE categories (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name TEXT NOT NULL
);

CREATE TABLE category_tags (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    tag_name TEXT NOT NULL,
    category_id INTEGER,
    UNIQUE (tag_name, category_id),
    FOREIGN KEY (category_id) REFERENCES categories(id)
);

CREATE TABLE activity_tags (
    activity_id INTEGER NOT NULL,
    category_tag_id INTEGER NOT NULL,
    PRIMARY KEY (activity_id, category_tag_id),
    FOREIGN KEY (activity_id) REFERENCES activities(id),
    FOREIGN KEY (category_tag_id) REFERENCES category_tags(id)
);
