mod gif_displayer;

use std::fs;
use std::thread;
use std::time;

fn main() {
    calc_time();
}

fn load_gif() {
    let file = fs::File::open("./assets/nyan_cat.gif").expect("Unable to open the file.");
    let mut displayer = gif_displayer::GifDisplayer::new(file);
    println!("{}", displayer.size());
    // frame.to_string()
    // fn main() {
        // use std::time::Instant;
        // let now = Instant::now();
    // 
    //     // Code block to measure.
    //     {
    //         my_function_to_measure();
    //     }
    
    //     let elapsed = now.elapsed();
    //     println!("Elapsed: {:.2?}", elapsed);
    // }
    loop {
        print!("{}", termion::cursor::Goto(1, 1));
        // let now = time::Instant::now();
        print!("{}", displayer.to_string());
        // displayer.to_string();
        // println!("Clapsed: {:.2?}", now.elapsed());
        displayer.next_frame();
        thread::sleep(time::Duration::from_millis(100));
    }
}

fn calc_time() {
    let file = fs::File::open("./assets/nyan_cat.gif").expect("Unable to open the file.");
    let mut displayer = gif_displayer::GifDisplayer::new(file);
    loop {
        let now = time::Instant::now();
        displayer.to_string();
        println!("Clapsed: {:.2?}", now.elapsed());
        displayer.next_frame();
    }
}


fn test() {
    // let mut bitmap = ascii_displayer::screen::BitMap::new(128, 128);
    // bitmap.fill(0xFE2E3A00);
    // bitmap.fill(0xFE2E3A00);
    let screen = ascii_displayer::screen::Screen::new(128, 128, 0x00_EE_AA_FF);
    println!("{}", screen);
}
