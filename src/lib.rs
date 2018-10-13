extern crate image;

use image::{GenericImage, Pixel};

fn psnr<I, P>(lhs: I, rhs: I) -> Result<Vec<f32>, &'static str>
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
    let mut max_err: Vec<f32> = Vec::with_capacity(channels_size);
    max_err.resize(channels_size, 0.0);
    let mut mean_err: Vec<f32> = Vec::with_capacity(channels_size);
    mean_err.resize(channels_size, 0.0);
    let (width, height) = lhs.dimensions();

    for line in 0..height {
        for column in 0..width {
            let l_pixel = lhs.get_pixel(column, line);
            let r_pixel = rhs.get_pixel(column, line);

            for chan in 0..channels_size {
                let diff = ((l_pixel.channels()[chan] as i16 - r_pixel.channels()[chan] as i16)
                    .abs() as f32)
                    .powf(2.0);
                if max_err[chan] < l_pixel.channels()[chan] as f32 {
                    max_err[chan] = l_pixel.channels()[chan] as f32;
                }
                mean_err[chan] += diff
            }
        }
    }
    let mut psnr_value: Vec<f32> = Vec::with_capacity(channels_size);
    psnr_value.resize(channels_size, 0.0);
    for chan in 0..channels_size {
        max_err[chan] = max_err[chan].powf(2.0);
        mean_err[chan] = mean_err[chan] / (width * height) as f32;
        if mean_err[chan] != 0.0 {
            psnr_value[chan] = 10.0 * (max_err[chan] / mean_err[chan]).log10();
        }
    }

    return Ok(psnr_value);
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::ImageBuffer;

    #[test]
    fn smoke_test() {
        let original = ImageBuffer::from_fn(8, 8, |_x, _y| image::Luma([255u8]));
        let edited = ImageBuffer::from_fn(8, 8, |x, y| {
            if (x, y) == (0, 0) {
                image::Luma([0u8])
            } else {
                image::Luma([255u8])
            }
        });

        let psnr_val = psnr(original, edited).unwrap();

        // max^2 = 255^2 = 65025
        // mse = sum(sum(orig - edited)^2)/size = 255^2/8*8 = 1016.015625
        // psnr = 10 * log_10(max^2/mse) = 10 * log_10(65025/1016.015625)
        // psnr = 10 * 1.8061799739838869 ~= 18.061
        assert!(18.1 > psnr_val[0] && psnr_val[0] > 18.0);
    }
}
