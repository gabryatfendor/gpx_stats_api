# gpx_stats_api
Check for basic stats on any GPX file

## Build
Simply run **cargo run** on the root directory of the project. This will startup the webserver on localhost:8080, you can reach there the endpoint listed below

## Endpoint List

### /upload \[POST\]
Accept a GPX file in input and answers with a list of baic stats (ascent, descent, distance)
