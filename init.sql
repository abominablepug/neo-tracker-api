CREATE TABLE users (
	id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
	username VARCHAR(255) NOT NULL UNIQUE,
	email VARCHAR(255) NOT NULL UNIQUE,
	is_verified BOOLEAN DEFAULT FALSE,
	password_hash VARCHAR(255) NOT NULL,
	created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE asteroids (
	id VARCHAR(255) PRIMARY KEY,
	name VARCHAR(255) NOT NULL,
	diameter FLOAT,
	velocity FLOAT,
	miss_distance FLOAT,
	is_hazardous BOOLEAN,
);

CREATE TABLE mission_logs (
	id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
	asteroid_id VARCHAR(255) REFERENCES asteroids(id),
	user_id UUID REFERENCES users(id),
	mission_name VARCHAR(255) NOT NULL,
	mission_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	delta_v FLOAT,
);
