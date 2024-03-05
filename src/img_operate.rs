use std::{
    num::NonZeroU32,
    path::{Path, PathBuf},
    sync::Arc,
    thread,
};

use eyre::OptionExt;

use crate::{
    quick_scale::resize,
    utils::{check_file, create_folder, format_name_logo, join_path, print_err},
};
use fast_image_resize as fr;

use crate::Extension::*;

const SIZES: [u32; 14] = [
    30, 32, 44, 50, 71, 89, 107, 128, 142, 150, 256, 284, 310, 512,
];

pub fn batch_rescale(file_in: &Path, out_folder_name: &str) -> eyre::Result<()> {
    let file_in = check_file(file_in).ok_or_eyre("File format must JPG, JPEG, PNG")?;

    let img = image::open(file_in)?;

    let width = NonZeroU32::new(img.width()).ok_or_eyre("Non Zero Width")?;
    let height = NonZeroU32::new(img.height()).ok_or_eyre("Non Zero Height")?;

    let img_bffr = img.into_rgba8();

    let mut src_img =
        fr::Image::from_vec_u8(width, height, img_bffr.into_raw(), fr::PixelType::U8x4)?;

    let alpha_mul_div = fr::MulDiv::default();
    alpha_mul_div.multiply_alpha_inplace(&mut src_img.view_mut())?;

    let resized_img = Arc::new(src_img);
    let mut handles = Vec::with_capacity(SIZES.len());
    let output_folder = PathBuf::from(out_folder_name);

    create_folder(output_folder.as_path());
    let out_arc = Arc::new(output_folder);

    for size in SIZES {
        let img_clone = Arc::clone(&resized_img);
        let out_clone = Arc::clone(&out_arc);

        match size {
            32 => {
                let handle = thread::spawn(move || {
                    let out_file = join_path(out_clone.as_path(), "32x32.png");
                    let res = resize(img_clone.as_ref(), 32, out_file.as_path(), PNG);
                    print_err(res);
                });

                handles.push(handle);
            }
            50 => {
                let handle = thread::spawn(move || {
                    let out_file = join_path(out_clone.as_path(), "StoreLogo.png");
                    let res = resize(img_clone.as_ref(), 50, out_file.as_path(), PNG);
                    print_err(res);
                });

                handles.push(handle);
            }
            128 => {
                let img_clone_2 = Arc::clone(&img_clone);
                let out_clone_2 = Arc::clone(&out_clone);
                let handle = thread::spawn(move || {
                    let out_file = join_path(out_clone_2.as_path(), "128x128.png");
                    let res = resize(img_clone_2.as_ref(), 128, out_file.as_path(), PNG);
                    print_err(res);
                });
                handles.push(handle);

                let handle_ico = thread::spawn(move || {
                    let out_file = join_path(out_clone.as_path(), "icon.ico");
                    let res = resize(img_clone.as_ref(), 128, out_file.as_path(), ICO);
                    print_err(res);
                });

                handles.push(handle_ico);
            }
            256 => {
                let handle = thread::spawn(move || {
                    let out_file = join_path(out_clone.as_path(), "128x128@2x.png");
                    let res = resize(img_clone.as_ref(), 256, out_file.as_path(), PNG);
                    print_err(res);
                });

                handles.push(handle);
            }
            512 => {
                let handle = thread::spawn(move || {
                    let out_file = join_path(out_clone.as_path(), "icon.png");
                    let res = resize(img_clone.as_ref(), 512, out_file.as_path(), PNG);
                    print_err(res);
                });

                handles.push(handle);
            }
            num => {
                let handle = thread::spawn(move || {
                    let name_logo = format_name_logo(num);
                    let out_file = join_path(out_clone.as_path(), &name_logo);
                    let res = resize(img_clone.as_ref(), num, out_file.as_path(), PNG);
                    print_err(res);
                });

                handles.push(handle);
            }
        }
    }

    for handle in handles {
        match handle.join() {
            Ok(_) => (),
            Err(_) => eprintln!("ERROR: cant join thread"),
        }
    }

    Ok(())
}
