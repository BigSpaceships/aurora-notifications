DELETE FROM event WHERE id IN (
    SELECT id FROM (
        SELECT id, ROW_NUMBER() OVER(PARTITION BY serial_num, issue_time ORDER BY id) as rn FROM event
    ) WHERE rn > 1
);
