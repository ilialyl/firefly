use image::{DynamicImage, GenericImageView};

pub fn crop_to_square(img: DynamicImage) -> DynamicImage {
    let (width, height) = img.dimensions();

    if width.eq(&height) {
        return img;
    }

    let size = width.min(height);
    let x = (width - size) / 2;
    let y = (height - size) / 2;

    img.crop_imm(x, y, size, size)
}
