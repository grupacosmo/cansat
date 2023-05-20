use crate::LoraError;

#[derive(Debug, derive_more::From)]
pub enum Error {
    CriticalDevice,
    #[from]
    Lora(LoraError),
    Response(i8)
}

impl defmt::Format for Error {
    fn format(&self, fmt: defmt::Formatter) {
        match self {
            Self::CriticalDevice => {
                defmt::write!(fmt, "Failed to initialize a critical peripheral device")
            }
            Self::Lora(e) => defmt::write!(fmt, "Lora Error: {}", &e),
            Self::Response(ec) => defmt::write!(fmt, "Received ERROR({}) response from LoRa", ec)
        }
    }
}