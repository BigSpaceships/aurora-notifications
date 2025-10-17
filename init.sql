CREATE TYPE alert_type AS ENUM ('warning', 'alert');

CREATE TABLE event (
    issue_time TIMESTAMP NOT NULL,
    serial_num INT NOT NULL,
    serial_num_ext INT,
    id SERIAL PRIMARY KEY,
    id_ext INT REFERENCES event(id) ON DELETE CASCADE
);

CREATE TABLE geomagnetic (
    event_id INT REFERENCES event(id) PRIMARY KEY,
    threshold INT NOT NULL,
    threshold_reached_time TIMESTAMP,
    warning_start_time TIMESTAMP,
    warning_end_time TIMESTAMP
);
