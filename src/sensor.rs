use crate::error::TlpmError;
use crate::{PowerMeter, VI_TRUE, sys};
use std::ffi::CStr;

impl PowerMeter {
    /// Retrieve comprehensive sensor information using the extended 32-bit flags.
    ///
    /// # Arguments
    ///
    /// * `channel` - The sensor channel (typically `1`).
    ///
    /// # Returns
    ///
    /// A tuple containing `(name, serial_number, message, type, subtype, flags)`.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
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
    ///
    /// # Returns
    ///
    /// The number of available measurement channels.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
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
    ///
    /// # Arguments
    ///
    /// * `channel` - The sensor channel (typically `1`).
    ///
    /// # Returns
    ///
    /// The calibration message string.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
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

    /// Check if a specific sensor is connected and operable.
    ///
    /// # Arguments
    ///
    /// * `channel` - The sensor channel (typically `1`).
    ///
    /// # Returns
    ///
    /// `true` if the sensor is connected, `false` otherwise.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn is_sensor_connected(&self, channel: u16) -> Result<bool, TlpmError> {
        tracing::debug!("checking if sensor is connected on channel {}", channel);
        let mut connected: sys::ViUInt16 = 0;
        self.check_status(
            unsafe { sys::TLPMX_isSensorConnected(self.session, &mut connected, channel) },
            "is_sensor_connected",
        )?;
        Ok(connected != 0)
    }

    /// Check if the External Measurement Module (EMM) is connected.
    ///
    /// # Returns
    ///
    /// `true` if the EMM is connected, `false` otherwise.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn is_emm_connected(&self) -> Result<bool, TlpmError> {
        tracing::debug!("checking if EMM is connected");
        let mut connected: sys::ViUInt16 = 0;
        self.check_status(
            unsafe { sys::TLPMX_isEmmConnected(self.session, &mut connected) },
            "is_emm_connected",
        )?;
        Ok(connected != 0)
    }

    /// Check if an external NTC thermistor is connected.
    ///
    /// # Arguments
    ///
    /// * `channel` - The sensor channel (typically `1`).
    ///
    /// # Returns
    ///
    /// `true` if the NTC is connected, `false` otherwise.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn is_ext_ntc_connected(&self, channel: u16) -> Result<bool, TlpmError> {
        tracing::debug!(
            "checking if external NTC is connected on channel {}",
            channel
        );
        let mut connected: sys::ViUInt16 = 0;
        self.check_status(
            unsafe { sys::TLPMX_isExtNtcConnected(self.session, &mut connected, channel) },
            "is_ext_ntc_connected",
        )?;
        Ok(connected != 0)
    }

    /// Read the state of the hardware shutter interlock.
    ///
    /// # Returns
    ///
    /// `true` if the interlock is closed/active, `false` otherwise.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn get_shutter_interlock(&self) -> Result<bool, TlpmError> {
        tracing::debug!("getting shutter interlock state");
        let mut closed: sys::ViBoolean = 0;
        self.check_status(
            unsafe { sys::TLPMX_getShutterInterlock(self.session, &mut closed) },
            "get_shutter_interlock",
        )?;
        Ok(closed == VI_TRUE)
    }

    impl_property!(
        numeric,
        channel,
        set_input_adapter_type,
        get_input_adapter_type,
        TLPMX_setInputAdapterType,
        TLPMX_getInputAdapterType,
        i16,
        "input adapter type"
    );

    /// Check if the Peak Detector is currently running.
    ///
    /// # Arguments
    ///
    /// * `channel` - The sensor channel (typically `1`).
    ///
    /// # Returns
    ///
    /// `true` if the peak detector is running, `false` otherwise.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn is_peak_detector_running(&self, channel: u16) -> Result<bool, TlpmError> {
        tracing::debug!(
            "checking if peak detector is running on channel {}",
            channel
        );
        let mut is_running: sys::ViUInt16 = 0;
        self.check_status(
            unsafe { sys::TLPMX_isPeakDetectorRunning(self.session, &mut is_running, channel) },
            "is_peak_detector_running",
        )?;
        Ok(is_running != 0)
    }

    /// Retrieve standard 16-bit sensor information.
    ///
    /// Note: `get_sensor_info` (which calls `TLPMX_getSensorInfoExt` under the hood)
    /// is generally preferred as it returns the full 32-bit flag data.
    ///
    /// # Arguments
    ///
    /// * `channel` - The sensor channel (typically `1`).
    ///
    /// # Returns
    ///
    /// A tuple containing `(name, serial_number, message, type, subtype, flags)`.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn get_sensor_info_standard(
        &self,
        channel: u16,
    ) -> Result<(String, String, String, i16, i16, i16), TlpmError> {
        tracing::debug!("getting standard sensor info on channel {}", channel);
        let mut name = [0i8; sys::TLPM_BUFFER_SIZE as usize];
        let mut snr = [0i8; sys::TLPM_BUFFER_SIZE as usize];
        let mut message = [0i8; sys::TLPM_BUFFER_SIZE as usize];
        let mut p_type: i16 = 0;
        let mut p_stype: i16 = 0;
        let mut p_flags: i16 = 0;

        self.check_status(
            unsafe {
                sys::TLPMX_getSensorInfo(
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
            "get_sensor_info_standard",
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
}
