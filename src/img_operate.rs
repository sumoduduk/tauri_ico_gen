use std::{path::Path, sync::Arc};

use eyre::OptionExt;

use crate::{quick_scale::resize, utils::check_file};

const SIZES: [u32; 14] = [
    30, 32, 44, 50, 71, 89, 107, 128, 142, 150, 256, 284, 310, 512,
];

pub fn batch_rescale(file_in: &Path) -> eyre::Result<()> {
    let file_in = check_file(file_in).ok_or_eyre("File format must JPG, JPEG, PNG")?;

    let img = image::open(file_in)?;
    let img = Arc::new(img);

    for size in SIZES {
        let img_clone = Arc::clone(&img);
        match size {
            32 => println!("is 32"),
            50 => println!("not 32"),
            128 => {
                if let Some(dyn_img) = Arc::into_inner(img_clone) {
                    resize(dyn_img, 128)?;
                }
            }
            256 => println!("not 32"),
            512 => println!("not 32"),
            _ => println!("not 32"),
        }
    }
    Ok(())
}
