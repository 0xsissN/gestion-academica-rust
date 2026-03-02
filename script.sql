CREATE DATABASE AcademicManagement;

CREATE TABLE "role" (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) UNIQUE NOT NULL
);

CREATE TABLE "user" (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(50) UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    first_name VARCHAR(100) NOT NULL,
    last_name VARCHAR(100) NOT NULL,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP DEFAULT NOW(),
    role_id INTEGER NOT NULL REFERENCES "role"(id)
);

CREATE TABLE career (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) UNIQUE NOT NULL,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE course (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    is_active BOOLEAN DEFAULT TRUE,
    credits INTEGER NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    career_id INTEGER NOT NULL REFERENCES career(id) ON DELETE CASCADE
);

CREATE TABLE enrollment (
    id SERIAL PRIMARY KEY,
    enrollment_date TIMESTAMP DEFAULT NOW(),
    grade DECIMAL(5,2) DEFAULT 0.00,
    student_id UUID NOT NULL REFERENCES "user"(id) ON DELETE CASCADE,
    course_id INTEGER NOT NULL REFERENCES course(id) ON DELETE CASCADE
);