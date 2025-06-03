CREATE TABLE users (
  user_id Text PRIMARY KEY,
  email VARCHAR(255) NOT NULL,
  name VARCHAR(255) NOT NULL,
  surname VARCHAR(255) NOT NULL,
  student_id VARCHAR(255),
  user_disabled BOOLEAN,
  last_login_time TIMESTAMP
);

CREATE TABLE roles (
  role_id Text PRIMARY KEY,
  name VARCHAR(255),
  permissions INT
);

CREATE TABLE subjects (
  subject_id Text PRIMARY KEY,
  subject_name VARCHAR(255),
  editor_role_id Text NOT NULL,
  FOREIGN KEY (editor_role_id) REFERENCES Roles(role_id)
);

CREATE TABLE user_subjects (
  user_id Text,
  subject_id Text,
  role_id Text,
  grade DECIMAL(3,2),
  PRIMARY KEY (user_id, subject_id),
  FOREIGN KEY (user_id) REFERENCES Users(user_id),
  FOREIGN KEY (subject_id) REFERENCES Subjects(subject_id),
  FOREIGN KEY (role_id) REFERENCES Roles(role_id)
);

CREATE TABLE assigments (
  assigment_id Text PRIMARY KEY,
  subject_id Text NOT NULL,
  title VARCHAR(255),
  description VARCHAR(1024),
  FOREIGN KEY (subject_id) REFERENCES Subjects(subject_id)
);

CREATE TABLE solution (
  solution_id Text PRIMARY KEY,
  grade DECIMAL(3,2),
  submission_date TIMESTAMP,
  solution_data BINARY,
  reviewed_by Text,
  review_date TIMESTAMP,
  FOREIGN KEY (reviewed_by) REFERENCES Users(user_id)
);

CREATE TABLE user_solution_assignments (
  user_id Text,
  solution_id Text,
  assigment_id Text,
  PRIMARY KEY (user_id, solution_id, assigment_id),
  FOREIGN KEY (user_id) REFERENCES Users(user_id),
  FOREIGN KEY (solution_id) REFERENCES Solution(solution_id),
  FOREIGN KEY (assigment_id) REFERENCES Assigments(assigment_id)
);

CREATE TABLE microsoft_logins (
  microsoft_login_id Text PRIMARY KEY,
  microsoft_id VARCHAR(255),
  user_id Text,
  FOREIGN KEY (user_id) REFERENCES Users(user_id)
);

CREATE TABLE session_refresh_keys (
  refresh_key_id Text PRIMARY KEY,
  user_id Text NOT NULL,
  expiration_time TIMESTAMP,
  refresh_count INT,
  refresh_limit INT,
  FOREIGN KEY (user_id) REFERENCES Users(user_id)
);
