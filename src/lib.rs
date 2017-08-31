// Res
// Copyright (c) 2017 Plop Grizzly, Jeron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/lib.rs

//! Res is a build-script dependency developed by Plop Grizzly for managing your
//! project's resources.

#![doc(
	html_logo_url = "https://raw.githubusercontent.com/plopgrizzly\
		/res/master/res/icon.png",
	html_favicon_url = "https://raw.githubusercontent.com/plopgrizzly\
		/res/master/res/symbol.png",
	html_root_url = "http://plopgrizzly.com/res/"
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

fn get_table(a: &toml::Value, vname: &str) -> toml::Value {
	match a.get(vname) {
		Some(v) => {
			v.clone()
		},
		None => {
			exit!("Couldn't find key {}!", vname);
		},
	}
}

fn check_exists() {
	if adi_storage::get_exists("res") == false {
		panic!("Folder res/ doesn't exist.");
	}

	if adi_storage::get_exists("res/icon.png") == false {
		panic!("File res/icon.png doesn't exist.");
	}

	if adi_storage::get_exists("res/symbol.png") == false
	{
		panic!("File res/symbol.svg or res/symbol.png doesn't exist.");
	}
}

/// Generate a `Res` data from folder res.
/// 
/// # Required Files
/// Folder res must contain `icon.png`, the launcher graphic.
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
	check_exists();

	let value = read("Cargo.toml");

	let package = get_table(&value, "package");
	let metadata = get_table(&package, "metadata");
	let res = get_table(&metadata, "res");

	println!("{:?}", res);

	let developer = get(&res, "developer");
	let name = get(&res, "name");
	let description = get(&res, "description");

	// Name, and Description for each language.
	adi_storage::save("target/res/text/en/name.text",
		utem::translate(English, &name));
	adi_storage::save("target/res/text/en/description.text",
		utem::translate(English, &description));

	// Developers Name Never Changes
	adi_storage::save("target/res/text/xx/developer.text", &developer);

	// RUST: Name & Developer
	adi_storage::save("target/res/src/name.rs",
		include_bytes!("res/name.rs") as &[u8]);
	adi_storage::save("target/res/src/developer.rs",
		include_bytes!("res/developer.rs") as &[u8]);

	// Create .cargo/config
	adi_storage::save(".cargo/config",
		include_bytes!("res/config") as &[u8]);

	println!("Done!");
}
