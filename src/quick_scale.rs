use eyre::OptionExt;
use fast_image_resize as fr;
use image::DynamicImage;

use std::num::NonZeroU32;

pub fn resize(img: DynamicImage) -> eyre::Result<()> {
    let width = NonZeroU32::new(img.width()).ok_or_eyre("Non Zero Width");
    let height = NonZeroU32::new(img.height()).ok_or_eyre("Non Zero Height");

    let img_bffr = img.into_rgba8().into_raw();

    Ok(())
}
