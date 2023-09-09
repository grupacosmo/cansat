use cansat_core::nmea::NmeaGga;
use derive_new::new;
use std::collections::VecDeque;

pub const DEFAULT_CAPACITY: usize = 20;

pub struct Data {
    data_points: usize,
    data_records: VecDeque<DataRecord>,
    signal_strength: SignalStrength,
}

#[derive(new, Debug)]
pub struct DataRecord {
    pub time: f64,
    pub bme: BmeData,
    pub orientation: Orientation,
    pub acceleration: Acceleration,
    pub rollpitch: RollPitch,
    pub nmea: Option<NmeaGga>,
}

#[derive(new, Debug)]
pub struct SignalStrength {
    pub rssi: i32,
    pub snr: i32,
}

#[derive(new, Debug)]
pub struct BmeData {
    pub temperature: Option<f64>,
    pub pressure: Option<f64>,
    pub height: Option<f64>,
}

#[derive(new, Debug)]
pub struct Orientation {
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub z: Option<f64>,
}

#[derive(new, Debug)]
pub struct Acceleration {
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub z: Option<f64>,
}

#[derive(new, Debug)]
pub struct RollPitch {
    pub roll: Option<f64>,
    pub pitch: Option<f64>,
}

impl Data {
    pub fn new(data_points: usize) -> Self {
        Self {
            data_points,
            data_records: VecDeque::with_capacity(data_points),
            signal_strength: SignalStrength::new(0, 0),
        }
    }

    pub fn push(&mut self, data_record: DataRecord) {
        self.data_records.push_back(data_record);
        if self.data_records.len() > self.data_points {
            self.data_records.pop_front();
        }
    }

    pub fn last_data(&self) -> Option<&DataRecord> {
        self.data_records.back()
    }

    pub fn nth_element(&self, n: usize) -> Option<&DataRecord> {
        self.data_records.get(n)
    }

    pub fn data_iter(&self) -> impl Iterator<Item = &DataRecord> {
        self.data_records.iter()
    }

    pub fn data_windows(&self) -> impl Iterator<Item = [&DataRecord; 2]> {
        self.data_records
            .iter()
            .zip(self.data_records.iter().skip(1))
            .map(|t| [t.0, t.1])
    }

    pub fn set_signal_strength(&mut self, signal_strength: SignalStrength) {
        self.signal_strength = signal_strength;
    }

    pub fn signal_strength(&self) -> &SignalStrength {
        &self.signal_strength
    }
}

impl Default for Data {
    fn default() -> Self {
        Self::new(DEFAULT_CAPACITY)
    }
}

impl Orientation {
    pub fn new_none() -> Self {
        Self::new(None, None, None)
    }

    pub fn new_some(x: f64, y: f64, z: f64) -> Self {
        Self::new(Some(x), Some(y), Some(z))
    }

    pub fn unwrap_or_nan(&self) -> (f64, f64, f64) {
        (
            self.x.unwrap_or(f64::NAN),
            self.y.unwrap_or(f64::NAN),
            self.z.unwrap_or(f64::NAN),
        )
    }
}

impl Acceleration {
    pub fn new_none() -> Self {
        Self::new(None, None, None)
    }

    pub fn new_some(x: f64, y: f64, z: f64) -> Self {
        Self::new(Some(x), Some(y), Some(z))
    }

    pub fn unwrap_or_nan(&self) -> (f64, f64, f64) {
        (
            self.x.unwrap_or(f64::NAN),
            self.y.unwrap_or(f64::NAN),
            self.z.unwrap_or(f64::NAN),
        )
    }
}

impl RollPitch {
    pub fn new_none() -> Self {
        Self::new(None, None)
    }

    pub fn new_some(roll: f64, pitch: f64) -> Self {
        Self::new(Some(roll), Some(pitch))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn new_test_data(test_value: f64) -> DataRecord {
        DataRecord::new(
            0.0,
            BmeData::new(Some(test_value), Some(test_value), Some(test_value)),
            Orientation::new(Some(test_value), Some(test_value), Some(test_value)),
            Acceleration::new(Some(test_value), Some(test_value), Some(test_value)),
        )
    }

    fn test_data_getter(data_record: &DataRecord) -> Option<f64> {
        data_record.bme.temperature
    }

    fn test_data_getter_arr(data_record: [&DataRecord; 2]) -> [Option<f64>; 2] {
        [
            data_record[0].bme.temperature,
            data_record[1].bme.temperature,
        ]
    }

    #[test]
    fn test_data_nth_element() {
        let mut data = Data::new(3);

        data.push(new_test_data(1.0));
        data.push(new_test_data(2.0));
        data.push(new_test_data(3.0));
        assert_eq!(test_data_getter(&data.nth_element(0).unwrap()), Some(1.0));
        assert_eq!(test_data_getter(&data.nth_element(1).unwrap()), Some(2.0));
        assert_eq!(test_data_getter(&data.nth_element(2).unwrap()), Some(3.0));

        data.push(new_test_data(4.0));
        assert_eq!(test_data_getter(&data.nth_element(0).unwrap()), Some(2.0));
        assert_eq!(test_data_getter(&data.nth_element(1).unwrap()), Some(3.0));
        assert_eq!(test_data_getter(&data.nth_element(2).unwrap()), Some(4.0));

        data.push(new_test_data(5.0));
        assert_eq!(test_data_getter(&data.nth_element(0).unwrap()), Some(3.0));
        assert_eq!(test_data_getter(&data.nth_element(1).unwrap()), Some(4.0));
        assert_eq!(test_data_getter(&data.nth_element(2).unwrap()), Some(5.0));
    }

    #[test]
    fn test_data_iter() {
        let mut data = Data::new(3);

        data.push(new_test_data(1.0));
        data.push(new_test_data(2.0));
        data.push(new_test_data(3.0));
        {
            let mut iter = data.data_iter();
            assert_eq!(iter.next().map(test_data_getter), Some(Some(1.0)));
            assert_eq!(iter.next().map(test_data_getter), Some(Some(2.0)));
            assert_eq!(iter.next().map(test_data_getter), Some(Some(3.0)));
            assert_eq!(iter.next().map(test_data_getter), None);
        }
        {
            let mut iter = data.data_windows();
            assert_eq!(
                iter.next().map(test_data_getter_arr),
                Some([Some(1.0), Some(2.0)])
            );
            assert_eq!(
                iter.next().map(test_data_getter_arr),
                Some([Some(2.0), Some(3.0)])
            );
            assert_eq!(iter.next().map(test_data_getter_arr), None);
        }

        data.push(new_test_data(4.0));
        data.push(new_test_data(5.0));
        let mut iter = data.data_iter();
        assert_eq!(iter.next().map(test_data_getter), Some(Some(3.0)));
        assert_eq!(iter.next().map(test_data_getter), Some(Some(4.0)));
        assert_eq!(iter.next().map(test_data_getter), Some(Some(5.0)));
        assert_eq!(iter.next().map(test_data_getter), None);

        let mut iter = data.data_windows();
        assert_eq!(
            iter.next().map(test_data_getter_arr),
            Some([Some(3.0), Some(4.0)])
        );
        assert_eq!(
            iter.next().map(test_data_getter_arr),
            Some([Some(4.0), Some(5.0)])
        );
        assert_eq!(iter.next().map(test_data_getter_arr), None);
    }
}
