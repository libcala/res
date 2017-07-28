// lib.rs
// Res
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE

//! res is a build-script-dependency for generating resources.

#![doc(
	html_logo_url =
		"https://rawgit.com/aldarons-tech/res/master/res/icon.png",
	html_favicon_url =
		"https://rawgit.com/aldarons-tech/res/master/res/symbol.png",
	html_root_url = "http://at.plopgrizzly.tech/res/"
)]

extern crate adi_storage;
extern crate toml;
extern crate utem;

use std::process;
use utem::Language::*;

macro_rules! exit {
	($ ( $ arg : tt ) *) => { {
		println!($($arg)*);
		process::exit(1);
	} };
}

fn read(file: &str) -> toml::Value {
	let byte_vec = adi_storage::load(file);
	let file_dat : String = match String::from_utf8(byte_vec) {
		Ok(v) => v,
		Err(_) => {
			exit!("{} is not UTF-8!", file);
		}
	};

	let r : Result<toml::Value, _> = file_dat.parse();
	match r {
		Ok(v) => v,
		Err(e) => {
			println!("{}", e);
			exit!("{} is not TOML!", file);
		}
	}
}

fn get(a: &toml::Value, vname: &str) -> String {
	match a.get(vname) {
		Some(v) => {
			if let toml::Value::String(v) = v.clone() {
				v
			} else {
				exit!("{} is not a string!", vname)
			}
		},
		None => {
			exit!("Couldn't find key {}!", vname);
		},
	}
}

/// Generate a `Res` data from folder res.
/// 
/// # Required Files
/// Folder res should contain the following files:
///
/// * `icon.png` - This is the launcher graphic.
/// * `symbol.svg` - This is a simplified version of icon, which shows while the
///	program is running.
/// * `details.toml` - A toml file containing the name of the program, comment,
///	and description in ran-slat format.
///
/// # File Formats
/// ## Aldaron's Tech File Formats
/// * `.av3d` animated vector in 3 dimensions
/// * `.av2d` animated vector in 2 dimensions
/// * `.sv3d` static vector in 3 dimensions
/// * `.sv2d` static vector in 2 dimensions
/// * `.utem` universal text encoding as meaning
/// * `.slat` ran-slat text file
/// * `.font` a font format
/// * `.synþ` a synthesizer format
/// * `.data` a format for storing application data
///
/// ## Standard File Formats
/// * `.png` a bitmap format for graphics
/// * `.jpg` a bitmap format for pictures
/// * `.opus` a lossy audio format ( compressed )
/// * `.wav` a lossless audio format ( uncompressed )
/// * `.apng` an animated `PNG`
/// * `.mp4` a video
///
/// The following are optional folders which you can add resources to.
///
/// # Graphics
/// * `bitmap/` - Contains `PNG`s (extension: `.png`)
/// * `photo/` - Contains `JPG`s (extension: `.jpg`)
/// * `vector/` - Contains `SV2D`s (extension: `.sv2d`)
/// * `model/` - Contains `SV3D`s (extension: `.sv3d`)
///
/// # Animations
/// * `bitmap_anim/` - Contains `APNG`s (extension: `.apng`)
/// * `photo_anim/` - Contains `H.264 Video`s (extension `mp4`)
/// * `vector_anim/` - Contains `AV2D`s (extension: `.av2d`)
/// * `model_anim/` - Contains `AV3D`s (extension: `av3d`)
///
/// # Audio
/// * `audio/` - Contains `OGG OPUS`s (extension: `.opus`)
/// * `sample/` - Contains `WAV`s (extension: `.wav`)
/// * `synth/` - Contains `ALDARONS TECH SYNTH FILE`s (extension: `.synþ`)
///
/// # Text
/// * `slat/` - Contains `RAN-SLAT TEXT`s (extension: `.slat`)
/// * `text/` - Contains `UTEM TEXT`s (extension: `.utem`)
/// * `font/` - Contains `FONT`s (extension: `.font`)
///
/// # Miscellaneous
/// * `data/` - Contains `DATA`s (extension: `.data`)
pub fn generate() -> () {
	if adi_storage::get_exists("res") == false {
		panic!("Folder res/ doesn't exist.");
	}

	if adi_storage::get_exists("res/details.toml") == false {
		panic!("File res/details.toml doesn't exist.");
	}

	if adi_storage::get_exists("res/icon.png") == false {
		panic!("File res/details.toml doesn't exist.");
	}

	if adi_storage::get_exists("res/symbol.svg") == false {
		panic!("File res/details.toml doesn't exist.");
	}

	let value = read("res/details.toml");

	let name = get(&value, "name");
	let comment = get(&value, "comment");
	let description = get(&value, "description");

	adi_storage::save("target/res/text/en/name.text",
		utem::translate(English, &name));
	adi_storage::save("target/res/text/en/comment.text",
		utem::translate(English, &comment));
	adi_storage::save("target/res/text/en/description.text",
		utem::translate(English, &description));

	adi_storage::save("target/res/src/name.rs",
		include_bytes!("res/name.rs") as &[u8]);

	println!("Done!");
}
