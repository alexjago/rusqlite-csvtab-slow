# rusqlite-csvtab-slow
A minimal demonstration of a performance caveat with Rusqlite's CSV tables feature. 

# Summary

If you're doing, for example, multi-table JOINs, you need to be sure to **load your tables into memory first.** This is a separate step from setting up the virtual table. 

`CREATE VIRTUAL TABLE foobar_v USING csv(...);` needs to be followed by `CREATE TABLE foobar AS SELECT * FROM foobar_v;`.


# Setup

Extract the two files from `csvs.zip` and place them at the project root. 

# Performance differences

	> time sqlite3 < commands.sql

About 12-13 seconds on my machine, to run three queries. (SQLite `.import` does the memory load.) Almost all the time is spent waiting for the first query to print. 

On my machine, running query #2 (a multi-table JOIN query with about 3600 rows involved) takes the better part of three minutes... when the file is on disk (a decently fast SSD). 

Running the queries with the tables fully in memory takes almost no time at all. 
