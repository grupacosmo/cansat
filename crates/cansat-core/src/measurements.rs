use crate::nmea::NmeaGga;
use crate::quantity::{Distance, Pressure, Temperature};
use accelerometer::vector;

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

    #[serde(serialize_with = "option_vector_f32x3")]
    #[serde(deserialize_with = "tuple_as_vector_f32x3")]
    pub acceleration: Option<vector::F32x3>,

    #[serde(serialize_with = "option_vector_f32x3")]
    #[serde(deserialize_with = "tuple_as_vector_f32x3")]
    pub gyro: Option<vector::F32x3>,

    #[serde(serialize_with = "option_vector_f32x2")]
    #[serde(deserialize_with = "tuple_as_vector_f32x2")]
    pub rollpitch: Option<vector::F32x2>,
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

fn option_vector_f32x2<S>(v: &Option<vector::F32x2>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match v {
        Some(v) => (v.x, v.y).serialize(serializer),
        None => ((), ()).serialize(serializer),
    }
}

fn tuple_as_vector_f32x2<'de, D>(deserializer: D) -> Result<Option<vector::F32x2>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt: Option<(f32, f32)> = de::Deserialize::deserialize(deserializer)?;
    Ok(opt.map(|(x, y)| vector::F32x2::new(x, y)))
}

fn option_vector_f32x3<S>(v: &Option<vector::F32x3>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match v {
        Some(v) => (v.x, v.y, v.z).serialize(serializer),
        None => ((), ()).serialize(serializer),
    }
}

fn tuple_as_vector_f32x3<'de, D>(deserializer: D) -> Result<Option<vector::F32x3>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt: Option<(f32, f32, f32)> = de::Deserialize::deserialize(deserializer)?;
    Ok(opt.map(|(x, y, z)| vector::F32x3::new(x, y, z)))
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
            OrError(&self.acceleration.map(Vector3)),
            OrError(&self.gyro.map(Vector3)),
            OrError(&self.rollpitch.map(Vector2)),
        );
    }
}

struct Vector2(pub vector::F32x2);

#[cfg(feature = "defmt")]
impl defmt::Format for Vector2 {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "{}, {}", self.0.x, self.0.y);
    }
}

struct Vector3(pub vector::F32x3);

#[cfg(feature = "defmt")]
impl defmt::Format for Vector3 {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "{}, {}, {}", self.0.x, self.0.y, self.0.z);
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
