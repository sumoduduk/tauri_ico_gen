use eyre::OptionExt;
use fast_image_resize as fr;
use image::{codecs::png::PngEncoder, ColorType, DynamicImage, ImageEncoder};

use std::{fs, io::BufWriter, num::NonZeroU32, path::Path};

pub fn resize(img: DynamicImage, dim: u32) -> eyre::Result<()> {
    let width = NonZeroU32::new(img.width()).ok_or_eyre("Non Zero Width")?;
    let height = NonZeroU32::new(img.height()).ok_or_eyre("Non Zero Height")?;

    let img_bffr = img.into_rgba8();

    let mut src_img =
        fr::Image::from_vec_u8(width, height, img_bffr.into_raw(), fr::PixelType::U8x4)?;

    let alpha_mul_div = fr::MulDiv::default();

    alpha_mul_div.multiply_alpha_inplace(&mut src_img.view_mut())?;

    let resize_w = NonZeroU32::new(dim).ok_or_eyre("Non Zero Width")?;
    let resize_h = NonZeroU32::new(dim).ok_or_eyre("Non Zero Height")?;

    let mut dst_image = fr::Image::new(resize_w, resize_h, src_img.pixel_type());

    let mut dst_view = dst_image.view_mut();

    let mut resizer = fr::Resizer::new(fr::ResizeAlg::Convolution(fr::FilterType::Lanczos3));
    resizer.resize(&src_img.view(), &mut dst_view)?;

    alpha_mul_div.divide_alpha_inplace(&mut dst_view).unwrap();

    let file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(Path::new("out.png"))?;

    let mut result_buf = BufWriter::new(file);

    PngEncoder::new(&mut result_buf).write_image(
        dst_image.buffer(),
        resize_w.get(),
        resize_h.get(),
        ColorType::Rgba8,
    )?;

    Ok(())
}
