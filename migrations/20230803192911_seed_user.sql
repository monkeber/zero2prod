-- Add migration script here
INSERT INTO users (user_id, username, password_hash)
VALUES (
    'c328e642-9dc0-44a1-ae91-99491c710645',
    'admin',
    '$argon2id$v=19$m=15000,t=2,p=1$t1RBgiuZLEfJt487+ySQjw$8bus5muaN3K5VCxtdrGwN6oQBv1FeNo2WV9ieqpWWRI'
);
