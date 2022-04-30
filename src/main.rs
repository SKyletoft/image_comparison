use std::{fs, path::PathBuf};

use anyhow::Result;
use rayon::{
	iter::{IndexedParallelIterator, ParallelIterator},
	slice::ParallelSlice,
};
use clap::Parser;

#[derive(Debug, Clone, PartialEq, Parser)]
struct Options {
	directory: String,
	cutoff: f64,
	#[clap(long, short)]
	debug: bool
}

fn compare(file1: &PathBuf, file2: &PathBuf, file3: &PathBuf, cutoff: f64, debug: bool) -> Result<()> {
	let img1 = image::open(file1)?.into_rgb8();
	let img2 = image::open(file2)?.into_rgb8();

	let (h1, w1) = img1.dimensions();
	let (h2, w2) = img2.dimensions();

	let v1 = img1.as_raw();
	let v2 = img2.as_raw();

	assert_eq!(h1, h2);
	assert_eq!(w1, w2);

	let total_diff = v1
		.iter()
		.zip(v2.iter())
		.map(|(&x, &y)| (x.max(y) - x.min(y)) as u64)
		.sum::<u64>();
	let percentage_diff = (total_diff * 100) as f64 / (256 * v1.len()) as f64;
	if debug {
		eprintln!("{file1:?}, {file2:?}: {percentage_diff}%");
	}
	if percentage_diff > cutoff {
		fs::copy(file2, file3)?;
	}
	Ok(())
}

fn main() -> Result<()> {
	let options = Options::parse();
	println!("{options:#?}");
	// ignore result as it will error if path already exists
	let _ = fs::create_dir("Result");
	let mut files = fs::read_dir(options.directory)?
		.filter_map(|entry| {
			let entry = entry.ok()?;
			if entry.metadata().ok()?.is_file() {
				Some(entry.path())
			} else {
				None
			}
		})
		.collect::<Vec<_>>();
	files.sort_unstable();
	files
		.par_windows(2)
		//.windows(2)
		.enumerate()
		.all(|(index, arr)| {
			let left = &arr[0];
			let right = &arr[1];
			let res = compare(left, right, &PathBuf::from(format!("Result/{index}.jpg")), options.cutoff, options.debug);
			if let Err(e) = res {
				dbg!(e);
				false
			} else {
				true
			}
		});

	Ok(())
}
