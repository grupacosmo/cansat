use derive_new::new;
use std::collections::VecDeque;

const MAX_DATA_POINTS: usize = 10;

pub struct Data {
    data_records: VecDeque<DataRecord>,
    signal_strength: SignalStrength,
}

#[derive(new)]
pub struct DataRecord {
    pub time: f64,
    pub bme: BmeData,
    pub orientation: Orientation,
    pub acceleration: Acceleration,
}

#[derive(new, Debug)]
pub struct SignalStrength {
    pub rssi: i32,
    pub snr: i32,
}

#[derive(new, Debug)]
pub struct BmeData {
    pub temperature: f64,
    pub pressure: f64,
    pub height: f64,
}

#[derive(new, Debug)]
pub struct Orientation {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(new, Debug)]
pub struct Acceleration {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Data {
    pub fn new() -> Self {
        Self {
            data_records: VecDeque::new(),
            signal_strength: SignalStrength::new(0, 0),
        }
    }

    pub fn push(&mut self, data_record: DataRecord) {
        self.data_records.push_back(data_record);
        if self.data_records.len() > MAX_DATA_POINTS {
            self.data_records.pop_front();
        }
    }

    pub fn get_last_data(&self) -> Option<&DataRecord> {
        self.data_records.back()
    }

    pub fn get_nth_element(&self, n: usize) -> Option<&DataRecord> {
        self.data_records.get(n)
    }

    pub fn get_data_iter(&self) -> impl Iterator<Item = &DataRecord> {
        self.data_records.iter()
    }

    pub fn set_signal_strength(&mut self, signal_strength: SignalStrength) {
        self.signal_strength = signal_strength;
    }

    pub fn get_signal_strength(&self) -> &SignalStrength {
        &self.signal_strength
    }
}

impl Default for Data {
    fn default() -> Self {
        Self::new()
    }
}
