--: User()

--! get_users : User
SELECT 
    id,
    email
FROM users;

--! create_user
INSERT INTO users (email, hashed_password)
VALUES(:email, :hashed_password);