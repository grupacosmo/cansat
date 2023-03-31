use heapless::Vec;

/// Data structure for double buffering.
///
/// It consists of statically allocated write and read buffers.
/// Buffers can be swapped in O(1).
//
// INVARIANT:
// 1. 0 <= write_buf_idx <= 1
#[derive(Default)]
pub struct DoubleBuf<T, const SIZE: usize> {
    bufs: [Vec<T, SIZE>; 2],
    write_buf_idx: usize,
}

impl<T, const SIZE: usize> DoubleBuf<T, SIZE> {
    pub fn write(&mut self) -> &mut Vec<T, SIZE> {
        // SAFETY:
        // * self.bufs.len() == 2,
        // * struct invariant
        unsafe { self.bufs.get_unchecked_mut(self.write_buf_idx) }
    }

    pub fn read(&self) -> &Vec<T, SIZE> {
        let read_buf_idx = self.write_buf_idx ^ 1;
        // SAFETY:
        // * self.bufs.len() == 2,
        // * struct invariant
        // * 0 <= x ^ 1 <= 1 for any x, where 0 <= x <= 1,
        unsafe { self.bufs.get_unchecked(read_buf_idx) }
    }

    pub fn swap(&mut self) {
        self.write_buf_idx ^= 1;
    }
}
