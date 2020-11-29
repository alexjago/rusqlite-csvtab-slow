# rusqlite-csvtab-slow
A minimal demonstration of performance issues with Rusqlite's CSV tables feature

# Setup

Extract the two files from `csvs.zip` and place them at the project root. 

# Comparison with SQLite3 shell

	> time sqlite3 < commands.sql

About 12-13 seconds on my machine. 

	> time ./target/release/rusqlite-csvtab-slow

About 145 seconds on my machine. 

Note that most of the SQLite3 shell time is before the results of the *first* query are printed, suggesting there's some extra processing done on CSV import. 

Conversely, almost all the Rust time is spent waiting for the *second* query. 