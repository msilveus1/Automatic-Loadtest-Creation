CREATE DATABASE IF NOT EXISTS Loadtest_Automation;
USE Loadtest_Automation;

CREATE TABLE Load_Test_Servers (
    server_ip INT UNSIGNED,
    server_name VARCHAR(128),
    server_type VARCHAR(64),
    last_connection_date DATETIME,
    first_connection_date DATETIME,
);

CREATE TABLE Load_Test_Definition(
    test_name VARCHAR(64),
    line_number INT UNSIGNED,
    origin_file_path VARCHAR(MAX),
    origin_repository VARCHAR(MAX),
    data_type VARCHAR(16),
    data_source VARCHAR(MAX),
    FOREIGN KEY (test_server_ip) REFERENCES Servers(server_ip);
);

