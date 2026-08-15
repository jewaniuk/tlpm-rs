use crate::enums::{AnalogRoute, DigIoPinMode, FanMode, FanTempSource, TlpmAttribute};
use crate::error::TlpmError;
use crate::{PowerMeter, VI_FALSE, VI_TRUE, sys};
use std::ffi::CStr;

impl PowerMeter {
    /// Set the fan control mode.
    ///
    /// # Arguments
    ///
    /// * `mode` - The `FanMode` to configure.
    /// * `channel` - The sensor channel (typically `1`).
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn set_fan_mode(&self, mode: FanMode, channel: u16) -> Result<(), TlpmError> {
        tracing::debug!("setting fan mode to {:?} on channel {}", mode, channel);
        self.check_status(
            unsafe { sys::TLPMX_setFanMode(self.session, mode as u16, channel) },
            "set_fan_mode",
        )
    }

    /// Read the currently configured fan control mode.
    ///
    /// # Arguments
    ///
    /// * `channel` - The sensor channel (typically `1`).
    ///
    /// # Returns
    ///
    /// The currently configured `FanMode`.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code,
    /// or `TlpmError::InvalidEnumValue` if an unrecognized mode code is returned.
    pub fn get_fan_mode(&self, channel: u16) -> Result<FanMode, TlpmError> {
        tracing::debug!("getting fan mode on channel {}", channel);
        let mut mode_code: u16 = 0;

        self.check_status(
            unsafe { sys::TLPMX_getFanMode(self.session, &mut mode_code, channel) },
            "get_fan_mode",
        )?;

        FanMode::try_from(mode_code)
    }

    /// Set the fan temperature control source.
    ///
    /// # Arguments
    ///
    /// * `source` - The `FanTempSource` to configure.
    /// * `channel` - The sensor channel (typically `1`).
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn set_fan_temperature_source(
        &self,
        source: FanTempSource,
        channel: u16,
    ) -> Result<(), TlpmError> {
        tracing::debug!(
            "setting fan temperature source to {:?} on channel {}",
            source,
            channel
        );
        self.check_status(
            unsafe { sys::TLPMX_setFanTemperatureSource(self.session, source as u16, channel) },
            "set_fan_temperature_source",
        )
    }

    /// Read the currently configured fan temperature control source.
    ///
    /// # Arguments
    ///
    /// * `channel` - The sensor channel (typically `1`).
    ///
    /// # Returns
    ///
    /// The currently configured `FanTempSource`.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code,
    /// or `TlpmError::InvalidEnumValue` if an unrecognized source code is returned.
    pub fn get_fan_temperature_source(&self, channel: u16) -> Result<FanTempSource, TlpmError> {
        tracing::debug!("getting fan temperature source on channel {}", channel);
        let mut source_code: u16 = 0;

        self.check_status(
            unsafe { sys::TLPMX_getFanTemperatureSource(self.session, &mut source_code, channel) },
            "get_fan_temperature_source",
        )?;

        FanTempSource::try_from(source_code)
    }

    /// Retrieve the current running state of the fan.
    ///
    /// # Arguments
    /// * `channel` - The sensor channel (typically `1`).
    ///
    /// # Returns
    /// `true` if the fan is running, `false` otherwise.
    pub fn get_fan_state(&self, channel: u16) -> Result<bool, TlpmError> {
        tracing::debug!("getting fan state on channel {}", channel);
        let mut is_running: sys::ViBoolean = 0;
        self.check_status(
            unsafe { sys::TLPMX_getFanState(self.session, &mut is_running, channel) },
            "get_fan_state",
        )?;
        Ok(is_running == VI_TRUE)
    }

    /// Set the fan voltage.
    ///
    /// # Arguments
    /// * `voltage` - The voltage to apply.
    /// * `channel` - The sensor channel (typically `1`).
    pub fn set_fan_voltage(&self, voltage: f64, channel: u16) -> Result<(), TlpmError> {
        tracing::debug!("setting fan voltage to {} on channel {}", voltage, channel);
        self.check_status(
            unsafe { sys::TLPMX_setFanVoltage(self.session, voltage, channel) },
            "set_fan_voltage",
        )
    }

    /// Retrieve the currently configured fan voltage.
    pub fn get_fan_voltage(&self, channel: u16) -> Result<f64, TlpmError> {
        tracing::debug!("getting fan voltage on channel {}", channel);
        let mut voltage: f64 = 0.0;
        self.check_status(
            unsafe { sys::TLPMX_getFanVoltage(self.session, &mut voltage, channel) },
            "get_fan_voltage",
        )?;
        Ok(voltage)
    }

    /// Set the maximum and target RPM for the fan.
    pub fn set_fan_rpm(
        &self,
        max_rpm: f64,
        target_rpm: f64,
        channel: u16,
    ) -> Result<(), TlpmError> {
        tracing::debug!("setting fan rpm on channel {}", channel);
        self.check_status(
            unsafe { sys::TLPMX_setFanRpm(self.session, max_rpm, target_rpm, channel) },
            "set_fan_rpm",
        )
    }

    /// Retrieve the maximum and target RPM for the fan.
    ///
    /// # Returns
    /// A tuple containing `(max_rpm, target_rpm)`.
    pub fn get_fan_rpm(&self, channel: u16) -> Result<(f64, f64), TlpmError> {
        tracing::debug!("getting fan rpm on channel {}", channel);
        let mut max_rpm: f64 = 0.0;
        let mut target_rpm: f64 = 0.0;
        self.check_status(
            unsafe { sys::TLPMX_getFanRpm(self.session, &mut max_rpm, &mut target_rpm, channel) },
            "get_fan_rpm",
        )?;
        Ok((max_rpm, target_rpm))
    }

    /// Retrieve the actual current RPM of the fan.
    pub fn get_act_fan_rpm(&self, channel: u16) -> Result<f64, TlpmError> {
        tracing::debug!("getting actual fan rpm on channel {}", channel);
        let mut rpm: f64 = 0.0;
        self.check_status(
            unsafe { sys::TLPMX_getActFanRpm(self.session, &mut rpm, channel) },
            "get_act_fan_rpm",
        )?;
        Ok(rpm)
    }

    /// Set the fan temperature adjustment parameters.
    pub fn set_fan_adjust_parameters(
        &self,
        voltage_min: f64,
        voltage_max: f64,
        temperature_min: f64,
        temperature_max: f64,
        channel: u16,
    ) -> Result<(), TlpmError> {
        tracing::debug!("setting fan adjust parameters on channel {}", channel);
        self.check_status(
            unsafe {
                sys::TLPMX_setFanAdjustParameters(
                    self.session,
                    voltage_min,
                    voltage_max,
                    temperature_min,
                    temperature_max,
                    channel,
                )
            },
            "set_fan_adjust_parameters",
        )
    }

    /// Retrieve the fan temperature adjustment parameters.
    ///
    /// # Returns
    /// A tuple containing `(voltage_min, voltage_max, temperature_min, temperature_max)`.
    pub fn get_fan_adjust_parameters(
        &self,
        channel: u16,
    ) -> Result<(f64, f64, f64, f64), TlpmError> {
        tracing::debug!("getting fan adjust parameters on channel {}", channel);
        let mut v_min: f64 = 0.0;
        let mut v_max: f64 = 0.0;
        let mut t_min: f64 = 0.0;
        let mut t_max: f64 = 0.0;
        self.check_status(
            unsafe {
                sys::TLPMX_getFanAdjustParameters(
                    self.session,
                    &mut v_min,
                    &mut v_max,
                    &mut t_min,
                    &mut t_max,
                    channel,
                )
            },
            "get_fan_adjust_parameters",
        )?;
        Ok((v_min, v_max, t_min, t_max))
    }

    /// Set the analog output routing strategy.
    ///
    /// # Arguments
    ///
    /// * `route` - The `AnalogRoute` to configure.
    /// * `channel` - The sensor channel (typically `1`).
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn set_analog_output_route(
        &self,
        route: AnalogRoute,
        channel: u16,
    ) -> Result<(), TlpmError> {
        tracing::debug!(
            "setting analog output route to {:?} on channel {}",
            route,
            channel
        );
        self.check_status(
            unsafe { sys::TLPMX_setAnalogOutputRoute(self.session, route as u16, channel) },
            "set_analog_output_route",
        )
    }

    /// Read the currently configured analog output route.
    ///
    /// # Arguments
    ///
    /// * `channel` - The sensor channel (typically `1`).
    ///
    /// # Returns
    ///
    /// A string representation of the active analog route.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn get_analog_output_route(&self, channel: u16) -> Result<String, TlpmError> {
        tracing::debug!("getting analog output route on channel {}", channel);
        let mut route_name = [0i8; sys::TLPM_BUFFER_SIZE as usize];

        self.check_status(
            unsafe {
                sys::TLPMX_getAnalogOutputRoute(self.session, route_name.as_mut_ptr(), channel)
            },
            "get_analog_output_route",
        )?;

        let c_str = unsafe { CStr::from_ptr(route_name.as_ptr()) };
        Ok(c_str.to_string_lossy().into_owned())
    }

    /// Set the digital I/O pin mode.
    ///
    /// # Arguments
    ///
    /// * `pin_number` - The pin number to configure.
    /// * `mode` - The `DigIoPinMode` to apply.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn set_dig_io_pin_mode(
        &self,
        pin_number: i16,
        mode: DigIoPinMode,
    ) -> Result<(), TlpmError> {
        tracing::debug!("setting digital io pin {} to mode {:?}", pin_number, mode);
        self.check_status(
            unsafe { sys::TLPMX_setDigIoPinMode(self.session, pin_number, mode as u16) },
            "set_dig_io_pin_mode",
        )
    }

    /// Read the currently configured digital I/O pin mode.
    ///
    /// # Arguments
    ///
    /// * `pin_number` - The pin number to query.
    ///
    /// # Returns
    ///
    /// The currently configured `DigIoPinMode`.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code,
    /// or `TlpmError::InvalidEnumValue` if an unrecognized mode code is returned.
    pub fn get_dig_io_pin_mode(&self, pin_number: i16) -> Result<DigIoPinMode, TlpmError> {
        tracing::debug!("getting digital io mode for pin {}", pin_number);
        let mut mode_code: u16 = 0;

        self.check_status(
            unsafe { sys::TLPMX_getDigIoPinMode(self.session, pin_number, &mut mode_code) },
            "get_dig_io_pin_mode",
        )?;

        DigIoPinMode::try_from(mode_code)
    }

    impl_property!(
        bool,
        global,
        set_shutter_position,
        get_shutter_position,
        TLPMX_setShutterPosition,
        TLPMX_getShutterPosition,
        "shutter position"
    );

    /// Set the laser state, including frequency and duration.
    pub fn set_laser_state(
        &self,
        state: bool,
        frequency: u32,
        duration: u32,
    ) -> Result<(), TlpmError> {
        tracing::debug!(
            "setting laser state to {} (freq: {}, dur: {})",
            state,
            frequency,
            duration
        );
        let c_state = if state { VI_TRUE } else { VI_FALSE };
        self.check_status(
            unsafe { sys::TLPMX_setLaserState(self.session, c_state, frequency, duration) },
            "set_laser_state",
        )
    }

    /// Retrieve the current boolean state of the laser.
    pub fn get_laser_state(&self) -> Result<bool, TlpmError> {
        tracing::debug!("getting laser state");
        let mut state: sys::ViBoolean = 0;
        self.check_status(
            unsafe { sys::TLPMX_getLaserState(self.session, &mut state) },
            "get_laser_state",
        )?;
        Ok(state == VI_TRUE)
    }

    impl_property!(
        numeric,
        attr_channel,
        set_analog_output_slope,
        get_analog_output_slope,
        TLPMX_setAnalogOutputSlope,
        TLPMX_getAnalogOutputSlope,
        f64,
        "analog output slope"
    );

    impl_property!(
        numeric,
        attr_channel,
        set_position_analog_output_slope,
        get_position_analog_output_slope,
        TLPMX_setPositionAnalogOutputSlope,
        TLPMX_getPositionAnalogOutputSlope,
        f64,
        "position analog output slope"
    );

    /// Set the analog logarithmic output configuration.
    ///
    /// # Arguments
    /// * `log_slope` - The slope of the logarithmic output.
    /// * `log_offset` - The offset of the logarithmic output.
    /// * `channel` - The sensor channel (typically `1`).
    pub fn set_analog_log_conf(
        &self,
        log_slope: f64,
        log_offset: f64,
        channel: u16,
    ) -> Result<(), TlpmError> {
        tracing::debug!(
            "setting analog log config (slope: {}, offset: {}) on channel {}",
            log_slope,
            log_offset,
            channel
        );
        self.check_status(
            unsafe { sys::TLPMX_setAnalogLogConf(self.session, log_slope, log_offset, channel) },
            "set_analog_log_conf",
        )
    }

    /// Read the currently configured analog logarithmic output settings.
    ///
    /// # Returns
    /// A tuple containing `(log_slope, log_offset)`.
    pub fn get_analog_log_conf(&self, channel: u16) -> Result<(f64, f64), TlpmError> {
        tracing::debug!("getting analog log config on channel {}", channel);
        let mut log_slope: f64 = 0.0;
        let mut log_offset: f64 = 0.0;
        self.check_status(
            unsafe {
                sys::TLPMX_getAnalogLogConf(self.session, &mut log_slope, &mut log_offset, channel)
            },
            "get_analog_log_conf",
        )?;
        Ok((log_slope, log_offset))
    }

    /// Set the Pass/Fail power window limits.
    pub fn set_pass_fail_power_window(
        &self,
        min: f64,
        max: f64,
        channel: u16,
    ) -> Result<(), TlpmError> {
        tracing::debug!("setting pass/fail power window on channel {}", channel);
        self.check_status(
            unsafe { sys::TLPMX_setPassFailPowerWindow(self.session, min, max, channel) },
            "set_pass_fail_power_window",
        )
    }

    /// Retrieve the Pass/Fail power window limits.
    pub fn get_pass_fail_power_window(&self, channel: u16) -> Result<(f64, f64), TlpmError> {
        let mut min: f64 = 0.0;
        let mut max: f64 = 0.0;
        self.check_status(
            unsafe { sys::TLPMX_getPassFailPowerWindow(self.session, &mut min, &mut max, channel) },
            "get_pass_fail_power_window",
        )?;
        Ok((min, max))
    }

    /// Set the Pass/Fail energy window limits.
    pub fn set_pass_fail_energy_window(
        &self,
        min: f64,
        max: f64,
        channel: u16,
    ) -> Result<(), TlpmError> {
        tracing::debug!("setting pass/fail energy window on channel {}", channel);
        self.check_status(
            unsafe { sys::TLPMX_setPassFailEnergyWindow(self.session, min, max, channel) },
            "set_pass_fail_energy_window",
        )
    }

    /// Retrieve the Pass/Fail energy window limits.
    pub fn get_pass_fail_energy_window(&self, channel: u16) -> Result<(f64, f64), TlpmError> {
        let mut min: f64 = 0.0;
        let mut max: f64 = 0.0;
        self.check_status(
            unsafe {
                sys::TLPMX_getPassFailEnergyWindow(self.session, &mut min, &mut max, channel)
            },
            "get_pass_fail_energy_window",
        )?;
        Ok((min, max))
    }
}
