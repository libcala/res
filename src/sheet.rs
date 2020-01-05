//! Generate Texture Sheet.

use heck;
use png_pong as png;
use sheep;
use pix;

use heck::ShoutySnakeCase;
use sheep::{InputSprite, MaxrectsOptions, MaxrectsPacker};
use std::fs::read_dir;

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
    type Data = (Named, Vec<sheep::SpriteAnchor>);
    type Options = Vec<String>;

    fn encode(
        dimensions: (u32, u32),
        in_sprites: &[sheep::SpriteAnchor],
        options: Self::Options,
    ) -> Self::Data {
        let (w, h) = dimensions;
        let (w, h) = (w as f32, h as f32);
        let mut sprites = vec![];

        for i in 0..in_sprites.len() {
            let x1 = in_sprites[i].position.0 as f32 / w;
            let y1 = in_sprites[i].position.1 as f32 / h;
            let x2 = in_sprites[i].dimensions.0 as f32 / w;
            let y2 = in_sprites[i].dimensions.1 as f32 / h;
            let name = options[in_sprites[i].id].to_string();

            sprites.push(Sprite {
                name,
                x1,
                y1,
                x2,
                y2,
            });
        }

        let named = Named { sprites };

        (named, in_sprites.to_vec())
    }
}

fn sample(pixels: &[u8], width: i32, x: i32, y: i32) -> [u8; 4] {
    [
        pixels[(x + (y * width)) as usize * 4],
        pixels[(x + (y * width)) as usize * 4 + 1],
        pixels[(x + (y * width)) as usize * 4 + 2],
        pixels[(x + (y * width)) as usize * 4 + 3],
    ]
}

fn gen_mipmaps(
    num_mipmaps: u32,
    bytes: &mut Vec<u8>,
    sprites: &mut [sheep::SpriteAnchor],
    width: i32,
) {
    let mut old_width = width;
    let mut old_pixels = bytes.clone();
    let mut pixels = vec![0; old_pixels.len() / 4];

    for _ in 0..num_mipmaps {
        for sprite in sprites.iter_mut() {
            sprite.position.0 /= 2;
            sprite.position.1 /= 2;
            sprite.dimensions.0 /= 2;
            sprite.dimensions.1 /= 2;

            for k in 0..sprite.dimensions.1 {
                for j in 0..sprite.dimensions.0 {
                    let (x, y) = (
                        j as i32 + sprite.position.0 as i32,
                        k as i32 + sprite.position.1 as i32,
                    );

                    let [r00, g00, b00, a00] =
                        sample(&old_pixels, old_width, x * 2, y * 2);
                    let [r01, g01, b01, a01] =
                        sample(&old_pixels, old_width, x * 2, y * 2 + 1);
                    let [r10, g10, b10, a10] =
                        sample(&old_pixels, old_width, x * 2 + 1, y * 2);
                    let [r11, g11, b11, a11] =
                        sample(&old_pixels, old_width, x * 2 + 1, y * 2 + 1);

                    /*                    let [r, g, b, a] = if j < sprite.dimensions.0 / 2 {
                        // +X
                        if k < sprite.dimensions.1 / 2 {
                            // +X+Y
                            [r11, g11, b11, a11]
                        } else {
                            // +X-Y
                            [r10, g10, b10, a10]
                        }
                    } else {
                        // -X
                        if k < sprite.dimensions.1 / 2 {
                            // -X+Y
                            [r01, g01, b01, a01]
                        } else {
                            // -X-Y
                            [r00, g00, b00, a00]
                        }
                    };*/

                    let [red, green, blue, alpha] = [
                        ((u16::from(r00)
                            + u16::from(r01)
                            + u16::from(r10)
                            + u16::from(r11))
                            >> 2) as u8,
                        ((u16::from(g00)
                            + u16::from(g01)
                            + u16::from(g10)
                            + u16::from(g11))
                            >> 2) as u8,
                        ((u16::from(b00)
                            + u16::from(b01)
                            + u16::from(b10)
                            + u16::from(b11))
                            >> 2) as u8,
                        ((u16::from(a00)
                            + u16::from(a01)
                            + u16::from(a10)
                            + u16::from(a11))
                            >> 2) as u8,
                    ];

                    pixels[(x + y * (old_width / 2)) as usize * 4] = red;
                    pixels[(x + y * (old_width / 2)) as usize * 4 + 1] = green;
                    pixels[(x + y * (old_width / 2)) as usize * 4 + 2] = blue;
                    pixels[(x + y * (old_width / 2)) as usize * 4 + 3] = alpha;
                }
            }
        }

        // Add to bytes.
        bytes.extend(&pixels);

        // For next run through loop.
        old_width /= 2;
        old_pixels = pixels;
        pixels = vec![0; old_pixels.len() / 4];
    }
}

pub fn write() -> String {
    println!("cargo:rerun-if-changed=./res/texture/");

    // Find all PNG files.
    let paths = read_dir("./res/texture/").unwrap();
    let mut sprites = vec![];
    let mut names = vec![];
    let mut min_size: Option<u32> = None;

    for path in paths {
        let path = path.unwrap().path();
        let data = std::fs::read(&path).expect("Failed to open PNG");
        let data = std::io::Cursor::new(data);
        let decoder = png::FrameDecoder::<_, pix::Rgba8>::new(data);
        let png::Frame { raster, delay: _ } = decoder
            .last()
            .expect("No frames in PNG")
            .expect("PNG parsing error");
        let dimensions = (raster.width(), raster.height());
        let bytes: &[u8] = raster.as_u8_slice();

        if let Some(min) = min_size {
            min_size = Some(min.min(dimensions.0).min(dimensions.1));
        } else {
            min_size = Some(dimensions.0.min(dimensions.1));
        };

        sprites.push(InputSprite {
            dimensions,
            bytes: bytes.to_vec(),
        });

        println!("cargo:rerun-if-changed={}", path.to_str().unwrap());

        names.push(path.file_stem().unwrap().to_str().unwrap().to_string());
    }

    // Calculate number of mipmaps to generate.
    let mipmap_count = if let Some(min) = min_size {
        let mut full_size = 4096;
        let mut mipmap_count = 0;

        while full_size > min {
            full_size /= 2;
            mipmap_count += 1;
        }

        mipmap_count
    } else {
        0
    };

    // Set texture sheet size.
    let options = MaxrectsOptions::default().max_width(4096).max_height(4096);

    // Do the actual packing! 4 defines the stride, since we're using rgba8 we
    // have 4 bytes per pixel.
    let results = sheep::pack::<MaxrectsPacker>(sprites, 4, options);

    // MaxrectsPacker always returns a single result. Other packers can return
    // multiple sheets; should they, for example, choose to enforce a maximum
    // texture size per sheet.
    let mut sprite_sheet = results
        .into_iter()
        .next()
        .expect("Should have returned a spritesheet");

    // Now, we can encode the sprite sheet in a format of our choosing to
    // save things such as offsets, positions of the sprites and so on.
    let (meta, mut ins) = sheep::encode::<Named>(&sprite_sheet, names);

    // Generate mipmaps.
    gen_mipmaps(mipmap_count, &mut sprite_sheet.bytes, &mut ins, 4096);

    // Next, we save the output to a file using the image crate again.
    let mut filename = std::env::var("OUT_DIR").unwrap();
    filename.push_str("/res/texture-sheet.pix");
    std::fs::write(filename, &sprite_sheet.bytes[..])
        .expect("Failed to save image");

    let mut meta_str = format!("pub(crate) const TEXTURE_SHEET: (u16, u16, &[u8]) = ({}, {}, include_bytes!(concat!(env!(\"OUT_DIR\"), \"/res/texture-sheet.pix\")));\npub(crate) mod texture {{\n", sprite_sheet.dimensions.0, sprite_sheet.dimensions.1);

    for i in &meta.sprites {
        meta_str.push_str(&format!("\tpub(crate) const {}: ([f32; 2],[f32; 2]) = ([{}f32, {}f32], [{}f32, {}f32]);\n", i.name.to_shouty_snake_case(), i.x1, i.y1, i.x2, i.y2));
    }
    meta_str.push_str("}\n");

    meta_str
}
