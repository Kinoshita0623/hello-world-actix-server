-- Your SQL goes here
CREATE TABLE `posts` (
    `id` BIGINT PRIMARY KEY AUTO_INCREMENT,
    `title` VARCHAR(255) NOT NULL,
    `text` TEXT NOT NULL
);