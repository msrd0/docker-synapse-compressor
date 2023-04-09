# docker-synapse-compressor

This is a docker image that allows for automated background compression of your synapse database. Basically, it is https://github.com/matrix-org/rust-synapse-compress-state but in a way that makes it suitable for docker-compose setups, i.e. using environment variables for configuration, and not relying on external cron-like daemons.

## Configuration

The following is a list of all environment variables. Those marked with a <sup><b>*</b></sup> are required. The documentation strings are directly taken from https://github.com/matrix-org/rust-synapse-compress-state.

 - `POSTGRES_URL`<sup><b>*</b></sup>: The configruation for connecting to the postgres database.
   
   The configuration for connecting to the Postgres database. This should be of the form 
   postgresql://username:password@mydomain.com/database" or a key-value pair 
   string: "user=username password=password dbname=database host=mydomain.com" 
   See https://docs.rs/tokio-postgres/0.7.2/tokio_postgres/config/struct.Config.html 
   for the full details.

 - `CHUNK_SIZE`<sup><b>*</b></sup>: The maximum number of state groups to load into memroy at once.

   The number of state_groups to work on at once. All of the entries
   from state_groups_state are requested from the database
   for state groups that are worked on. Therefore small
   chunk sizes may be needed on machines with low memory.
   (Note: if the compressor fails to find space savings on the
   chunk as a whole (which may well happen in rooms with lots
   of backfill in) then the entire chunk is skipped.)

 - `DEFAULT_LEVELS`: Sizes of each new level in the compression algorithm, as a comma separated list.

   The first entry in the list is for the lowest, most granular level,
   with each subsequent entry being for the next highest level.
   The number of entries in the list determines the number of levels
   that will be used.

   The sum of the sizes of the levels effect the performance of fetching the state
   from the database, as the sum of the sizes is the upper bound on number of
   iterations needed to fetch a given set of state.

 - `NUMBER_OF_CHUNKS`: The number of chunks to compress.

   This many chunks of the database will be compressed. The higher this number is set to,
   the longer the compressor will run for.

 - `RUST_LOG`: Log level
