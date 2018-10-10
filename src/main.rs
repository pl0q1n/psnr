extern crate image;

use image::{GenericImage, Pixel};

fn psnr<I, P>(lhs: I, rhs: I) -> Result<Vec<usize>, &'static str>
where
    P: Pixel<Subpixel = u8>,
    I: GenericImage<Pixel = P>,
{
    if lhs.dimensions() != rhs.dimensions() {
        return Err("Pictures have different dimensions");
    }
    if lhs.get_pixel(0, 0).channels().len() != rhs.get_pixel(0, 0).channels().len() {
        return Err("Pictures have different number of channels");
    }

    let channels_size = lhs.get_pixel(0, 0).channels().len();
    let mut max_err: Vec<u32> = Vec::with_capacity(channels_size);
    max_err.resize(channels_size, 0);
    let mut mean_err: Vec<u32> = Vec::with_capacity(channels_size);
    mean_err.resize(channels_size, 0);
    let (width, height) = lhs.dimensions();

    for line in 0..height {
        for column in 0..width {
            let l_pixel = lhs.get_pixel(column, line);
            let r_pixel = rhs.get_pixel(column, line);

            for chan in 0..channels_size {
                let diff = ((l_pixel.channels()[chan] as i16 - r_pixel.channels()[chan] as i16)
                    .abs() as u32)
                    .pow(2);
                if max_err[chan] < diff {
                    max_err[chan] = diff;
                }
                mean_err[chan] += diff
            }
        }
    }
    let mut psnr_value = Vec::with_capacity(channels_size);
    psnr_value.resize(channels_size, 0);
    for chan in 0..channels_size {
        mean_err[chan] = mean_err[chan] / (width * height);
        if mean_err[chan] != 0 {
            psnr_value[chan] =
                ((10 as f64 * (max_err[chan] / mean_err[chan]) as f64).log10()) as usize;
        }
    }

    return Ok(psnr_value);
}

fn main() {
    let img_1 = image::open("test.jpg").unwrap();
    let img_2 = image::open("test1.jpg").unwrap();

    let psnr_val = psnr(img_1, img_2).unwrap();

    println!("{:?}", psnr_val);
}
