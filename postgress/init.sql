CREATE ROLE WRITEONCE_DB_ADMIN WITH LOGIN PASSWORD 'adminpassword';
CREATE DATABASE WRITEONCE_DB OWNER admin;
GRANT ALL PRIVILEGES ON DATABASE WRITEONCE_DB TO admin;