// Res
// Copyright (c) 2017-2019 Jeron Aldaron Lau <jeronlau@plopgrizzly.com>
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

mod sheet;

// extern crate adi_storage;
// extern crate toml;

use std::io::Write;
use std::path::Path;

// use std::process;

/* macro_rules! exit {
	($ ( $ arg : tt ) *) => { {
		println!($($arg)*);
		process::exit(1);
	} };
} */

/*fn read(file: &str) -> toml::Value {
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
}*/

/*/// Generate a `Res` data from folder res.
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
}*/

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
    tint: bool,
    gradient: bool,
    graphic: bool,
    depth: bool,
    blend: bool,
}

impl ShaderBuilder {
    /// Create a new `ShaderBuilder`.
    fn new(name: &str) -> ShaderBuilder {
        ShaderBuilder {
            name: name.to_string(),
            transform: 0,
            tint: false,
            gradient: false,
            graphic: false,
            depth: false,
            blend: false,
        }
    }

    /// Add a unique transform to the shader.
    pub const fn transform(mut self) -> Self {
        self.transform += 1;
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

    /// Add a graphic to the shader.
    pub fn graphic(mut self) -> Self {
        assert_eq!(self.graphic, false);
        self.graphic = true;
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

    // Generate the shader.
    fn gen(&self) {
        let mut opengl_frag = "precision mediump float;\n".to_string();
        if self.tint {
            opengl_frag.push_str("uniform vec4 tint;\n");
        }
        if self.graphic {
            opengl_frag.push_str("uniform sampler2D tex;");
            opengl_frag.push_str("varying vec2 texcoord;\n");
        }
        if self.gradient {
            opengl_frag.push_str("varying vec4 v_gradient;\n");
        }
        opengl_frag.push_str("void main() {\n    gl_FragColor = ");
        if self.gradient && self.graphic {
            if self.tint {
                opengl_frag.push_str("tint * ");
            }
            opengl_frag.push_str("v_gradient * texture2D(tex, texcoord)");
        } else if self.gradient {
            if self.tint {
                opengl_frag.push_str("tint * ");
            }
            opengl_frag.push_str("v_gradient");
        } else if self.graphic {
            if self.tint {
                opengl_frag.push_str("tint * ");
            }
            opengl_frag.push_str("texture2D(tex, texcoord)");
        } else {
            if self.tint {
                opengl_frag.push_str("tint");
            } else {
                // Fallback color
                opengl_frag.push_str("vec4(1.0, 1.0, 1.0, 1.0)");
            }
        }
        opengl_frag.push_str(";\n}\\0");

        //

        let mut opengl_vert = "".to_string();
        if self.depth {
            opengl_vert.push_str("attribute vec3 pos;\n");
            opengl_vert.push_str("uniform mat4 cam;\n");
        } else {
            opengl_vert.push_str("attribute vec2 pos;\n");
        }
        if self.graphic {
            opengl_vert.push_str("varying vec2 texcoord;\nattribute vec2 texpos;\n");
        }
        if self.gradient {
            opengl_vert.push_str("varying vec4 v_gradient;\nattribute vec4 col;\n");
        }
        opengl_vert.push_str("void main() {\n");
        if self.gradient {
            opengl_vert.push_str("v_gradient = col;\n");
        }
        if self.graphic {
            opengl_vert.push_str("texcoord = texpos;\n");
        }
        opengl_vert.push_str("gl_Position = ");
        if self.depth {
            opengl_vert.push_str("cam * ");
        }
        if self.depth {
            opengl_vert.push_str("vec4(pos, 1.0);\n");
        } else {
            opengl_vert.push_str("vec4(pos, 0.0, 1.0);\n");            
        }
        opengl_vert.push_str("}\\0");

        save(&format!("res/{}.rs", self.name), format!("ShaderBuilder {{tint:{},gradient:{},graphic:{},depth:{},blend:{},opengl_frag:\"{}\",opengl_vert:\"{}\"}}", self.tint, self.gradient, self.graphic, self.depth, self.blend, opengl_frag, opengl_vert).as_bytes());
    }
}

/// Generate shader resources.
pub fn shader(name: &str) -> ShaderBuilder {
    ShaderBuilder::new(name)
}

/// Generate code for including resources in your project.
///
/// Call this function in your `build.rs`, and in your crate's root add this:
/// ```
/// mod res { include!(concat!(env!("OUT_DIR"), "/res.rs")); }
/// ```
/// ... to create a `res` module that contains all of your resources.
/// 
/// # Where do I put my resources?
/// Resources go in `/res/` under the crate's root directory.  Each type of file
/// has it's own folder within `/res/`.
/// - `/res/texture/` - PNG textures files.
/// - `/res/shader/` - MuON shader files.
///
/// # Where do I find my resources?
/// `res/texture/image.png`
///
/// ```
/// // Get image data for image.png
/// use crate::res::texture;
///
/// texture::image
/// ```
pub fn generate(shader_builders: &[ShaderBuilder]) {
    println!("cargo:rerun-if-changed=build.rs");

    let mut filename2 = std::env::var("OUT_DIR").unwrap();
    filename2.push_str("/res");
    std::fs::create_dir_all(filename2).unwrap();

    for shader_builder in shader_builders.iter() {
        shader_builder.gen();
    }

    let mut output = "".to_string();

    // Check for textures, if they exist make a texture sheet.
    if Path::new("./res/texture/").exists() {
        output.push_str(&sheet::write());
    }

    // Create `res.rs` module.
    let mut filename = std::env::var("OUT_DIR").unwrap();
    filename.push_str("/res.rs");
    std::fs::write(filename, output).unwrap();
}
