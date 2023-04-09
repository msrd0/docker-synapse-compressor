use anyhow::{anyhow, Context as _};
use clokwerk::{Job, Scheduler, TimeUnits as _};
use log::{error, info};
use std::{env, fmt::Display, str::FromStr, thread, time::Duration};
use synapse_auto_compressor::{manager, state_saving, LevelInfo};

/// Get a required environment variable.
fn env(name: &str) -> anyhow::Result<String> {
	env::var(name).map_err(|_| anyhow!("Missing required environment variable {name}"))
}

/// Get an optional environment variable, or its default value.
fn env_or<D: Into<String>>(name: &str, default: D) -> String {
	env::var(name).unwrap_or_else(|_| default.into())
}

fn env_parse_impl<T>(name: &str, env: anyhow::Result<String>) -> anyhow::Result<T>
where
	T: FromStr,
	<T as FromStr>::Err: Display
{
	env.and_then(|value| {
		value.parse().map_err(|err| {
			anyhow!("Environment variable {name} has an invalid value: {err}")
		})
	})
}

/// Get a required environment variable and parse its value.
fn env_parse<T>(name: &str) -> anyhow::Result<T>
where
	T: FromStr,
	<T as FromStr>::Err: Display
{
	env_parse_impl(name, env(name))
}

/// Get an optional environment variable, or its default value.
fn env_parse_or<D, T>(name: &str, default: D) -> anyhow::Result<T>
where
	D: Into<String>,
	T: FromStr,
	<T as FromStr>::Err: Display
{
	env_parse_impl(name, Ok(env_or(name, default)))
}

fn run(
	postgres_url: &str,
	chunk_size: i64,
	default_levels: &LevelInfo,
	number_of_chunks: i64
) -> anyhow::Result<()> {
	info!("Starting compression");

	// do all the stuff that synapse_auto_compressor does in its main function

	let mut client = state_saving::connect_to_database(postgres_url)
		.context("Error connecting to database")?;
	state_saving::create_tables_if_needed(&mut client)
		.context("Error creating tables in database")?;

	manager::compress_chunks_of_database(
		postgres_url,
		chunk_size,
		&default_levels.0,
		number_of_chunks
	)?;

	info!("Compression finished");
	Ok(())
}

fn main() -> anyhow::Result<()> {
	env_logger::init();

	let postgres_url = env("POSTGRES_URL")?;
	let chunk_size = env_parse("CHUNK_SIZE")?;
	let default_levels: LevelInfo = env_parse_or("DEFAULT_LEVELS", "100,50,25")?;
	let number_of_chunks = env_parse("NUMBER_OF_CHUNKS")?;

	let mut scheduler = Scheduler::new();
	scheduler.every(1.day()).at("05:00").run(move || {
		match run(&postgres_url, chunk_size, &default_levels, number_of_chunks) {
			Ok(()) => {},
			Err(err) => {
				error!("Failed to compress: {err:?}");
			}
		}
	});
	loop {
		scheduler.run_pending();
		thread::sleep(Duration::from_secs(30));
	}
}
