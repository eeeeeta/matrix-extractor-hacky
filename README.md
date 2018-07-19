Very Hacky Matrix Extractor
===========================

Only exposed to the public because someone in `#matrix:matrix.org` wanted it.
This code is...not great (but it might work). No guarantees are provided.

## Usage instructions

1. Get a synapse database dump (in SQL format).
2. Through arcane magic with sed or otherwise, extract the content of the `event_json` table - i.e. the bit between `COPY...` and the last line (which is a backslash or something, idk).
3. Filter that down to the room ID you want (i.e. via `grep 'ROOM_ID'`). See, told you this was hacky.
4. Pipe that extract into this thing's stdin (i.e. `cargo run < data.txt`).
5. It should vomit out a timeline to stdout.

## License

Whatever you want. Let's say CC0, just to make the lawyers happy.
