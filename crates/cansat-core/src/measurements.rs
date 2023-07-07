use crate::nmea::NmeaGga;
use crate::quantity::{Distance, Pressure, Temperature};
use accelerometer::{vector, Orientation};
use serde::Serialize;

#[derive(Default, serde::Serialize)]
pub struct Measurements {
    #[serde(serialize_with = "option_temperature_celsius")]
    pub temperature: Option<Temperature>,

    #[serde(serialize_with = "option_pressure_pascals")]
    pub pressure: Option<Pressure>,

    #[serde(serialize_with = "option_distance_meters")]
    pub altitude: Option<Distance>,

    pub nmea: Option<NmeaGga>,

    #[serde(serialize_with = "option_vector_i16x3")]
    pub acceleration: Option<vector::I16x3>,

    #[serde(serialize_with = "option_orientation")]
    pub orientation: Option<Orientation>,
}

fn option_temperature_celsius<S>(v: &Option<Temperature>, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    v.map(|v| v.as_celsius()).serialize(s)
}

fn option_pressure_pascals<S>(v: &Option<Pressure>, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    v.map(|v| v.as_pascals()).serialize(s)
}

fn option_distance_meters<S>(v: &Option<Distance>, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    v.map(|v| v.as_meters()).serialize(s)
}

fn orientation_to_str(v: Orientation) -> &'static str {
    match v {
        Orientation::Unknown => "unknown",
        Orientation::PortraitUp => "portrait up",
        Orientation::PortraitDown => "portrait down",
        Orientation::LandscapeUp => "landscape up",
        Orientation::LandscapeDown => "landscape down",
        Orientation::FaceUp => "face up",
        Orientation::FaceDown => "face down",
    }
}

fn option_vector_i16x3<S>(v: &Option<vector::I16x3>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match v {
        Some(v) => (v.x, v.y, v.z).serialize(serializer),
        None => ((), (), ()).serialize(serializer),
    }
}

fn option_orientation<S>(
    v: &Option<accelerometer::Orientation>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    v.map(orientation_to_str).serialize(serializer)
}

#[cfg(feature = "defmt")]
impl defmt::Format for Measurements {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "temp: {}, pres: {}, alt: {}, accel: {}, orient: {}, nmea: {}",
            OrError(&self.temperature.map(Celsius)),
            OrError(&self.pressure.map(HectoPascals)),
            OrError(&self.altitude.map(Meters)),
            OrError(&self.acceleration.map(|v| (v.x, v.y, v.z))),
            OrError(&self.orientation.as_ref().map(defmt::Debug2Format)),
            OrError(&self.nmea)
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
