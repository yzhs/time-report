CREATE VIEW items_view AS
SELECT items.id AS id, employees.name AS name,
	date(start_datetime) AS day, type_of_week,
	time(start_datetime) AS start, time(end_datetime) AS stop,
	remark
FROM items
JOIN employees ON items.employee_id = employees.id
JOIN weeks ON
	cast(strftime('%y', start_datetime) AS integer) = weeks.year AND
	cast(strftime('%W', start_datetime) AS integer) = weeks.week_of_year
