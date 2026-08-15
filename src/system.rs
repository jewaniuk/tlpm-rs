use crate::error::TlpmError;
use crate::{PowerMeter, VI_FALSE, VI_TRUE, sys};
use std::ffi::{CStr, CString};

impl PowerMeter {
    /// Retrieve the instrument's identification information.
    ///
    /// # Returns
    ///
    /// A tuple containing `(manufacturer, device_name, serial_number, firmware_revision)`.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn identification_query(&self) -> Result<(String, String, String, String), TlpmError> {
        tracing::debug!("querying instrument identification");
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
    ///
    /// # Arguments
    ///
    /// * `old_password` - The current password string.
    /// * `new_password` - The new password string to set.
    /// * `encryption_enabled` - `true` to enable encryption, `false` to disable.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::StringConversion` if either password contains a null byte,
    /// or a `TlpmError::VisaError` if the device responds with an error code.
    pub fn set_encryption(
        &self,
        old_password: &str,
        new_password: &str,
        encryption_enabled: bool,
    ) -> Result<(), TlpmError> {
        tracing::debug!("setting encryption state to {}", encryption_enabled);
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
    ///
    /// A tuple containing `(password_string, encryption_enabled)`.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
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

    /// Retrieve the current instrument error count.
    ///
    /// # Returns
    ///
    /// The number of errors currently in the instrument's error queue.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn error_count(&self) -> Result<u32, TlpmError> {
        tracing::debug!("getting error count");
        let mut count: u32 = 0;
        self.check_status(
            unsafe { sys::TLPMX_errorCount(self.session, &mut count) },
            "error_count",
        )?;
        Ok(count)
    }

    /// Read the oldest error from the instrument's error queue.
    ///
    /// # Returns
    ///
    /// A tuple containing `(error_code, error_message)`.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn error_query(&self) -> Result<(i32, String), TlpmError> {
        tracing::debug!("querying instrument error queue");
        let mut err_num: i32 = 0;
        let mut buffer = [0i8; sys::TLPM_ERR_DESCR_BUFFER_SIZE as usize];
        self.check_status(
            unsafe { sys::TLPMX_errorQuery(self.session, &mut err_num, buffer.as_mut_ptr()) },
            "error_query",
        )?;
        let msg = unsafe { std::ffi::CStr::from_ptr(buffer.as_ptr()) }
            .to_string_lossy()
            .into_owned();
        Ok((err_num, msg))
    }

    /// Enable or disable the automatic error query mode.
    ///
    /// # Arguments
    ///
    /// * `mode` - `true` to enable automatic error queries, `false` to disable.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn error_query_mode(&self, mode: bool) -> Result<(), TlpmError> {
        tracing::debug!("setting error query mode to {}", mode);
        let c_mode = if mode { VI_TRUE } else { VI_FALSE };
        self.check_status(
            unsafe { sys::TLPMX_errorQueryMode(self.session, c_mode) },
            "error_query_mode",
        )
    }

    /// Export the current instrument settings as a JSON string.
    ///
    /// # Arguments
    ///
    /// * `max_size` - The maximum string buffer size to allocate for the JSON payload.
    ///
    /// # Returns
    ///
    /// A JSON string containing the instrument configuration.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn export_settings_as_json(&self, max_size: u32) -> Result<String, TlpmError> {
        tracing::debug!("exporting settings as JSON (max size: {})", max_size);
        let mut buffer = vec![0i8; max_size as usize];
        self.check_status(
            unsafe { sys::TLPMX_exportSettingsAsJson(self.session, buffer.as_mut_ptr(), max_size) },
            "export_settings_as_json",
        )?;
        let msg = unsafe { std::ffi::CStr::from_ptr(buffer.as_ptr()) }
            .to_string_lossy()
            .into_owned();
        Ok(msg)
    }

    /// Import instrument settings from a JSON string.
    ///
    /// # Arguments
    ///
    /// * `adapt` - `true` to adapt the settings to the current sensor if they differ.
    /// * `json_settings` - The JSON configuration string.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::StringConversion` if the JSON string contains a null byte,
    /// or `TlpmError::VisaError` if the device responds with an error code.
    pub fn import_settings_from_json(
        &self,
        adapt: bool,
        json_settings: &str,
    ) -> Result<(), TlpmError> {
        tracing::debug!("importing settings from JSON (adapt: {})", adapt);
        let c_adapt = if adapt { VI_TRUE } else { VI_FALSE };
        let c_str = std::ffi::CString::new(json_settings)
            .map_err(|_| TlpmError::StringConversion("invalid JSON string".to_string()))?;
        self.check_status(
            unsafe {
                sys::TLPMX_importSettingsFromJson(self.session, c_adapt, c_str.as_ptr() as *mut _)
            },
            "import_settings_from_json",
        )
    }

    /// Send an NTP request to synchronize the instrument's time.
    ///
    /// # Arguments
    ///
    /// * `time_mode` - `true` for summertime, `false` for wintertime.
    /// * `time_zone` - The UTC timezone offset in hours.
    /// * `ip_address` - The IP address of the NTP server.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::StringConversion` for invalid IP strings,
    /// or `TlpmError::VisaError` if the device responds with an error code.
    pub fn send_ntp_request(
        &self,
        time_mode: bool,
        time_zone: i16,
        ip_address: &str,
    ) -> Result<(), TlpmError> {
        tracing::debug!(
            "sending NTP request to {} (tz offset: {})",
            ip_address,
            time_zone
        );
        let c_mode = if time_mode { VI_TRUE } else { VI_FALSE };
        let c_ip = std::ffi::CString::new(ip_address)
            .map_err(|_| TlpmError::StringConversion("invalid IP".to_string()))?;
        self.check_status(
            unsafe {
                sys::TLPMX_sendNTPRequest(self.session, c_mode, time_zone, c_ip.as_ptr() as *mut _)
            },
            "send_ntp_request",
        )
    }

    /// Execute the instrument's internal self-test routine.
    ///
    /// # Returns
    ///
    /// A tuple containing `(test_result_code, description)`. A code of 0 typically indicates success.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the communication fails during the test.
    pub fn self_test(&self) -> Result<(i16, String), TlpmError> {
        tracing::debug!("executing device self-test");
        let mut result: i16 = 0;
        let mut buffer = [0i8; sys::TLPM_BUFFER_SIZE as usize];
        self.check_status(
            unsafe { sys::TLPMX_selfTest(self.session, &mut result, buffer.as_mut_ptr()) },
            "self_test",
        )?;
        let msg = unsafe { std::ffi::CStr::from_ptr(buffer.as_ptr()) }
            .to_string_lossy()
            .into_owned();
        Ok((result, msg))
    }

    /// Query the driver and firmware revision strings.
    ///
    /// # Returns
    ///
    /// A tuple containing `(driver_revision, firmware_revision)`.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn revision_query(&self) -> Result<(String, String), TlpmError> {
        tracing::debug!("querying firmware and driver revisions");
        let mut drv_rev = [0i8; sys::TLPM_BUFFER_SIZE as usize];
        let mut fw_rev = [0i8; sys::TLPM_BUFFER_SIZE as usize];
        self.check_status(
            unsafe {
                sys::TLPMX_revisionQuery(self.session, drv_rev.as_mut_ptr(), fw_rev.as_mut_ptr())
            },
            "revision_query",
        )?;
        let d_str = unsafe { std::ffi::CStr::from_ptr(drv_rev.as_ptr()) }
            .to_string_lossy()
            .into_owned();
        let f_str = unsafe { std::ffi::CStr::from_ptr(fw_rev.as_ptr()) }
            .to_string_lossy()
            .into_owned();
        Ok((d_str, f_str))
    }
}
