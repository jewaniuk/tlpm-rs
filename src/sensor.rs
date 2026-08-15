use crate::error::TlpmError;
use crate::{PowerMeter, sys};
use std::ffi::CStr;

impl PowerMeter {
    /// Retrieve comprehensive sensor information using the extended 32-bit flags.
    ///
    /// # Returns
    /// A tuple containing `(name, serial_number, message, type, subtype, flags)`.
    pub fn get_sensor_info(
        &self,
        channel: u16,
    ) -> Result<(String, String, String, i16, i16, i32), TlpmError> {
        tracing::debug!("getting sensor info on channel {}", channel);
        let mut name = [0i8; sys::TLPM_BUFFER_SIZE as usize];
        let mut snr = [0i8; sys::TLPM_BUFFER_SIZE as usize];
        let mut message = [0i8; sys::TLPM_BUFFER_SIZE as usize];
        let mut p_type: i16 = 0;
        let mut p_stype: i16 = 0;
        let mut p_flags: i32 = 0;

        self.check_status(
            unsafe {
                sys::TLPMX_getSensorInfoExt(
                    self.session,
                    name.as_mut_ptr(),
                    snr.as_mut_ptr(),
                    message.as_mut_ptr(),
                    &mut p_type,
                    &mut p_stype,
                    &mut p_flags,
                    channel,
                )
            },
            "get_sensor_info",
        )?;

        let to_string = |buf: &[i8]| -> String {
            unsafe { CStr::from_ptr(buf.as_ptr()) }
                .to_string_lossy()
                .into_owned()
        };

        Ok((
            to_string(&name),
            to_string(&snr),
            to_string(&message),
            p_type,
            p_stype,
            p_flags,
        ))
    }

    /// Retrieve the number of channels available on the connected instrument.
    pub fn get_channels(&self) -> Result<u16, TlpmError> {
        tracing::debug!("getting available channel count");
        let mut channel_count: u16 = 0;
        self.check_status(
            unsafe { sys::TLPMX_getChannels(self.session, &mut channel_count) },
            "get_channels",
        )?;
        Ok(channel_count)
    }

    /// Retrieve the calibration message for the sensor.
    pub fn get_calibration_msg(&self, channel: u16) -> Result<String, TlpmError> {
        tracing::debug!("getting calibration message on channel {}", channel);
        let mut message = [0i8; sys::TLPM_BUFFER_SIZE as usize];
        self.check_status(
            unsafe { sys::TLPMX_getCalibrationMsg(self.session, message.as_mut_ptr(), channel) },
            "get_calibration_msg",
        )?;
        let c_str = unsafe { CStr::from_ptr(message.as_ptr()) };
        Ok(c_str.to_string_lossy().into_owned())
    }
}
