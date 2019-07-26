//! Generate Texture Sheet.

use png_pong as png;
use sheep;
use heck;

use sheep::{InputSprite, MaxrectsPacker, MaxrectsOptions};
use std::fs::read_dir;
use heck::ShoutySnakeCase;

pub struct Sprite {
    name: String,
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
}

pub struct Named {
    sprites: Vec<Sprite>,
}

impl sheep::Format for Named {
    type Data = Named;
    type Options = Vec<String>;

    fn encode(
        dimensions: (u32, u32),
        in_sprites: &[sheep::SpriteAnchor],
        options: Self::Options
    ) -> Self::Data {
        let (w, h) = dimensions;
        let (w, h) = (w as f32, h as f32);
        let mut sprites = vec![];

        for i in 0..in_sprites.len() {
            let x1 = in_sprites[i].position.0 as f32 / w;
            let y1 = in_sprites[i].position.1 as f32 / h;
            let x2 = in_sprites[i].dimensions.0 as f32 / w;
            let y2 = in_sprites[i].dimensions.1 as f32 / h;
            let name = options[i].to_string();

            sprites.push(Sprite {
                name,
                x1,
                y1,
                x2,
                y2,
            });
        }

        Named {
            sprites,
        }
    }
}

pub fn write() -> String {
    // Find all PNG files.
    let paths = read_dir("./res/texture/").unwrap();
    let mut sprites = vec![];
    let mut names = vec![];

    let mut decoder_builder = png::DecoderBuilder::new();

    for path in paths {
        let path = path.unwrap().path();
        let data = std::fs::read(&path).expect("Failed to open PNG");
        let data = std::io::Cursor::new(data);
        let decoder = decoder_builder.decode_rasters(data);
        let (raster, _nanos) = decoder.last().expect("No frames in PNG").expect("PNG parsing error");
        let dimensions = (raster.width(), raster.height());
        let bytes: &[u8] = raster.as_u8_slice();

        sprites.push(InputSprite {
            dimensions,
            bytes: bytes.to_vec(),
        });

        println!("cargo:rerun-if-changed={}", path.to_str().unwrap());

        names.push(path.file_stem().unwrap().to_str().unwrap().to_string());
    }

    // Set texture sheet size.
    let options = MaxrectsOptions::default()
        .max_width(4096)
        .max_height(4096);

    // Do the actual packing! 4 defines the stride, since we're using rgba8 we
    // have 4 bytes per pixel.
    let results = sheep::pack::<MaxrectsPacker>(sprites, 4, options);

    // MaxrectsPacker always returns a single result. Other packers can return
    // multiple sheets; should they, for example, choose to enforce a maximum
    // texture size per sheet.
    let sprite_sheet = results
        .into_iter()
        .next()
        .expect("Should have returned a spritesheet");

    // Now, we can encode the sprite sheet in a format of our choosing to
    // save things such as offsets, positions of the sprites and so on.
    let meta = sheep::encode::<Named>(&sprite_sheet, names);

    // Next, we save the output to a file using the image crate again.
    let mut filename = std::env::var("OUT_DIR").unwrap();
//    filename.push_str("/res/texture-sheet.png");
    filename.push_str("/res/texture-sheet.pix");

    std::fs::write(filename, &sprite_sheet.bytes[..]).expect("Failed to save image");

//    let raster: pix::Raster<pix::Rgba8> = pix::RasterBuilder::new().with_u8_buffer(sprite_sheet.dimensions.0, sprite_sheet.dimensions.1, &sprite_sheet.bytes[..]);

/*    let mut out_data = Vec::new();
    let mut encoder = png::EncoderBuilder::new();
    let mut encoder = encoder.encode_rasters(&mut out_data);
    encoder.add_frame(&raster, 0).expect("Failed to add frame");
    std::fs::write(filename, out_data).expect("Failed to save image");*/

    // Lastly, we serialize the meta info using serde. This can be any format
    // you want, just implement the trait and pass it to encode.
//    let mut meta_str = "pub(crate) const TEXTURE_SHEET: &[u8] = include_bytes!(concat!(env!(\"OUT_DIR\"), \"/res/texture-sheet.png\"));\npub(crate) mod texture {\n".to_string();

    let mut meta_str = format!("pub(crate) const TEXTURE_SHEET: (u16, u16, &[u8]) = ({}, {}, include_bytes!(concat!(env!(\"OUT_DIR\"), \"/res/texture-sheet.pix\")));\npub(crate) mod texture {{\n", sprite_sheet.dimensions.0, sprite_sheet.dimensions.1);

    for i in &meta.sprites {
        meta_str.push_str(&format!("\tpub(crate) const {}: ([f32; 2],[f32; 2]) = ([{}f32, {}f32], [{}f32, {}f32]);\n", i.name.to_shouty_snake_case(), i.x1, i.y1, i.x2, i.y2));
    }
    meta_str.push_str("}\n");

    meta_str
}
