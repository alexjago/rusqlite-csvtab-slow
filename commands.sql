.mode csv
.import stop_times.txt StopTimes
.import trips.txt Trips

CREATE VIEW TripSeqs AS
SELECT StopTimes.stop_id, StopTimes.stop_sequence, Trips.direction_id, Trips.route_id
FROM StopTimes
INNER JOIN Trips on StopTimes.trip_id = Trips.trip_id;

SELECT count(*) FROM StopTimes WHERE stop_id IS '893';

SELECT count(*) FROM TripSeqs WHERE stop_id IS '893';

SELECT count(*) FROM TripSeqs WHERE stop_id IS '313178';


