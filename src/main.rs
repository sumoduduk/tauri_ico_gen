use std::path::Path;

use tauri_ico_gen::img_operate::batch_rescale;

fn main() {
    let path = Path::new("./test/test.png");
    let _ = batch_rescale(path);
}
