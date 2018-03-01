CREATE TABLE reports (
	id integer PRIMARY KEY AUTOINCREMENT,
	title varchar NOT NULL CHECK (length(title) > 7),
	start_date integer NOT NULL,
	end_date integer NOT NULL CHECK (
		start_date <= end_date AND
		end_date <= cast(strftime('%s', 'now') as integer) + 7*24*60*60
	),
	was_pdf_generated boolean NOT NULL DEFAULT false
)
