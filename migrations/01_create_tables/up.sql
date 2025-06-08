CREATE TABLE users (
  user_id TEXT PRIMARY KEY NOT NULL,
  email TEXT NOT NULL,
  name TEXT NOT NULL,
  surname TEXT NOT NULL,
  student_id TEXT,
  user_disabled BOOLEAN NOT NULL,
  last_login_time DATETIME,
  is_admin BOOLEAN  NOT NULL DEFAULT 0
);

CREATE TABLE roles (
  role_id TEXT PRIMARY KEY NOT NULL,
  name TEXT NOT NULL,
  permissions INTEGER NOT NULL
);

CREATE TABLE subjects (
  subject_id TEXT PRIMARY KEY NOT NULL,
  subject_name TEXT,
  editor_role_id TEXT NOT NULL,
  FOREIGN KEY (editor_role_id) REFERENCES roles(role_id)
);

CREATE TABLE user_subjects (
  user_id TEXT NOT NULL,
  subject_id TEXT NOT NULL,
  role_id TEXT NOT NULL,
  grade DECIMAL(3,2),
  PRIMARY KEY (user_id, subject_id),
  FOREIGN KEY (user_id) REFERENCES users(user_id),
  FOREIGN KEY (subject_id) REFERENCES subjects(subject_id),
  FOREIGN KEY (role_id) REFERENCES roles(role_id)
);

CREATE TABLE assignments (
  assignment_id TEXT PRIMARY KEY NOT NULL,
  subject_id TEXT NOT NULL,
  title TEXT NOT NULL,
  description TEXT,
  accepted_mime_types TEXT, -- Store as comma-separated or JSON string
  FOREIGN KEY (subject_id) REFERENCES subjects(subject_id)
);

CREATE TABLE solution (
  solution_id TEXT PRIMARY KEY NOT NULL,
  grade DECIMAL(3,2),
  submission_date DATETIME,
  solution_data BLOB,
  reviewed_by TEXT,
  review_comment TEXT,
  review_date DATETIME,
  mime_type TEXT,
  FOREIGN KEY (reviewed_by) REFERENCES users(user_id)
);

CREATE TABLE user_solution_assignments (
  user_id TEXT NOT NULL,
  solution_id TEXT NOT NULL,
  assignment_id TEXT NOT NULL,
  PRIMARY KEY (user_id, solution_id, assignment_id),
  FOREIGN KEY (user_id) REFERENCES users(user_id),
  FOREIGN KEY (solution_id) REFERENCES solution(solution_id),
  FOREIGN KEY (assignment_id) REFERENCES assignments(assignment_id)
);

CREATE TABLE microsoft_logins (
  microsoft_login_id TEXT PRIMARY KEY NOT NULL,
  microsoft_id TEXT NOT NULL,
  user_id TEXT NOT NULL UNIQUE,
  FOREIGN KEY (user_id) REFERENCES users(user_id)
);

CREATE TABLE session_refresh_keys (
  refresh_key_id TEXT PRIMARY KEY NOT NULL,
  user_id TEXT NOT NULL,
  expiration_time DATETIME NOT NULL,
  refresh_count INTEGER NOT NULL,
  refresh_limit INTEGER NOT NULL,
  FOREIGN KEY (user_id) REFERENCES users(user_id)
);