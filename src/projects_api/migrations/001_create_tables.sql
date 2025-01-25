CREATE TABLE IF NOT EXISTS teams
(
    id   INTEGER PRIMARY KEY,
    name TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS members
(
    identity  TEXT PRIMARY KEY,
    full_name TEXT    NOT NULL,
    score     INTEGER NOT NULL,
    team_id   INTEGER NOT NULL,
    FOREIGN KEY (team_id) REFERENCES teams (id)
);

CREATE TABLE criteria_sets
(
    id   INTEGER PRIMARY KEY,
    name TEXT NOT NULL
);

CREATE TABLE criterion
(
    id              INTEGER PRIMARY KEY,
    name            TEXT    NOT NULL,
    point           INTEGER NOT NULL,
    criteria_set_id INTEGER NOT NULL,
    FOREIGN KEY (criteria_set_id) REFERENCES criteria_sets (id)
);

CREATE TABLE projects
(
    id              INTEGER PRIMARY KEY,
    name            TEXT    NOT NULL,
    language        TEXT    NOT NULL,
    summary         TEXT    NOT NULL,
    criteria_set_id INTEGER NOT NULL,
    FOREIGN KEY (criteria_set_id) REFERENCES criteria_sets (id)
);

CREATE TABLE assignments
(
    project_id INTEGER NOT NULL,
    team_id    INTEGER NOT NULL,
    status     TEXT    NOT NULL CHECK (status IN ('Planned', 'InProgress', 'Completed', 'Failed')),
    start_date TEXT    NOT NULL,
    end_date   TEXT    NOT NULL,
    repository TEXT    NOT NULL,
    PRIMARY KEY (project_id, team_id),
    FOREIGN KEY (project_id) REFERENCES projects (id),
    FOREIGN KEY (team_id) REFERENCES teams (id)
);

CREATE TABLE history
(
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    time        TEXT NOT NULL,
    event       TEXT NOT NULL,
    description TEXT NOT NULL
);