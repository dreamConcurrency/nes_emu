pub mod cpu;
pub mod opcodes;
pub mod cartridge;
pub mod bus;
pub mod trace;
pub mod ppu;
pub mod render;

use cpu::CPU;
use bus::Bus;
use cartridge::Rom;
use trace::trace;
use render::frame::Frame;
use render::palette;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate bitflags;

#[cfg(test)]
mod cpu_tests;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Tile viewer", (256.0 * 3.0) as u32, (240.0 * 3.0) as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    canvas.set_scale(3.0, 3.0).unwrap();

    let creator = canvas.texture_creator();
    let mut texture = creator
        .create_texture_target(PixelFormatEnum::RGB24, 256, 240)
        .unwrap();
    
    // Load the game.
    //let bytes: Vec<u8> = std::fs::read("pacman.nes").unwrap();
    //let rom = Rom::new(&bytes).unwrap();
    
    //let bus = Bus::new(rom);
    //let mut cpu = CPU::new(bus);

    //cpu.reset();
    //cpu.program_counter = 0xC000; 
    //let mut screen_state = [0 as u8; 32 * 3 * 32];
    //let mut rng = rand::thread_rng();


    //cpu.run_with_callback(move |cpu| {
        //println!("{}", trace(cpu));
        //handle_user_input(cpu, &mut event_pump);
        //cpu.mem_write(0xfe, rng.gen_range(1, 16));

        //if read_screen_state(cpu, &mut screen_state) {
        //    texture.update(None, &screen_state, 32 * 3).unwrap();
        //    canvas.copy(&texture, None, None).unwrap();
        //    canvas.present();
        //}

        //std::thread::sleep(std::time::Duration::new(0, 120_000));
    //});

   let bytes: Vec<u8> = std::fs::read("pacman.nes").unwrap();
   let rom = Rom::new(&bytes).unwrap();

   let right_bank = show_tile_bank(&rom.chr_rom, 1);

   texture.update(None, &right_bank.data, 256 * 3).unwrap();
   canvas.copy(&texture, None, None).unwrap();
   canvas.present();

   loop {
      for event in event_pump.poll_iter() {
         match event {
           Event::Quit { .. }
           | Event::KeyDown {
               keycode: Some(Keycode::Escape),
               ..
           } => std::process::exit(0),
           _ => { /* do nothing */ }
         }
      }
   }
}

fn show_tile_bank(chr_rom: &Vec<u8>, bank: usize) ->Frame {
    assert!(bank <= 1);

    let mut frame = Frame::new();
    let mut tile_y = 0;
    let mut tile_x = 0;
    let bank = (bank * 0x1000) as usize;

    for tile_n in 0..255 {
        if tile_n != 0 && tile_n % 20 == 0 {
            tile_y += 10;
            tile_x = 0;
        }
        let tile = &chr_rom[(bank + tile_n * 16)..=(bank + tile_n * 16 + 15)];

        for y in 0..=7 {
            let mut upper = tile[y];
            let mut lower = tile[y + 8];

            for x in (0..=7).rev() {
                let value = (1 & upper) << 1 | (1 & lower);
                upper = upper >> 1;
                lower = lower >> 1;
                let rgb = match value {
                    0 => palette::SYSTEM_PALLETE[0x01],
                    1 => palette::SYSTEM_PALLETE[0x23],
                    2 => palette::SYSTEM_PALLETE[0x27],
                    3 => palette::SYSTEM_PALLETE[0x30],
                    _ => panic!("can't be"),
                };
                frame.set_pixel(tile_x + x, tile_y + y, rgb)
            }
        }

        tile_x += 10;
    }
    frame
}
