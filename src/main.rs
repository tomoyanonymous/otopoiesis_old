// use std::thread;


fn main() {
    println!("Hello, world!");
    // let audiothread = thread::spawn(move || { return otopoiesis::audio::run()});
    let _ = otopoiesis::app::run_app();
    // let audio_res = audiothread.join().unwrap();
}
