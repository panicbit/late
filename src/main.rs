extern crate sdl2;
extern crate itertools;
use std::env::args;
use std::fs::File;
use std::io::Read;
use itertools::Itertools;
use sdl2::event::Event;
use sdl2::surface::Surface;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::keyboard::Keycode;

pub const SCALE_FACTOR: usize = 5; 
pub const SCREEN_WIDTH: usize = 1280;
pub const SCREEN_HEIGHT: usize = 720;

mod tile;
use tile::Tile;

fn main() {
    let file = args().nth(1).expect("file parameter missing");
    let mut file = File::open(file).unwrap();
    let mut data = Vec::new();
    file.read_to_end(&mut data).unwrap();
    let tiles: Vec<Surface> = data[0x30000..]
        .chunks(tile::BYTES)
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

        let tile_width = SCALE_FACTOR * tile::WIDTH;
        let tile_height = SCALE_FACTOR * tile::HEIGHT;

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

