CREATE TABLE items (
	id integer PRIMARY KEY AUTOINCREMENT,
	report_id integer NOT NULL REFERENCES reports(id),
	employee_id integer NOT NULL REFERENCES employees(id),


	start_datetime integer NOT NULL CHECK(
		date(start_datetime) >= '2017-08-01' AND
		time(start_datetime) >= '11:00'
	),
	end_datetime integer NOT NULL CHECK (
		date(start_datetime) = date(end_datetime) AND
		start_datetime <= end_datetime - 15*60 AND
		time(end_datetime) <= '16:00'
	),

	remark varchar
)
