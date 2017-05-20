extern crate sdl2;
extern crate itertools;
use std::env::args;
use std::fs::File;
use std::io::Read;
use itertools::Itertools;
use sdl2::event::Event;
use sdl2::surface::Surface;
use sdl2::pixels::PixelFormatEnum::RGB24;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::keyboard::Keycode;

const TILE_WIDTH: usize = 8;
const TILE_HEIGHT: usize = 8;
const TILE_BYTES: usize = 16;

const SCALE_FACTOR: usize = 5; 
const SCREEN_WIDTH: usize = 1280;
const SCREEN_HEIGHT: usize = 720;

fn main() {
    let file = args().nth(1).expect("file parameter missing");
    let mut file = File::open(file).unwrap();
    let mut data = Vec::new();
    file.read_to_end(&mut data).unwrap();
    let tiles: Vec<Surface> = data[0x30000..]
        .chunks(TILE_BYTES)
        .take(8200)
        .map(Tile::from_bytes)
        .map(Tile::into_surface)
        .collect();


    let sdl_context = sdl2::init().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("TLoZ:LA Tileset viewer ", SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32).build().unwrap();
    let mut offset: usize = 0;

    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => break 'main,
                  Event::KeyDown { keycode: Some(Keycode::Down),..}
                | Event::KeyDown { keycode: Some(Keycode::Right),..} => offset = offset.saturating_add(SCALE_FACTOR*8),
                  Event::KeyDown { keycode: Some(Keycode::Up),..}
                | Event::KeyDown { keycode: Some(Keycode::Left),..} => offset = offset.saturating_sub(SCALE_FACTOR*8),
                _ => {}
            }
        }

        let (screen_width, _) = window.size();
        let mut surface = window.surface(&mut event_pump).unwrap();

        {
            let rect = surface.rect();
            surface.fill_rect(rect, Color::RGB(22,22,22)).unwrap();
        }

        let tile_width = SCALE_FACTOR * TILE_WIDTH;
        let tile_height = SCALE_FACTOR * TILE_HEIGHT;

        for (i, mut tiles) in tiles.iter().chunks(4).into_iter().enumerate() {
            let max_fit = screen_width as usize / (2 * tile_width);
            let max_width = max_fit * (2 * tile_width);
            let x = (i * 2 * tile_width) % max_width as usize;
            let n_rows = (i * 2 * tile_width) / max_width as usize ;
            let y = n_rows * 2 * tile_width;

            let tile = tiles.next().unwrap();
            let source = tile.rect();
            let target = Rect::new(
                x as i32,
                y as i32 - offset as i32,
                tile_width as u32,
                tile_height as u32
            );
            tile.blit_scaled(source, &mut surface, target).unwrap();

            let tile = tiles.next().unwrap();
            let source = tile.rect();
            let target = Rect::new(
                x as i32,
                y as i32 + tile_height as i32 - offset as i32,
                tile_width as u32,
                tile_height as u32
            );
            tile.blit_scaled(source, &mut surface, target).unwrap();

            let tile = tiles.next().unwrap();
            let source = tile.rect();
            let target = Rect::new(
                x as i32 + tile_width as i32,
                y as i32 - offset as i32,
                tile_width as u32,
                tile_height as u32
            );
            tile.blit_scaled(source, &mut surface, target).unwrap();

            let tile = tiles.next().unwrap();
            let source = tile.rect();
            let target = Rect::new(
                x as i32 + tile_width as i32,
                y as i32 + tile_height as i32 - offset as i32,
                tile_width as u32,
                tile_height as u32
            );
            tile.blit_scaled(source, &mut surface, target).unwrap();
        }

        surface.finish().unwrap();
    }
}

#[derive(Copy,Clone,Debug)]
enum GBPalletIndex {
    One,
    Two,
    Three,
    Four,
}

#[derive(Clone,Debug)]
struct Tile(Vec<GBPalletIndex>);

impl Tile {
    fn from_bytes(bytes: &[u8]) -> Self{
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

    fn into_surface(tile: Tile) -> Surface<'static> {
        let mut surface = Surface::new(TILE_WIDTH as u32, TILE_HEIGHT as u32, RGB24).expect("new surface");
        let pitch = surface.pitch();

        surface.with_lock_mut(|data| {
            let target = data
                .chunks_mut(pitch as usize)
                .flat_map(|data| data[..3*TILE_WIDTH].chunks_mut(3));
            let source = tile.0.iter();

            for (pallet_index, target) in source.zip(target) {
                let color = LINK_PALLET.get_color(*pallet_index);
                target[0] = color.r;
                target[1] = color.g;
                target[2] = color.b;
            }
        });

        surface
    }
}

struct GBPallet {
    one:   Color,
    two:   Color,
    three: Color,
    four:  Color,
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

const BGB_PALLET: GBPallet = GBPallet {
    one:   Color { r: 224, g: 248, b: 208, a: 255 },
    two:   Color { r: 140, g: 198, b: 115, a: 255 },
    three: Color { r:  52, g: 104, b:  86, a: 255 },
    four:  Color { r:   8, g:  24, b:  32, a: 255 },
};

const LINK_PALLET: GBPallet = GBPallet {
    one:   Color { r:  0xE0, g: 0xF8, b:  0xD0, a: 255 },
    two:   Color { r: 140, g: 198, b: 115, a: 255 },
    three: Color { r:  0x08, g: 0x18, b:  0x20, a: 255 },
    four:  Color { r:  0xE0, g: 0xF8, b:  0xD0, a: 255 },
};
