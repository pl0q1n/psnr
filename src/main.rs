extern crate image;

use image::{imageops, GenericImage, ImageBuffer, Pixel};

fn psnr<I, P>(lhs: I, rhs: I) -> Result<usize, &'static str>
where
    P: Pixel<Subpixel = u8>,
    I: GenericImage<Pixel = P>,
{
    if lhs.dimensions() != rhs.dimensions() {
        return Err("Pictures have different dimensions");
    }

    let mut max_err = 0;
    let mut mean_err = 0u32;
    let (width, height) = lhs.dimensions();
    for line in (0..height) {
        for column in (0..width) {
            let l_pixel = unsafe { lhs.get_pixel(column, line).to_luma() };
            let r_pixel = unsafe { rhs.get_pixel(column, line).to_luma() };
            let diff = ((l_pixel.data[0] as i8 - r_pixel.data[0] as i8).abs() as u32).pow(2);
            if max_err < diff {
                max_err = diff;
            }
            mean_err += diff
        }
    }
    mean_err = mean_err / (width * height);
    let mut psnr_value = 0f64;
    if mean_err != 0 {
        psnr_value = 10 as f64 * ((max_err / mean_err) as f64).log10();
    }
    return Ok(psnr_value as usize);
}

fn main() {
    let img_1 = image::open("test.jpg").unwrap();
    let img_2 = image::open("test1.jpg").unwrap();

    let psnr_val = psnr(img_1, img_2);
    
    println!("{}", psnr_val.unwrap());
}
