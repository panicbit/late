use sdl2::surface::Surface;
use sdl2::pixels::PixelFormatEnum::RGB24;
use sdl2::pixels::Color;

pub const WIDTH: usize = 8;
pub const HEIGHT: usize = 8;
pub const BYTES: usize = 16;

#[derive(Copy,Clone,Debug)]
enum GBPalletIndex {
    One,
    Two,
    Three,
    Four,
}

#[derive(Clone,Debug)]
pub struct Tile(Vec<GBPalletIndex>);

impl Tile {
    pub fn from_bytes(bytes: &[u8]) -> Self{
        let mut pixels = Vec::new();

        for row in bytes.chunks(2) {
            if row.len() < 2 { continue }
            let a = row[0];
            let b = row[1];

            for i in 0..8 {
                let pixel = (a >> (7-i) & 1) << 1 | (b >> (7-i) & 1) << 0;
                let pixel = match pixel {
                    0 => GBPalletIndex::One,
                    1 => GBPalletIndex::Two,
                    2 => GBPalletIndex::Three,
                    3 => GBPalletIndex::Four,
                    _ => panic!("Bug in color conversion!")
                };

                pixels.push(pixel);
            }
        }

        Tile(pixels)
    }

    pub fn into_surface(tile: Tile) -> Surface<'static> {
        let mut surface = Surface::new(WIDTH as u32, HEIGHT as u32, RGB24).expect("new surface");
        let pitch = surface.pitch();

        surface.with_lock_mut(|data| {
            let target = data
                .chunks_mut(pitch as usize)
                .flat_map(|data| data[..3*WIDTH].chunks_mut(3));
            let source = tile.0.iter();

            for (pallet_index, target) in source.zip(target) {
                let color = BGB_PALLET.get_color(*pallet_index);
                target[0] = color.r;
                target[1] = color.g;
                target[2] = color.b;
            }
        });

        surface
    }
}

pub struct GBPallet {
    pub one:   Color,
    pub two:   Color,
    pub three: Color,
    pub four:  Color,
}

impl GBPallet {
    fn get_color(&self, index: GBPalletIndex) -> Color {
        match index {
            GBPalletIndex::One => self.one,
            GBPalletIndex::Two => self.two,
            GBPalletIndex::Three => self.three,
            GBPalletIndex::Four => self.four,
        }
    }
}

pub const BGB_PALLET: GBPallet = GBPallet {
    one:   Color { r: 224, g: 248, b: 208, a: 255 },
    two:   Color { r: 140, g: 198, b: 115, a: 255 },
    three: Color { r:  52, g: 104, b:  86, a: 255 },
    four:  Color { r:   8, g:  24, b:  32, a: 255 },
};

pub const LINK_PALLET: GBPallet = GBPallet {
    one:   Color { r:  0xE0, g: 0xF8, b:  0xD0, a: 255 },
    two:   Color { r: 140, g: 198, b: 115, a: 255 },
    three: Color { r:  0x08, g: 0x18, b:  0x20, a: 255 },
    four:  Color { r:  0xE0, g: 0xF8, b:  0xD0, a: 255 },
};
