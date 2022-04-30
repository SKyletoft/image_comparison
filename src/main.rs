use std::{fs, path::PathBuf};

use anyhow::Result;
use image::{ImageBuffer, ImageFormat, Rgb};
use rayon::{
	iter::{IndexedParallelIterator, ParallelIterator},
	slice::ParallelSlice,
};

#[inline(never)]
fn compare(file1: &PathBuf, file2: &PathBuf, file3: &PathBuf) -> Result<()> {
	let img1 = image::open(file1)?.into_rgb8();
	let img2 = image::open(file2)?.into_rgb8();

	let (h1, w1) = img1.dimensions();
	let (h2, w2) = img2.dimensions();

	let v1 = img1.as_raw();
	let v2 = img2.as_raw();

	assert_eq!(h1, h2);
	assert_eq!(w1, w2);

	let res_vec = v1
		.iter()
		.zip(v2.iter())
		.map(|(&x, &y)| (x.max(y) - x.min(y)) * 16)
		.collect::<Vec<_>>();
	let res_img = ImageBuffer::<Rgb<u8>, Vec<_>>::from_vec(h1, w1, res_vec).unwrap();
	res_img.save_with_format(file3, ImageFormat::Jpeg)?;
	Ok(())
}

fn main() -> Result<()> {
	// ignore result as it will error if path already exists
	let _ = fs::create_dir("Result");
	let mut files = fs::read_dir("ExamplePicts")?
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
		.windows(2)
		//.par_windows(2)
		.enumerate()
		.all(|(index, arr)| {
			let left = &arr[0];
			let right = &arr[1];
			let res = compare(left, right, &PathBuf::from(format!("Result/{index}.jpg")));
			if let Err(e) = res {
				dbg!(e);
				false
			} else {
				true
			}
		});

	Ok(())
}
