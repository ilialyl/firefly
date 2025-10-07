use image::{DynamicImage, GenericImageView};

pub fn crop_to_square(dyn_img: DynamicImage) -> DynamicImage {
    let (width, height) = dyn_img.dimensions();

    if width.eq(&height) {
        return dyn_img;
    }

    let size = width.min(height);
    let x = (width - size) / 2;
    let y = (height - size) / 2;

    dyn_img.crop_imm(x, y, size, size)
}
