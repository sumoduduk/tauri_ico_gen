use std::path::Path;
use std::time::Instant;

use tauri_ico_gen::img_operate::batch_rescale;

fn main() {
    let start = Instant::now();
    let path = Path::new("./test/test.png");
    let _ = batch_rescale(path, "./output");

    let duration = start.elapsed();
    println!("Time duration : {:#?}", duration);
}
