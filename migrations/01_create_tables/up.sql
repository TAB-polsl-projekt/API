CREATE TABLE users (
  user_id TEXT NOT NULL PRIMARY KEY,
  name TEXT NOT NULL,
  surname TEXT NOT NULL,
  student_id TEXT,
  is_admin BOOLEAN NOT NULL
);

CREATE TABLE subjects (
  subject_id TEXT NOT NULL PRIMARY KEY,
  subject_name TEXT NOT NULL
);

CREATE TABLE roles (
  role_id TEXT NOT NULL PRIMARY KEY,
  name TEXT NOT NULL
);

CREATE TABLE user_role (
  role_id TEXT NOT NULL,
  user_id TEXT NOT NULL,
  PRIMARY KEY (role_id, user_id),
  FOREIGN KEY (role_id) REFERENCES roles(role_id) ON DELETE CASCADE,
  FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE
);

CREATE TABLE subject_role (
  subject_id TEXT NOT NULL,
  role_id TEXT NOT NULL,
  PRIMARY KEY (subject_id, role_id),
  FOREIGN KEY (subject_id) REFERENCES subjects(subject_id) ON DELETE CASCADE,
  FOREIGN KEY (role_id) REFERENCES roles(role_id) ON DELETE CASCADE
);

CREATE TABLE assignments (
  assignment_id TEXT NOT NULL PRIMARY KEY,
  subject_id TEXT NOT NULL,
  title TEXT NOT NULL,
  description TEXT NOT NULL,
  accepted_mime_types TEXT NOT NULL,
  FOREIGN KEY (subject_id) REFERENCES subjects(subject_id) ON DELETE CASCADE
);

CREATE TABLE solutions (
  solution_id TEXT NOT NULL PRIMARY KEY,
  grade DECIMAL(3,2),
  submission_date TIMESTAMP NOT NULL,
  solution_data BLOB NOT NULL,
  reviewed_by TEXT,
  review_comment TEXT,
  review_date TIMESTAMP,
  mime_type TEXT NOT NULL,
  assignment_id TEXT NOT NULL,
  FOREIGN KEY (reviewed_by) REFERENCES users(user_id),
  FOREIGN KEY (assignment_id) REFERENCES assignments(assignment_id) ON DELETE CASCADE
);

CREATE TABLE user_solution (
  user_id TEXT NOT NULL,
  solution_id TEXT NOT NULL,
  PRIMARY KEY (user_id, solution_id),
  FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE,
  FOREIGN KEY (solution_id) REFERENCES solutions(solution_id) ON DELETE CASCADE
);

CREATE TABLE session_ids (
  refresh_key_id TEXT NOT NULL PRIMARY KEY,
  user_id TEXT NOT NULL,
  expiration_time TIMESTAMP NOT NULL,
  FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE
);

CREATE TABLE logins (
  login_id TEXT NOT NULL PRIMARY KEY,
  user_id TEXT NOT NULL,
  email TEXT NOT NULL,
  passwd_hash TEXT NOT NULL,
  FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE
);