CREATE TABLE weeks (
	year integer NOT NULL CHECK (
		year >= 2017 AND year <= cast(strftime('%y', 'now') as integer) + 1
	),

	week_of_year integer NOT NULL CHECK (
		0 < week_of_year AND week_of_year <= 52
	),

	type_of_week integer NOT NULL CHECK (0 <= type_of_week AND type_of_week < 4),

	PRIMARY KEY (year, week_of_year)
)
