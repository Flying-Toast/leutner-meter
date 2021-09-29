CREATE TABLE votes (
	id INTEGER PRIMARY KEY NOT NULL,
	meal_id INTEGER NOT NULL,
	-- case ID (e.g. abc123) of the voter
	voter_caseid TEXT NOT NULL,
	-- the rating (0 to 10)
	score INTEGER NOT NULL,
	FOREIGN KEY (meal_id) REFERENCES meals(id)
);

CREATE TABLE meals (
	id INTEGER PRIMARY KEY NOT NULL,
	year INTEGER NOT NULL,
	month INTEGER NOT NULL,
	day INTEGER NOT NULL,
	-- 0 = breakfast, 1 = brunch, 2 = lunch, 3 = dinner
	meal_period INTEGER NOT NULL
);
