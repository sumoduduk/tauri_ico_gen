use eyre::OptionExt;
use fast_image_resize as fr;
use fr::Image;
use image::{
    codecs::{ico::IcoEncoder, png::PngEncoder},
    ColorType, ImageEncoder,
};

use std::{fs, io::BufWriter, num::NonZeroU32, path::Path};

use crate::Extension;

pub fn resize(src_img: &Image, dim: u32, out_file: &Path, ext: Extension) -> eyre::Result<()> {
    let alpha_mul_div = fr::MulDiv::default();

    let resize_w = NonZeroU32::new(dim).ok_or_eyre("Non Zero Width")?;
    let resize_h = NonZeroU32::new(dim).ok_or_eyre("Non Zero Height")?;

    let mut dst_image = fr::Image::new(resize_w, resize_h, src_img.pixel_type());

    let mut dst_view = dst_image.view_mut();

    let mut resizer = fr::Resizer::new(fr::ResizeAlg::Convolution(fr::FilterType::Lanczos3));
    resizer.resize(&src_img.view(), &mut dst_view)?;

    alpha_mul_div.divide_alpha_inplace(&mut dst_view)?;

    let file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(out_file)?;

    let mut result_buf = BufWriter::new(file);

    match ext {
        Extension::PNG => {
            PngEncoder::new(&mut result_buf).write_image(
                dst_image.buffer(),
                resize_w.get(),
                resize_h.get(),
                ColorType::Rgba8,
            )?;
        }
        Extension::ICO => {
            IcoEncoder::new(&mut result_buf).write_image(
                dst_image.buffer(),
                resize_w.get(),
                resize_h.get(),
                ColorType::Rgba8,
            )?;
        }
    }

    Ok(())
}
