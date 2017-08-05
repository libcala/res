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
/// Folder res should contain the following files:
///
/// * `icon.png` - This is the launcher graphic.
/// * `symbol.svg` or `symbol.png` - This is a simplified version of icon, which
///	shows while the program is running.
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

	let nament = get(&package, "name");

	// Name, and Description for each language.
	adi_storage::save("target/res/text/en/name.text",
		utem::translate(English, &name));
	adi_storage::save("target/res/text/en/description.text",
		utem::translate(English, &description));

	// Developers Name Never Changes
	adi_storage::save("target/res/text/xx/developer.text", &developer);

	adi_storage::save("target/res/src/name.rs",
		include_bytes!("res/name.rs") as &[u8]);
	adi_storage::save("target/res/src/developer.rs",
		include_bytes!("res/developer.rs") as &[u8]);

	// Create target/res/run_linux.sh
	adi_storage::save("target/res/run_linux.sh",
		include_bytes!("res/run_linux.sh") as &[u8]);

	// Install a .desktop for Linux
	{
		let mut desktop_data = format!(
			"[Desktop Entry]\nExec={}\nIcon={}\nType=Application\n",
			&nament, &nament);

		// localize / english
		let en_name = adi_storage::load("target/res/text/en/name.text");

		desktop_data.push_str(&format!("Name[en]={}\n",
			String::from_utf8(en_name).unwrap()));
		// TODO: Dialects
		/*} else {
			program::exit("No English Translation.");
		}*/

		adi_storage::save(
			format!("{}/.local/share/applications/{}.desktop",
				::std::env::home_dir().unwrap().display(), nament),
			desktop_data.as_bytes());
	}

	// Create .cargo/config
	adi_storage::save(".cargo/config",
		include_bytes!("res/config") as &[u8]);

	println!("Done!");
}
