use rayon::{iter::ParallelIterator, prelude::{IndexedParallelIterator, ParallelSliceMut}};
use super::primitives::Color;

pub struct Canvas {
    pub(super) width: usize,
    pub(super) height: usize,
    pub(super) buffer: Vec<u32>,
}

pub struct Row<'a> {
    pub(super) y: usize,
    pub(super) buffer: &'a mut [u32],
}

impl Row<'_> {
    pub fn draw_pixel(&mut self, x: usize, color: Color) {
        self.buffer[x] = color.into()
    }
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        let buffer = vec![0_u32; width * height];
        Self {
            height,
            width,
            buffer,
        }
    }

    pub fn clear(&mut self) {
        self.buffer.iter_mut().for_each(|v| *v = 0);
    }

    pub fn par_rows_mut(&mut self) -> impl IndexedParallelIterator<Item = Row> {
        self.buffer
            .par_chunks_mut(self.width)
            .enumerate()
            .map(move |(y, buffer)| { Row {y, buffer}})
    }
}
