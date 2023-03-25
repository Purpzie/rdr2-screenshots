#![forbid(unsafe_code)]

use std::{
	fs::{self, File, OpenOptions},
	io::{self, BufReader, BufWriter, Seek, SeekFrom, Write},
	path::{Path, PathBuf},
};

fn main() -> anyhow::Result<()> {
	let mut path = match std::env::args_os().nth(1) {
		Some(p) => PathBuf::from(p),
		None => anyhow::bail!("Please specify the path to your RDR2 profile folder."),
	};
	if !path.is_dir() {
		anyhow::bail!("Path must point to a folder.");
	}

	// create required directories if they don't exist
	path.push("screenshots");
	if !path.is_dir() {
		fs::create_dir(&path)?;
	}
	path.pop();
	let mut backup_path = path.clone();
	backup_path.push("screenshots_backup");
	if !backup_path.is_dir() {
		fs::create_dir(&backup_path)?;
	}

	// reuse this in the loop
	let mut open_write = OpenOptions::new();
	open_write.create_new(true).write(true);

	for entry in fs::read_dir(&path)? {
		let entry = entry?;

		// ignore everything that isn't a screenshot
		if !entry.metadata()?.is_file() {
			continue;
		}
		let file_name = entry
			.file_name()
			.into_string()
			.map_err(|s| anyhow::anyhow!("Unable to translate filename to utf8: {s:?}"))?;
		if !file_name.starts_with("PRDR") || Path::new(&file_name).extension().is_some() {
			continue;
		}

		println!("Converting {file_name}");
		path.push("screenshots");
		path.push(&file_name);
		path.set_extension("jpg");
		let mut writer = BufWriter::new(open_write.open(&path)?);
		path.pop();
		path.pop();
		path.push(&file_name);
		let mut reader = BufReader::new(File::open(&path)?);
		reader.seek(SeekFrom::Start(300))?;
		io::copy(&mut reader, &mut writer)?;
		writer.flush()?;

		// move original to backup directory
		backup_path.push(&file_name);
		fs::rename(&path, &backup_path)?;

		// prepare for next loop
		path.pop();
		backup_path.pop();
	}

	println!("Done");
	Ok(())
}
