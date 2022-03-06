use std::thread;


fn main() {
    println!("Hello, world!");
    let audiothread = thread::spawn(move || { return otopoiesis::audio::run()});
    otopoiesis::gui::run();
    let audio_res = audiothread.join().unwrap();
}
