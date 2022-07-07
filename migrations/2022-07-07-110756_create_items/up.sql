-- Your SQL goes here
CREATE TABLE `users` (
	`id` SERIAL PRIMARY KEY AUTO_INCREMENT,
	`username` VARCHAR(30) NOT NULL,
	`password` VARCHAR(255) NOT NULL,
	`email` VARCHAR(255) NOT NULL
);
CREATE TABLE `items` (
	`id` SERIAL PRIMARY KEY,
	`content` TEXT NOT NULL,
	`user_id` BIGINT UNSIGNED NOT NULL,
	`is_important` BOOLEAN NOT NULL DEFAULT false,
    CONSTRAINT fk_user_id
        FOREIGN KEY (user_id)
            REFERENCES users (id)
            ON DELETE CASCADE
);