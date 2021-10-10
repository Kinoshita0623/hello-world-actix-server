-- Your SQL goes here
CREATE TABLE `users` (
    `id` BIGINT PRIMARY KEY AUTO_INCREMENT,
    `username` VARCHAR(255) NOT NULL,
    `encrypted_password` VARCHAR(255) NOT NULL
);