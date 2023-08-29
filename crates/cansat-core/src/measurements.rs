use crate::nmea::NmeaGga;
use crate::quantity::{Distance, Pressure, Temperature};

use serde::{de, Deserializer, Serialize};

#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct Measurements {
    #[serde(serialize_with = "option_temperature_celsius")]
    #[serde(deserialize_with = "f32_as_optional_temperature_in_celcius")]
    pub temperature: Option<Temperature>,

    #[serde(serialize_with = "option_pressure_pascals")]
    #[serde(deserialize_with = "f32_as_optional_pressure_in_pascals")]
    pub pressure: Option<Pressure>,

    #[serde(serialize_with = "option_distance_meters")]
    #[serde(deserialize_with = "f32_as_optional_distance_in_meters")]
    pub altitude: Option<Distance>,

    pub nmea: Option<NmeaGga>,

    #[serde(serialize_with = "option_tuple_f32x3")]
    pub acceleration: Option<(f32, f32, f32)>,

    #[serde(serialize_with = "option_tuple_f32x3")]
    pub gyro: Option<(f32, f32, f32)>,

    #[serde(serialize_with = "option_tuple_f32x2")]
    pub rollpitch: Option<(f32, f32)>,
}

fn option_temperature_celsius<S>(v: &Option<Temperature>, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    v.map(|v| v.as_celsius()).serialize(s)
}

fn f32_as_optional_temperature_in_celcius<'de, D>(
    deserializer: D,
) -> Result<Option<Temperature>, D::Error>
where
    D: Deserializer<'de>,
{
    let temperature: Option<f32> = de::Deserialize::deserialize(deserializer)?;
    Ok(temperature.map(Temperature::from_celsius))
}

fn option_pressure_pascals<S>(v: &Option<Pressure>, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    v.map(|v| v.as_pascals()).serialize(s)
}

fn f32_as_optional_pressure_in_pascals<'de, D>(
    deserializer: D,
) -> Result<Option<Pressure>, D::Error>
where
    D: Deserializer<'de>,
{
    let pressure: Option<f32> = de::Deserialize::deserialize(deserializer)?;
    Ok(pressure.map(Pressure::from_pascals))
}

fn option_distance_meters<S>(v: &Option<Distance>, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    v.map(|v| v.as_meters()).serialize(s)
}

fn f32_as_optional_distance_in_meters<'de, D>(deserializer: D) -> Result<Option<Distance>, D::Error>
where
    D: Deserializer<'de>,
{
    let distance: Option<f32> = de::Deserialize::deserialize(deserializer)?;
    Ok(distance.map(Distance::from_meters))
}

fn option_tuple_f32x2<S>(v: &Option<(f32, f32)>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match v {
        Some((x, y)) => (x, y).serialize(serializer),
        None => ((), ()).serialize(serializer),
    }
}

fn option_tuple_f32x3<S>(v: &Option<(f32, f32, f32)>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match v {
        Some((x, y, z)) => (x, y, z).serialize(serializer),
        None => ((), ()).serialize(serializer),
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for Measurements {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "temp: {}, pres: {}, alt: {}, nmea: {}, acc: {}, gyro: {}, rollpitch: {}",
            OrError(&self.temperature.map(Celsius)),
            OrError(&self.pressure.map(HectoPascals)),
            OrError(&self.altitude.map(Meters)),
            OrError(&self.nmea),
            OrError(&self.acceleration),
            OrError(&self.gyro),
            OrError(&self.rollpitch),
        );
    }
}

struct Celsius(pub Temperature);

#[cfg(feature = "defmt")]
impl defmt::Format for Celsius {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "{} °C", self.0.as_celsius());
    }
}

struct HectoPascals(pub Pressure);

#[cfg(feature = "defmt")]
impl defmt::Format for HectoPascals {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "{} hPa", self.0.as_hectos());
    }
}

struct Meters(pub Distance);

#[cfg(feature = "defmt")]
impl defmt::Format for Meters {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "{} m", self.0.as_meters());
    }
}

struct Ascii<'a>(pub &'a [u8]);

#[cfg(feature = "defmt")]
impl<'a> defmt::Format for Ascii<'a> {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "{=[u8]:a}", self.0);
    }
}

struct OrError<'a, T>(pub &'a Option<T>);

#[cfg(feature = "defmt")]
impl<'a, T: defmt::Format> defmt::Format for OrError<'a, T> {
    fn format(&self, fmt: defmt::Formatter) {
        if let Some(v) = self.0 {
            defmt::write!(fmt, "{}", v);
        } else {
            defmt::write!(fmt, "ERROR");
        }
    }
}
