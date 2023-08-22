const MAX_DATA_POINTS: usize = 10;

pub struct Data {
    pub time: f64,
    pub bme: BmeData,
    pub orientation: Option<Orientation>,
    pub acceleration: Option<Acceleration>,
    pub signal_strength: Option<SignalStrength>,
}

pub struct BmeData {
    pub temperature: OffsetVec<[f64; 2]>,
    pub pressure: OffsetVec<[f64; 2]>,
    pub height: OffsetVec<[f64; 2]>,
}

pub struct SignalStrength {
    pub rssi: i32,
    pub snr: i32,
}

pub struct Orientation {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub struct Acceleration {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Data {
    pub fn empty() -> Self {
        Self {
            time: 0f64,
            bme: BmeData::empty(),
            orientation: None,
            acceleration: None,
            signal_strength: None,
        }
    }
}

impl BmeData {
    pub fn empty() -> Self {
        Self {
            temperature: OffsetVec::new(MAX_DATA_POINTS),
            pressure: OffsetVec::new(MAX_DATA_POINTS),
            height: OffsetVec::new(MAX_DATA_POINTS),
        }
    }
}

pub struct OffsetVec<T> {
    vec: Vec<T>,
    offset: usize,
    max_size: usize,
    elem_count: usize,
}

impl<T> OffsetVec<T>
where
    T: Clone + Copy,
{
    pub fn new(max_size: usize) -> Self {
        OffsetVec {
            vec: Vec::with_capacity(max_size),
            offset: 0,
            elem_count: 0,
            max_size,
        }
    }

    pub fn push(&mut self, elem: T) {
        if self.elem_count < self.max_size {
            self.elem_count += 1;
            self.vec.push(elem);
        } else {
            self.vec[self.offset] = elem;
        }
        self.offset = (self.offset + 1) % self.max_size;
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.elem_count {
            return None;
        }
        if self.elem_count < self.max_size {
            return Some(&self.vec[index]);
        }
        let index = (self.offset + index) % self.max_size;

        Some(&self.vec[index])
    }

    pub fn to_vec(&self) -> Vec<T> {
        //TODO optimize
        let mut vec: Vec<T> = Vec::with_capacity(self.elem_count);
        for i in 0..self.elem_count {
            vec.push(*self.get(i).unwrap());
        }

        vec
    }

    pub fn get_elem_count(&self) -> usize {
        self.elem_count
    }

    pub fn get_max_size(&self) -> usize {
        self.max_size
    }
}

pub struct OffsetVecIter<'a, T>
where
    T: Clone + Copy,
{
    vec: &'a OffsetVec<T>,
    elem_count: usize,
    index: usize,
}

impl<'a, T> IntoIterator for &'a OffsetVec<T>
where
    T: Clone + Copy,
{
    type Item = &'a T;
    type IntoIter = OffsetVecIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        OffsetVecIter {
            vec: self,
            elem_count: self.get_elem_count(),
            index: 0,
        }
    }
}

impl<'a, T> Iterator for OffsetVecIter<'a, T>
where
    T: Clone + Copy,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.elem_count {
            return None;
        }
        let val = self.vec.get(self.index);
        self.index += 1;

        val
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_offset_vec() {
        let mut vec = OffsetVec::new(3);
        assert_eq!(vec.get_max_size(), 3);

        assert_eq!(vec.get_elem_count(), 0);

        vec.push(1);
        vec.push(2);
        vec.push(3);
        assert_eq!(vec.get_elem_count(), 3);
        assert_eq!(vec.get(0), Some(&1));
        assert_eq!(vec.get(1), Some(&2));
        assert_eq!(vec.get(2), Some(&3));

        vec.push(4);
        assert_eq!(vec.get_elem_count(), 3);
        assert_eq!(vec.get(0), Some(&2));
        assert_eq!(vec.get(1), Some(&3));
        assert_eq!(vec.get(2), Some(&4));

        vec.push(5);
        assert_eq!(vec.get_elem_count(), 3);
        assert_eq!(vec.get(0), Some(&3));
        assert_eq!(vec.get(1), Some(&4));
        assert_eq!(vec.get(2), Some(&5));
    }

    #[test]
    fn test_offset_vec_iterator() {
        let mut vec = OffsetVec::new(3);

        vec.push(1);
        vec.push(2);
        vec.push(3);
        let mut iter = vec.into_iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);

        vec.push(4);
        vec.push(5);
        let mut iter = vec.into_iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), Some(&5));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_offset_vec_to_vec() {
        let mut vec = OffsetVec::new(3);
        assert_eq!(vec.to_vec(), vec![]);

        vec.push(1);
        assert_eq!(vec.to_vec(), vec![1]);

        vec.push(2);
        assert_eq!(vec.to_vec(), vec![1, 2]);

        vec.push(3);
        assert_eq!(vec.to_vec(), vec![1, 2, 3]);

        vec.push(4);
        vec.push(5);
        assert_eq!(vec.to_vec(), vec![3, 4, 5]);
    }
}
