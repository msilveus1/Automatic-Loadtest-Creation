CREATE DATABASE IF NOT EXISTS Loadtest_Automation;
USE Loadtest_Automation;

CREATE TABLE Load_Test_Definition(
    config_id BINARY(16) 
    data_type VARCHAR(16),
    data_source VARCHAR(MAX)
);

CREATE TABLE DB_Version(
    version_number INT
)

INSERT INTO DB_Version (version_number) VALUES(0)
