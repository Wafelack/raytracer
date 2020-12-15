use crate::colors::write_color_to_writer;
use crate::vec3::color;
use rayon::prelude::*;

use std::io::stdout;
use std::io::Write;

pub struct Canvas {
    pixels: Box<[color]>,
    samples_per_pixel: usize,
    xsize: usize,
    ysize: usize,
}

impl Canvas {
    pub fn from_fn<F>(x: usize, y: usize, samples_per_pixel: usize, mut f: F) -> Self
    where
        F: FnMut(usize, usize) -> color,
    {
        let size = x.checked_mul(y).unwrap();
        let mut pixels = (vec![color::default(); size]).into_boxed_slice();
        let mut pix_iter = pixels.iter_mut();
        for y in 0..y {
            for x in 0..x {
                *pix_iter.next().unwrap() = f(x, y);
            }
        }
        Self {
            samples_per_pixel,
            pixels,
            xsize: x,
            ysize: y,
        }
    }
    pub fn from_fn_parallel<F>(x: usize, y: usize, samples_per_pixel: usize, f: F) -> Self
    where
        F: Fn(usize, usize) -> color + Send + Sync,
    {
        Self::from_fn_parallel_with_progress(x, y, samples_per_pixel, f, |_, _| {})
    }
    pub fn from_fn_parallel_with_progress<F, P>(
        x: usize,
        y: usize,
        samples_per_pixel: usize,
        func: F,
        mut progress: P,
    ) -> Self
    where
        F: Fn(usize, usize) -> color + Send + Sync,

        /* first arg: total number of pixels
         *  second arg: number of pixels compleated
         */
        P: FnMut(usize, usize),
    {
        let size = x.checked_mul(y).unwrap();
        let mut pixels = (vec![color::default(); size]).into_boxed_slice();
        for (y_idx, yline) in pixels.chunks_mut(x).enumerate() {
            assert_eq!(yline.len(), x);
            yline.par_iter_mut().zip(0..x).for_each(|(px, x)| {
                *px = func(x, y_idx);
            });
            progress(size, (y_idx + 1) * x);
        }
        Self {
            samples_per_pixel,
            pixels,
            xsize: x,
            ysize: y,
        }
    }
    pub fn write_pixels(&self) {
        let stdout = stdout();
        let mut locked = stdout.lock();
        self.write_pixels_to_writer(&mut locked).unwrap();
    }
    pub fn write_pixels_to_writer<W: Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
        self.pixels.chunks(self.xsize).rev().try_for_each(|line| {
            line.iter()
                .try_for_each(|p| write_color_to_writer(writer, *p, self.samples_per_pixel))
        })
    }
    pub fn write_header(&self){
        self.write_header_to_writer(&mut stdout()).unwrap();
    }
    pub fn write_header_to_writer<W: Write>(&self , w: &mut W) -> Result<() , std::io::Error>{
        writeln!(w , "P3\n{} {}\n255", self.xsize , self.ysize)
    }


    pub fn dimensions(&self) -> (usize, usize) {
        (self.xsize, self.ysize)
    }
    fn get_pixels(&self) -> &[color] {
        &*self.pixels
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_fn_dummy() {
        let size = 100usize;
        let c = Canvas::from_fn(size, size, 1, |_, _| {
            use std::f32::NAN;
            color::from(NAN, NAN, NAN)
        });
        for pix in c.get_pixels().iter() {
            assert!(pix.x().is_nan());
            assert!(pix.y().is_nan());
            assert!(pix.z().is_nan());
        }
    }

    #[test]
    fn from_fn_parallel() {
        let size = 100usize;
        let c = Canvas::from_fn_parallel(size, size, 1, |_, _| {
            use std::f32::NAN;
            color::from(NAN, NAN, NAN)
        });
        for pix in c.get_pixels().iter() {
            assert!(pix.x().is_nan());
            assert!(pix.y().is_nan());
            assert!(pix.z().is_nan());
        }
    }
}
