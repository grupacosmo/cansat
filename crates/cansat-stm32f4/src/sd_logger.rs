use crate::{SdmmcController, SdmmcError};
use core::mem::ManuallyDrop;
use embedded_sdmmc::{File, Volume, VolumeIdx};

pub struct SdLogger {
    file: ManuallyDrop<File>,
    volume: Volume,
    controller: SdmmcController,
}

#[cfg(debug_assertions)]
const FILENAME: &str = "debug.log";

#[cfg(not(debug_assertions))]
const FILENAME: &str = "release.log";

impl SdLogger {
    pub fn new(mut controller: SdmmcController) -> Result<Self, SdmmcError> {
        let mut volume = controller.get_volume(VolumeIdx(0))?;
        let root_dir = controller.open_root_dir(&volume)?;
        let file = controller.open_file_in_dir(
            &mut volume,
            &root_dir,
            FILENAME,
            embedded_sdmmc::Mode::ReadWriteCreateOrAppend,
        )?;
        controller.close_dir(&volume, root_dir);
        Ok(Self {
            controller,
            volume,
            file: ManuallyDrop::new(file),
        })
    }

    pub fn write(&mut self, data: &[u8]) -> Result<usize, SdmmcError> {
        self.controller
            .write(&mut self.volume, &mut self.file, data)
    }
}

impl Drop for SdLogger {
    fn drop(&mut self) {
        #[allow(unsafe_code)]
        let file = unsafe { ManuallyDrop::take(&mut self.file) };
        self.controller
            .close_file(&self.volume, file)
            .expect("Failed to close the log file");
    }
}
