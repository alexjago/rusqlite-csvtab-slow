use rusqlite::{Connection, Result, NO_PARAMS};
use std::time::{Instant};

fn main() {
    println!("This crate should hopefully demonstrate a performance issue in Rusqlite's csvtab module: multi-table queries must have the tables in memory for performance.");
    println!("Make sure the two files from `csvs.zip`, `stop_times.txt` and `trips.txt`, are in the working directory...");
    println!("First we're going to do three queries with virtual tables (i.e. on-disk).");
    println!("Then we're going to load the tables into memory properly and repeat.");
    let db = Connection::open_in_memory().expect("Could not open in-memory database");

    rusqlite::vtab::csvtab::load_module(&db)
        .expect("Could not load CSV module of virtual database");

    load_gtfs(&db).expect("Failed loading GTFS (are the files there?)");

    println!("This first query should complete quickly");
    query_one(&db, 'v');

    println!("This second query be slow in Rust, but was almost as quick as #1 in the SQLite shell. This is because SQLite loads the file into memory, but we haven't yet. Expect this query to take several minutes...");
    query_slow(&db, 893, 'v');
    
    println!("This third query is like #2 but should be quick, with many fewer rows involved.");
    query_slow(&db, 313178, 'v');
    
    println!("\nNow we'll load the tables into memory and create another view...");
    load_memory(&db).expect("Failed loading into memory");
    
    println!("Query #1 should still be fast");
    query_one(&db, 'm');
    
    println!("Query #2 should now also be fast");
    query_slow(&db, 893, 'm');

    println!("Query #3 should still be fast");
    query_slow(&db, 313178, 'm');    
}

fn load_gtfs(db: &Connection) -> Result<(), rusqlite::Error> {
    //! Loads all the GTFS CSVs into SQLite tables in `db`

    for (t, p) in [("StopTimesV", "stop_times.txt"), ("TripsV", "trips.txt")].iter() {
        let schema = format!(
            "CREATE VIRTUAL TABLE {} USING csv(filename='{}', header=YES)",
            &t, &p
        );

        println!("{}\n", schema);

        db.execute_batch(&schema)?;
    }

    let q = "CREATE VIEW TripSeqsV AS
SELECT StopTimesV.stop_id, StopTimesV.stop_sequence, TripsV.direction_id, TripsV.route_id
FROM StopTimesV
INNER JOIN TripsV on StopTimesV.trip_id = TripsV.trip_id;";
    
    println!("{}\n", q);
    
    db.execute_batch(q)?;

    Ok(())
}

fn query_one(db: &Connection, v_or_m: char) {
    //! Execute the first, fast query on a single table
    
    let q = format!("SELECT count(*) FROM StopTimes{} WHERE stop_id IS '893';", v_or_m);
    println!("{}", q);
    
    let now = Instant::now();
    
    let mut stmt = db
        .prepare(&q)
        .unwrap();
    let rows = stmt
        .query_map(NO_PARAMS, |r| r.get(0))
        .expect("Failed query");

    for r in rows {
        let t: i64 = r.unwrap();
        println!("{:?}", t);
    }
    
    println!("Completed in {} seconds.\n", now.elapsed().as_secs());
}

fn query_slow(db: &Connection, stop_id: usize, v_or_m: char) {
    //! Execute the INNER JOIN query

    let q = format!("SELECT count(*) FROM TripSeqs{} WHERE stop_id IS '{}';", v_or_m, stop_id);
    println!("{}", q);

    let now = Instant::now();

    let mut stmt = db
        .prepare(&q)
        .unwrap();
    let mut rows = stmt.query(NO_PARAMS).expect("Failed query");

    println!("Prepared statement, called query() ...");
    
    // iterators are lazy, so calling next() like this demonstrates the slowness

    let r: i64 = rows.next().unwrap().unwrap().get_unwrap(0);

    println!("{:?}", r);
    
    println!("Completed in {} seconds.\n", now.elapsed().as_secs());

}  

fn load_memory(db: &Connection) -> Result<(), rusqlite::Error> {
    println!("Loading the tables into memory...");
    
    for t in ["StopTimes", "Trips"].iter() {
        let schema = format!(
            "CREATE TABLE {}M AS SELECT * FROM {}V;",
            &t, &t
        );

        println!("{}\n", schema);

        let now = Instant::now();
        db.execute_batch(&schema)?;
        println!("Loaded {} in {} seconds.\n", &t, now.elapsed().as_secs());
    }

    let q = "CREATE VIEW TripSeqsM AS
SELECT StopTimesM.stop_id, StopTimesM.stop_sequence, TripsM.direction_id, TripsM.route_id
FROM StopTimesM
INNER JOIN TripsM on StopTimesM.trip_id = TripsM.trip_id;";
    
    println!("{}\n", q);
    
    db.execute_batch(q)?;

    Ok(())
}

