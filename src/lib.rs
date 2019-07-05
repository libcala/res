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

use std::io::Write;
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
    let file_dat: String = match String::from_utf8(byte_vec) {
        Ok(v) => v,
        Err(_) => {
            exit!("{} is not UTF-8!", file);
        }
    };

    let r: Result<toml::Value, _> = file_dat.parse();
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
        }
        None => {
            exit!("Couldn't find key {}!", vname);
        }
    }
}

fn get_table(a: &toml::Value, vname: &str) -> toml::Value {
    match a.get(vname) {
        Some(v) => v.clone(),
        None => {
            exit!("Couldn't find key {}!", vname);
        }
    }
}

fn check_exists() {
    if adi_storage::get_exists("res") == false {
        panic!("Folder res/ doesn't exist.");
    }

    if adi_storage::get_exists("res/icon.png") == false {
        panic!("File res/icon.png doesn't exist.");
    }

    if adi_storage::get_exists("res/symbol.png") == false {
        panic!("File res/symbol.svg or res/symbol.png doesn't exist.");
    }
}

/// Generate a `Res` data from folder res.
///
/// # Required Files
/// Folder res must contain `icon.png`, the launcher graphic.
///
/// # File Formats
/// ## Plop Grizzly's File Formats
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
pub fn generate_old() -> () {
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
    save(
        "res/text/en/name.text",
        utem::translate(English, &name).as_bytes(),
    );
    save(
        "res/text/en/description.text",
        utem::translate(English, &description).as_bytes(),
    );

    // Developers Name Never Changes
    save("res/text/xx/developer.text", developer.as_bytes());

    // RUST: Name & Developer
    save("res/src/name.rs", include_bytes!("res/name.rs") as &[u8]);
    save(
        "res/src/developer.rs",
        include_bytes!("res/developer.rs") as &[u8],
    );

    // Create .cargo/config
    adi_storage::save(".cargo/config", include_bytes!("res/config") as &[u8]);

    println!("Done!");
}

fn save(filename: &str, content: &[u8]) {
    let mut filename2 = std::env::var("OUT_DIR").unwrap();
    filename2.push('/');
    filename2.push_str(filename);

    let mut file = std::fs::File::create(filename2).unwrap();
    file.write_all(content).unwrap();
}

#[must_use]
pub struct ShaderBuilder {
    name: String,
    transform: u8,
    group: u8,
    tint: bool,
    gradient: bool,
    depth: bool,
    blend: bool,
    instances_num: u16,
}

impl ShaderBuilder {
    /// Create a new `ShaderBuilder`.
    fn new(name: &str) -> ShaderBuilder {
        ShaderBuilder {
            name: name.to_string(),
            transform: 0,
            group: 0,
            tint: false,
            gradient: false,
            depth: false,
            blend: false,
            instances_num: 1,
        }
    }

    /// Add a unique transform to the shader.
    pub const fn transform(mut self) -> Self {
        self.transform += 1;
        self
    }

    /// Add a group transform to the shader.
    pub const fn group(mut self) -> Self {
        self.group += 1;
        self
    }

    /// Add a tint to the shader.
    pub fn tint(mut self) -> Self {
        assert_eq!(self.tint, false);
        self.tint = true;
        self
    }

    /// Add a gradient (vertex-specific tint) to the shader.
    pub fn gradient(mut self) -> Self {
        assert_eq!(self.gradient, false);
        self.gradient = true;
        self
    }

    /// Add depth to the shader (Z coordinate for vertices).
    pub fn depth(mut self) -> Self {
        assert_eq!(self.depth, false);
        self.depth = true;
        self
    }

    /// Add transparency blending to the shader.
    pub fn blend(mut self) -> Self {
        assert_eq!(self.blend, false);
        self.blend = true;
        self
    }

    /// Set the number of instances.
    pub fn num_instances(mut self, n: u16) -> Self {
        self.instances_num = n;
        self
    }

    // Generate the shader.
    fn gen(&self) {
        // Convert a number to text.
        fn num_to_text(l: u8) -> [u8; 2] {
            if l >= 128 {
                panic!("Number too high");
            }

            let a = (l >> 4) + b'a';
            let b = (l << 4) + b'a';

            [a, b]
        }

        //

        let mut opengl_frag = "precision mediump float;\n".to_string();
        if self.gradient {
            opengl_frag.push_str("varying vec4 v_gradient;\n");
        }
        opengl_frag.push_str("void main() {\ngl_FragColor = ");
        if self.gradient {
            opengl_frag.push_str("v_gradient * ");
        } else {
            // Fallback color
            opengl_frag.push_str("vec4(1.0, 1.0, 1.0, 1.0) * ");
        }
        opengl_frag.pop();
        opengl_frag.pop();
        opengl_frag.pop();
        opengl_frag.push_str(";\n}\\0");

        //

        let mut opengl_vert = "attribute vec4 pos;uniform int cala_InstanceID;\n".to_string();
        for i in 0..self.transform {
            let ntt = num_to_text(i);
            let ntt = [ntt[0] as char, ntt[1] as char];
            opengl_vert.push_str(&format!("uniform mat4 transform_{}{}[{}];\n", ntt[0], ntt[1], self.instances_num));
        }
        if self.gradient {
            opengl_vert.push_str("varying vec4 v_gradient;\nattribute vec4 col;\n");
        }
        opengl_vert.push_str("void main() {\ngl_Position = ");
        for i in 0..self.transform {
            let ntt = num_to_text(i);
            let ntt = [ntt[0] as char, ntt[1] as char];
            opengl_vert.push_str(&format!("transform_{}{}[cala_InstanceID] * ", ntt[0], ntt[1]));
        }
        opengl_vert.push_str("pos;\n");
        if self.gradient {
            opengl_vert.push_str("v_gradient = col;\n");
        }
        opengl_vert.push_str("}\\0");

        save(&format!("res/{}.rs", self.name), format!("ShaderBuilder {{transform:{},group:{},tint:{},gradient:{},depth:{},blend:{},opengl_frag:\"{}\",opengl_vert:\"{}\",instance_count:{}}}", self.transform, self.group, self.tint, self.gradient, self.depth, self.blend, opengl_frag, opengl_vert, self.instances_num).as_bytes());
    }
}

/// Generate shader resources.
pub fn shader(name: &str) -> ShaderBuilder {
    ShaderBuilder::new(name)
}

/// Generate
pub fn generate(shader_builders: &[ShaderBuilder]) {
    let mut filename2 = std::env::var("OUT_DIR").unwrap();
    filename2.push('/');
    filename2.push_str("res");
    std::fs::create_dir_all(filename2).unwrap();

    for shader_builder in shader_builders.iter() {
        shader_builder.gen();
    }
}
