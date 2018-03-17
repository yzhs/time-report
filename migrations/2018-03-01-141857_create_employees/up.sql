CREATE TABLE employees (
	id integer PRIMARY KEY AUTOINCREMENT NOT NULL,
	name varchar NOT NULL UNIQUE,
	name_sort varchar NOT NULL UNIQUE
)
