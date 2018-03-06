CREATE TABLE reports (
	id integer PRIMARY KEY AUTOINCREMENT NOT NULL,
	title varchar NOT NULL UNIQUE CHECK (length(title) > 7),
	start_date text NOT NULL UNIQUE,
	end_date text NOT NULL UNIQUE CHECK (
		start_date <= end_date AND
		end_date <= strftime('%s', 'now', '-1 week')
	),
	was_pdf_generated boolean NOT NULL DEFAULT false
)
