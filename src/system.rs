use crate::error::TlpmError;
use crate::{PowerMeter, VI_FALSE, VI_TRUE, sys};
use std::ffi::{CStr, CString};

impl PowerMeter {
    /// Retrieve the instrument's identification information.
    ///
    /// # Returns
    ///
    /// A tuple containing the manufacturer, device name, serial number, and firmware revision.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn identification_query(&self) -> Result<(String, String, String, String), TlpmError> {
        let mut manufacturer = [0i8; sys::TLPM_BUFFER_SIZE as usize];
        let mut device_name = [0i8; sys::TLPM_BUFFER_SIZE as usize];
        let mut serial_number = [0i8; sys::TLPM_BUFFER_SIZE as usize];
        let mut firmware = [0i8; sys::TLPM_BUFFER_SIZE as usize];

        self.check_status(
            unsafe {
                sys::TLPMX_identificationQuery(
                    self.session,
                    manufacturer.as_mut_ptr(),
                    device_name.as_mut_ptr(),
                    serial_number.as_mut_ptr(),
                    firmware.as_mut_ptr(),
                )
            },
            "identification_query",
        )?;

        // helper closure to safely convert the populated c style arrays into owned rust strings
        let to_string = |buf: &[i8]| -> String {
            unsafe { CStr::from_ptr(buf.as_ptr()) }
                .to_string_lossy()
                .into_owned()
        };

        Ok((
            to_string(&manufacturer),
            to_string(&device_name),
            to_string(&serial_number),
            to_string(&firmware),
        ))
    }

    impl_property!(
        numeric,
        global,
        set_disp_brightness,
        get_disp_brightness,
        TLPMX_setDispBrightness,
        TLPMX_getDispBrightness,
        f64,
        "display brightness"
    );

    impl_property!(
        numeric,
        global,
        set_disp_contrast,
        get_disp_contrast,
        TLPMX_setDispContrast,
        TLPMX_getDispContrast,
        f64,
        "display contrast"
    );

    impl_property!(
        numeric,
        global,
        set_line_frequency,
        get_line_frequency,
        TLPMX_setLineFrequency,
        TLPMX_getLineFrequency,
        i16,
        "line frequency"
    );

    impl_property!(
        bool,
        global,
        set_summertime,
        get_summertime,
        TLPMX_setSummertime,
        TLPMX_getSummertime,
        "summertime (daylight saving) state"
    );

    impl_property!(
        string,
        global,
        set_display_name,
        get_display_name,
        TLPMX_setDisplayName,
        TLPMX_getDisplayName,
        "display name"
    );

    /// Set the system time of the device.
    ///
    /// # Arguments
    ///
    /// * `year` - The year (e.g., 2026).
    /// * `month` - The month (1-12).
    /// * `day` - The day (1-31).
    /// * `hour` - The hour (0-23).
    /// * `minute` - The minute (0-59).
    /// * `second` - The second (0-59).
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn set_time(
        &self,
        year: i16,
        month: i16,
        day: i16,
        hour: i16,
        minute: i16,
        second: i16,
    ) -> Result<(), TlpmError> {
        tracing::debug!("setting system time");
        self.check_status(
            unsafe { sys::TLPMX_setTime(self.session, year, month, day, hour, minute, second) },
            "set_time",
        )
    }

    /// Retrieve the system time of the device.
    ///
    /// # Returns
    ///
    /// A tuple containing `(year, month, day, hour, minute, second)`.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn get_time(&self) -> Result<(i16, i16, i16, i16, i16, i16), TlpmError> {
        tracing::debug!("getting system time");
        let mut year: i16 = 0;
        let mut month: i16 = 0;
        let mut day: i16 = 0;
        let mut hour: i16 = 0;
        let mut minute: i16 = 0;
        let mut second: i16 = 0;

        self.check_status(
            unsafe {
                sys::TLPMX_getTime(
                    self.session,
                    &mut year,
                    &mut month,
                    &mut day,
                    &mut hour,
                    &mut minute,
                    &mut second,
                )
            },
            "get_time",
        )?;

        Ok((year, month, day, hour, minute, second))
    }

    /// Retrieve the instrument's battery voltage.
    ///
    /// # Returns
    ///
    /// The battery voltage in Volts.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn get_battery_voltage(&self) -> Result<f64, TlpmError> {
        tracing::debug!("getting battery voltage");
        let mut voltage: f64 = 0.0;
        self.check_status(
            unsafe { sys::TLPMX_getBatteryVoltage(self.session, &mut voltage) },
            "get_battery_voltage",
        )?;
        Ok(voltage)
    }

    /// Produce a beep sound from the device.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn beep(&self) -> Result<(), TlpmError> {
        tracing::debug!("triggering device beep");
        self.check_status(unsafe { sys::TLPMX_beep(self.session) }, "beep")
    }

    /// Set the device encryption configuration.
    pub fn set_encryption(
        &self,
        old_password: &str,
        new_password: &str,
        encryption_enabled: bool,
    ) -> Result<(), TlpmError> {
        tracing::debug!("setting encryption state");
        let c_old = CString::new(old_password)
            .map_err(|_| TlpmError::StringConversion("invalid old password".to_string()))?;
        let c_new = CString::new(new_password)
            .map_err(|_| TlpmError::StringConversion("invalid new password".to_string()))?;
        let c_enc = if encryption_enabled {
            VI_TRUE
        } else {
            VI_FALSE
        };

        self.check_status(
            unsafe {
                sys::TLPMX_setEncryption(
                    self.session,
                    c_old.as_ptr() as *mut _,
                    c_new.as_ptr() as *mut _,
                    c_enc,
                )
            },
            "set_encryption",
        )
    }

    /// Retrieve the current encryption configuration.
    ///
    /// # Returns
    /// A tuple containing `(password_string, encryption_enabled)`.
    pub fn get_encryption(&self) -> Result<(String, bool), TlpmError> {
        tracing::debug!("getting encryption state");
        let mut password = [0i8; sys::TLPM_BUFFER_SIZE as usize];
        let mut is_encrypted: sys::ViBoolean = 0;

        self.check_status(
            unsafe {
                sys::TLPMX_getEncryption(self.session, password.as_mut_ptr(), &mut is_encrypted)
            },
            "get_encryption",
        )?;

        let c_str = unsafe { CStr::from_ptr(password.as_ptr()) };
        Ok((
            c_str.to_string_lossy().into_owned(),
            is_encrypted == VI_TRUE,
        ))
    }
}
