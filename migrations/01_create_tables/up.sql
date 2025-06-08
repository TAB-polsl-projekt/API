PRAGMA foreign_keys = ON;

-- 1) Solutions (so users → solution.reviewed_by can reference it)
CREATE TABLE solution (
  solution_id       TEXT    PRIMARY KEY,
  grade             REAL,
  submission_date   TIMESTAMP,      -- ISO timestamp
  solution_data     BLOB,
  reviewed_by       TEXT,      -- user_id of the reviewer
  review_comment    TEXT,
  review_date       TIMESTAMP,      -- ISO timestamp
  mime_type         TEXT
);

-- 2) Users
CREATE TABLE users (
  user_id           TEXT    PRIMARY KEY,
  email             TEXT    NOT NULL,
  name              TEXT    NOT NULL,
  surname           TEXT    NOT NULL,
  student_id        TEXT,
  user_disabled     BOOLEAN NOT NULL,
  last_login_time   TIMESTAMP,              -- ISO timestamp
  FOREIGN KEY(user_id)        REFERENCES solution(reviewed_by)
);

-- 3) Subjects
CREATE TABLE subjects (
  subject_id        TEXT    PRIMARY KEY,
  subject_name      TEXT,
  editor_role_id    TEXT    NOT NULL,
  FOREIGN KEY(editor_role_id) REFERENCES roles(role_id)
);

-- 4) Roles
CREATE TABLE roles (
  role_id           TEXT    PRIMARY KEY,
  name              TEXT    NOT NULL,
  permissions       INTEGER NOT NULL,
  FOREIGN KEY(role_id)       REFERENCES user_subjects(role_id)
);

-- 5) User–Subject assignments
CREATE TABLE user_subjects (
  user_id           TEXT,
  subject_id        TEXT,
  role_id           TEXT    NOT NULL,
  grade             REAL,
  PRIMARY KEY (user_id, subject_id),
  FOREIGN KEY(user_id)      REFERENCES users(user_id),
  FOREIGN KEY(subject_id)   REFERENCES subjects(subject_id)
);

-- 6) Assignments
CREATE TABLE assignments (
  assignment_id        TEXT    PRIMARY KEY,
  subject_id           TEXT    NOT NULL,
  title                TEXT    NOT NULL,
  description          TEXT,
  accepted_mime_types  TEXT,       -- e.g. JSON array or CSV list
  FOREIGN KEY(subject_id) REFERENCES subjects(subject_id)
);

-- 7) User–Solution–Assignment link
CREATE TABLE user_solution_assignments (
  user_id        TEXT,
  solution_id    TEXT,
  assignment_id  TEXT,
  PRIMARY KEY (user_id, solution_id, assignment_id),
  FOREIGN KEY(user_id)       REFERENCES users(user_id),
  FOREIGN KEY(solution_id)   REFERENCES solution(solution_id),
  FOREIGN KEY(assignment_id) REFERENCES assignments(assignment_id)
);

-- 8) Microsoft Logins
CREATE TABLE microsoft_logins (
  microsoft_login_id  TEXT    PRIMARY KEY,
  microsoft_id        TEXT    NOT NULL,
  user_id             TEXT    NOT NULL,
  FOREIGN KEY(user_id) REFERENCES users(user_id)
);

-- 9) Session refresh keys
CREATE TABLE session_refresh_keys (
  refresh_key_id   TEXT    PRIMARY KEY,
  user_id          TEXT    NOT NULL,
  expiration_time  TIMESTAMP    NOT NULL,  -- ISO timestamp
  refresh_count    INTEGER NOT NULL,
  refresh_limit    INTEGER NOT NULL,
  FOREIGN KEY(user_id) REFERENCES users(user_id)
);
