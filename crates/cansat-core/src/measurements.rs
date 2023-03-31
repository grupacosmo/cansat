use crate::quantity::{Distance, Pressure, Temperature};
use accelerometer::{vector, Orientation};
use heapless::Vec;
use serde::Serialize;

#[derive(Default, serde::Serialize)]
pub struct Measurements {
    #[serde(serialize_with = "option_temperature_celsius")]
    pub temperature: Option<Temperature>,

    #[serde(serialize_with = "option_pressure_pascals")]
    pub pressure: Option<Pressure>,

    #[serde(serialize_with = "option_distance_meters")]
    pub altitude: Option<Distance>,

    pub nmea: Option<Vec<u8, 82>>,

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
    v.map(|v| (v.x, v.y, v.z)).serialize(serializer)
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
