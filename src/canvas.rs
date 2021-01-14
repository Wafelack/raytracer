use crate::colors::write_color_to_writer;
use crate::vec3::color;
use rayon::prelude::*;

use std::io::stdout;
use std::io::Write;

use std::sync::{Mutex , Condvar , Arc , atomic::{Ordering , AtomicUsize}};

pub struct Canvas {
    pixels: Box<[color]>,
    samples_per_pixel: usize,
    xsize: usize,
    ysize: usize,
}

const UPDATE_INTEVAL: usize = 1024;

#[inline]
fn index_to_xy(xsize: usize , index: usize) -> (usize , usize){
    (index % xsize , index / xsize)
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
        P: FnMut(usize, usize) + Send + 'static,
    {
        let size = x.checked_mul(y).unwrap();
        let mut pixels = Vec::with_capacity(size);
        let notify = Arc::new((Mutex::new(()) , Condvar::new() , AtomicUsize::new(0_usize)));
        let nt = Arc::clone(&notify);
        let notify_thread = std::thread::spawn(move ||{
            let (lock , cvar , counter) = &*nt;
            let mut lock = lock.lock().unwrap();
            let mut num_pixels_processed = 0;
            while num_pixels_processed < size{
                progress(size , num_pixels_processed);
                lock = cvar.wait(lock).unwrap();
                let add = counter.swap(0 , Ordering::Relaxed);
                if add == std::usize::MAX{
                    progress(size , size);
                    break;
                }
                num_pixels_processed += add;
            }
        });
        pixels.par_extend((0..size).into_par_iter().map(|idx| {
            let (x , y) = index_to_xy(x , idx);
            if notify.2.fetch_add(1 , Ordering::Relaxed) >= UPDATE_INTEVAL{
                notify.1.notify_one();
            } 
            func(x , y)
        }));
        notify.2.store(std::usize::MAX , Ordering::SeqCst);
        notify.1.notify_one();
        notify_thread.join().unwrap();
        Self {
            samples_per_pixel,
            pixels: pixels.into_boxed_slice(),
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
        self.iter_pixels()
            .try_for_each(|&p| write_color_to_writer(writer, p, self.samples_per_pixel))
    }
    pub fn write_header(&self) {
        self.write_header_to_writer(&mut stdout()).unwrap();
    }
    pub fn write_header_to_writer<W: Write>(&self, w: &mut W) -> Result<(), std::io::Error> {
        writeln!(w, "P3\n{} {}\n255", self.xsize, self.ysize)
    }

    pub fn iter_pixels(&self) -> impl Iterator<Item = &'_ color> + '_ {
        self.pixels.chunks(self.xsize).rev().flatten()
    }

    pub fn dimensions(&self) -> (usize, usize) {
        (self.xsize, self.ysize)
    }
    pub fn get_pixels(&self) -> &[color] {
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
