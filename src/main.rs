use rusqlite::{params, Connection, Result, NO_PARAMS};

fn main() {
    println!("This crate should hopefully demonstrate a performance issue in Rusqlite's csvtab module...");
    println!("Make sure the two files from `csvs.zip`, `stop_times.txt` and `trips.txt`, are in the working directory...");
    let db = Connection::open_in_memory().expect("Could not open virtual database");

    rusqlite::vtab::csvtab::load_module(&db)
        .expect("Could not load CSV module of virtual database");

    load_gtfs(&db).expect("Failed loading GTFS (are the files there?)");

    println!("This first query should complete quickly");
    query_one(&db);

    println!("\nThis second query should not complete quickly in Rust, but took the same time in the SQLite shell.");
    query_two(&db);
    
}

fn load_gtfs(db: &Connection) -> Result<(), rusqlite::Error> {
    //! Loads all the GTFS CSVs into SQLite tables in `db`

    for (t, p) in [("StopTimes", "stop_times.txt"), ("Trips", "trips.txt")].iter() {
        let schema = format!(
            "CREATE VIRTUAL TABLE {} USING csv(filename='{}', header=YES)",
            &t, &p
        );

        eprintln!("{}\n", schema);

        db.execute_batch(&schema)?;
    }

    let q = "CREATE VIEW TripSeqs AS
SELECT StopTimes.stop_id, StopTimes.stop_sequence, Trips.direction_id, Trips.route_id
FROM StopTimes
INNER JOIN Trips on StopTimes.trip_id = Trips.trip_id;";
    
    eprintln!("{}\n", q);
    
    db.execute_batch(q)?;

    Ok(())
}

fn query_one(db: &Connection) {
    //! Execute the first, fast query on a single table
    
    let q = "SELECT count(*) FROM StopTimes WHERE stop_id IS '893';";
    eprintln!("{}", q);
    
    let mut stmt = db
        .prepare(q)
        .unwrap();
    let rows = stmt
        .query_map(NO_PARAMS, |r| r.get(0))
        .expect("Failed query");

    for r in rows {
        let t: i64 = r.unwrap();
        println!("{:?}", t);
    }
}

fn query_two(db: &Connection) {
    //! Execute the second, slow query on an INNER JOIN view

    let q = "SELECT count(*) FROM TripSeqs WHERE stop_id IS '893'";
    eprintln!("{}", q);

    let mut stmt = db
        .prepare(q)
        .unwrap();
    let mut rows = stmt.query(NO_PARAMS).expect("Failed query");

    println!("Prepared statement, called query(), slowdown imminent");
    
    // iterators are lazy, so calling next() like this demonstrates

    let r: i64 = rows.next().unwrap().unwrap().get_unwrap(0);

    println!("{:?}", r);
}  