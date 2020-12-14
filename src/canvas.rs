use crate::colors::write_color;
use crate::vec3::color;
use rayon::prelude::*;

pub struct Canvas{
    pixels: Box<[color]>,
    samples_per_pixel: usize,
    xsize: usize,
    ysize: usize,
}

impl Canvas{
    pub fn from_fn<F>(x: usize , y: usize , samples_per_pixel: usize , mut f: F)  -> Self
        where
            F: FnMut(usize , usize) -> color,
    {
        let size = x.checked_mul(y).unwrap();
        let mut pixels = (vec![color::default() ; size]).into_boxed_slice();
        let mut pix_iter = pixels.iter_mut();
        for y in 0..y{
            for x in 0..x{
                *pix_iter.next().unwrap() = f(x , y);
            }
        }
        Self{
            samples_per_pixel,
            pixels,
            xsize: x,
            ysize: y,
        }
    }
    pub fn from_fn_paralell<F>(x: usize , y: usize , samples_per_pixel: usize , mut f: F)  -> Self
        where
            F: Fn(usize , usize) -> color + Send + Sync,
    {
        let size = x.checked_mul(y).unwrap();
        let mut pixels = (vec![color::default() ; size]).into_boxed_slice();
        for (y , yline) in pixels.chunks_mut(x).enumerate(){
            assert_eq!(yline.len() , x);
            yline.par_iter_mut().zip(0..x).for_each(|(px , x)|{
                *px = f(x , y);
            })
        }
        Self{
            samples_per_pixel,
            pixels,
            xsize: x,
            ysize: y,
        }
    }
    pub fn write_pixels(&self){
        self.pixels.iter().rev().for_each(|c|{
            write_color(*c , self.samples_per_pixel as i32)
        })
    }
}
