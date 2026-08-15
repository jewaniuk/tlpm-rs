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
    ///
    /// * `channel` - The sensor channel (typically `1`).
    ///
    /// # Returns
    ///
    /// `true` if the fan is running, `false` otherwise.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
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
    ///
    /// * `voltage` - The voltage to apply.
    /// * `channel` - The sensor channel (typically `1`).
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn set_fan_voltage(&self, voltage: f64, channel: u16) -> Result<(), TlpmError> {
        tracing::debug!("setting fan voltage to {} on channel {}", voltage, channel);
        self.check_status(
            unsafe { sys::TLPMX_setFanVoltage(self.session, voltage, channel) },
            "set_fan_voltage",
        )
    }

    /// Retrieve the currently configured fan voltage.
    ///
    /// # Arguments
    ///
    /// * `channel` - The sensor channel (typically `1`).
    ///
    /// # Returns
    ///
    /// The currently configured fan voltage.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
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
    ///
    /// # Arguments
    ///
    /// * `max_rpm` - The maximum RPM value.
    /// * `target_rpm` - The target RPM value.
    /// * `channel` - The sensor channel (typically `1`).
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
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
    /// # Arguments
    ///
    /// * `channel` - The sensor channel (typically `1`).
    ///
    /// # Returns
    ///
    /// A tuple containing `(max_rpm, target_rpm)`.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
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
    ///
    /// # Arguments
    ///
    /// * `channel` - The sensor channel (typically `1`).
    ///
    /// # Returns
    ///
    /// The actual measured RPM of the fan.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
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
    ///
    /// # Arguments
    ///
    /// * `voltage_min` - The minimum voltage.
    /// * `voltage_max` - The maximum voltage.
    /// * `temperature_min` - The minimum temperature.
    /// * `temperature_max` - The maximum temperature.
    /// * `channel` - The sensor channel (typically `1`).
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
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
    /// # Arguments
    ///
    /// * `channel` - The sensor channel (typically `1`).
    ///
    /// # Returns
    ///
    /// A tuple containing `(voltage_min, voltage_max, temperature_min, temperature_max)`.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
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
    ///
    /// # Arguments
    ///
    /// * `state` - `true` to enable the laser, `false` to disable.
    /// * `frequency` - The laser frequency in Hz.
    /// * `duration` - The duration to output.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
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
    ///
    /// # Returns
    ///
    /// `true` if the laser is active, `false` otherwise.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
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
    ///
    /// * `log_slope` - The slope of the logarithmic output.
    /// * `log_offset` - The offset of the logarithmic output.
    /// * `channel` - The sensor channel (typically `1`).
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
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
    /// # Arguments
    ///
    /// * `channel` - The sensor channel (typically `1`).
    ///
    /// # Returns
    ///
    /// A tuple containing `(log_slope, log_offset)`.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
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
    ///
    /// # Arguments
    ///
    /// * `min` - The minimum power limit.
    /// * `max` - The maximum power limit.
    /// * `channel` - The sensor channel (typically `1`).
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
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
    ///
    /// # Arguments
    ///
    /// * `channel` - The sensor channel (typically `1`).
    ///
    /// # Returns
    ///
    /// A tuple containing `(min_power, max_power)`.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn get_pass_fail_power_window(&self, channel: u16) -> Result<(f64, f64), TlpmError> {
        tracing::debug!("getting pass/fail power window on channel {}", channel);
        let mut min: f64 = 0.0;
        let mut max: f64 = 0.0;
        self.check_status(
            unsafe { sys::TLPMX_getPassFailPowerWindow(self.session, &mut min, &mut max, channel) },
            "get_pass_fail_power_window",
        )?;
        Ok((min, max))
    }

    /// Set the Pass/Fail energy window limits.
    ///
    /// # Arguments
    ///
    /// * `min` - The minimum energy limit.
    /// * `max` - The maximum energy limit.
    /// * `channel` - The sensor channel (typically `1`).
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
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
    ///
    /// # Arguments
    ///
    /// * `channel` - The sensor channel (typically `1`).
    ///
    /// # Returns
    ///
    /// A tuple containing `(min_energy, max_energy)`.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn get_pass_fail_energy_window(&self, channel: u16) -> Result<(f64, f64), TlpmError> {
        tracing::debug!("getting pass/fail energy window on channel {}", channel);
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

    /// Read the overall Pass/Fail state.
    ///
    /// # Arguments
    ///
    /// * `channel` - The sensor channel (typically `1`).
    ///
    /// # Returns
    ///
    /// `true` if the state is Pass, `false` if Fail.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn get_pass_fail_state(&self, channel: u16) -> Result<bool, TlpmError> {
        tracing::debug!("getting pass/fail state on channel {}", channel);
        let mut state: sys::ViUInt16 = 0;
        self.check_status(
            unsafe { sys::TLPMX_getPassFailState(self.session, &mut state, channel) },
            "get_pass_fail_state",
        )?;
        Ok(state != 0)
    }

    impl_property!(
        numeric,
        channel,
        set_analog_output_config,
        get_analog_output_config,
        TLPMX_setAnalogOutputConfig,
        TLPMX_getAnalogOutputConfig,
        i16,
        "analog output config"
    );

    impl_property!(
        numeric,
        channel,
        set_analog_output_gain_range,
        get_analog_output_gain_range,
        TLPMX_setAnalogOutputGainRange,
        TLPMX_getAnalogOutputGainRange,
        i16,
        "analog output gain range"
    );

    /// Set the direction (Input/Output) for all 4 Digital IO pins simultaneously.
    ///
    /// # Arguments
    ///
    /// * `io0` - `true` for Output, `false` for Input.
    /// * `io1` - `true` for Output, `false` for Input.
    /// * `io2` - `true` for Output, `false` for Input.
    /// * `io3` - `true` for Output, `false` for Input.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn set_dig_io_direction(
        &self,
        io0: bool,
        io1: bool,
        io2: bool,
        io3: bool,
    ) -> Result<(), TlpmError> {
        tracing::debug!(
            "setting digital io direction: 0={}, 1={}, 2={}, 3={}",
            io0,
            io1,
            io2,
            io3
        );
        let c0 = if io0 { VI_TRUE } else { VI_FALSE };
        let c1 = if io1 { VI_TRUE } else { VI_FALSE };
        let c2 = if io2 { VI_TRUE } else { VI_FALSE };
        let c3 = if io3 { VI_TRUE } else { VI_FALSE };
        self.check_status(
            unsafe { sys::TLPMX_setDigIoDirection(self.session, c0, c1, c2, c3) },
            "set_dig_io_direction",
        )
    }

    /// Get the direction (Input/Output) for all 4 Digital IO pins.
    ///
    /// # Returns
    ///
    /// A tuple: `(io0, io1, io2, io3)` where `true` = Output, `false` = Input.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn get_dig_io_direction(&self) -> Result<(bool, bool, bool, bool), TlpmError> {
        tracing::debug!("getting digital io direction");
        let mut io0: sys::ViBoolean = 0;
        let mut io1: sys::ViBoolean = 0;
        let mut io2: sys::ViBoolean = 0;
        let mut io3: sys::ViBoolean = 0;
        self.check_status(
            unsafe {
                sys::TLPMX_getDigIoDirection(self.session, &mut io0, &mut io1, &mut io2, &mut io3)
            },
            "get_dig_io_direction",
        )?;
        Ok((
            io0 == VI_TRUE,
            io1 == VI_TRUE,
            io2 == VI_TRUE,
            io3 == VI_TRUE,
        ))
    }

    /// Set the output logic state for all 4 Digital IO pins simultaneously.
    ///
    /// # Arguments
    ///
    /// * `io0` - `true` for High, `false` for Low.
    /// * `io1` - `true` for High, `false` for Low.
    /// * `io2` - `true` for High, `false` for Low.
    /// * `io3` - `true` for High, `false` for Low.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn set_dig_io_output(
        &self,
        io0: bool,
        io1: bool,
        io2: bool,
        io3: bool,
    ) -> Result<(), TlpmError> {
        tracing::debug!(
            "setting digital io output: 0={}, 1={}, 2={}, 3={}",
            io0,
            io1,
            io2,
            io3
        );
        let c0 = if io0 { VI_TRUE } else { VI_FALSE };
        let c1 = if io1 { VI_TRUE } else { VI_FALSE };
        let c2 = if io2 { VI_TRUE } else { VI_FALSE };
        let c3 = if io3 { VI_TRUE } else { VI_FALSE };
        self.check_status(
            unsafe { sys::TLPMX_setDigIoOutput(self.session, c0, c1, c2, c3) },
            "set_dig_io_output",
        )
    }

    /// Get the actual physical logic state of all 4 Digital IO port pins.
    ///
    /// # Returns
    ///
    /// A tuple: `(io0, io1, io2, io3)` where `true` = High, `false` = Low.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn get_dig_io_port(&self) -> Result<(bool, bool, bool, bool), TlpmError> {
        tracing::debug!("getting digital io port states");
        let mut io0: sys::ViBoolean = 0;
        let mut io1: sys::ViBoolean = 0;
        let mut io2: sys::ViBoolean = 0;
        let mut io3: sys::ViBoolean = 0;
        self.check_status(
            unsafe {
                sys::TLPMX_getDigIoPort(self.session, &mut io0, &mut io1, &mut io2, &mut io3)
            },
            "get_dig_io_port",
        )?;
        Ok((
            io0 == VI_TRUE,
            io1 == VI_TRUE,
            io2 == VI_TRUE,
            io3 == VI_TRUE,
        ))
    }

    /// Set the output logic state for all 4 Digital IO pins (Alternative function).
    ///
    /// # Arguments
    ///
    /// * `io0` - `true` for High, `false` for Low.
    /// * `io1` - `true` for High, `false` for Low.
    /// * `io2` - `true` for High, `false` for Low.
    /// * `io3` - `true` for High, `false` for Low.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn set_dig_io_pin_output(
        &self,
        io0: bool,
        io1: bool,
        io2: bool,
        io3: bool,
    ) -> Result<(), TlpmError> {
        tracing::debug!(
            "setting digital io pin output: 0={}, 1={}, 2={}, 3={}",
            io0,
            io1,
            io2,
            io3
        );
        let c0 = if io0 { VI_TRUE } else { VI_FALSE };
        let c1 = if io1 { VI_TRUE } else { VI_FALSE };
        let c2 = if io2 { VI_TRUE } else { VI_FALSE };
        let c3 = if io3 { VI_TRUE } else { VI_FALSE };
        self.check_status(
            unsafe { sys::TLPMX_setDigIoPinOutput(self.session, c0, c1, c2, c3) },
            "set_dig_io_pin_output",
        )
    }

    /// Get the configured output logic state of all 4 Digital IO pins.
    ///
    /// # Returns
    ///
    /// A tuple: `(io0, io1, io2, io3)` where `true` = High, `false` = Low.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn get_dig_io_pin_output(&self) -> Result<(bool, bool, bool, bool), TlpmError> {
        tracing::debug!("getting digital io pin outputs");
        let mut io0: sys::ViBoolean = 0;
        let mut io1: sys::ViBoolean = 0;
        let mut io2: sys::ViBoolean = 0;
        let mut io3: sys::ViBoolean = 0;
        self.check_status(
            unsafe {
                sys::TLPMX_getDigIoPinOutput(self.session, &mut io0, &mut io1, &mut io2, &mut io3)
            },
            "get_dig_io_pin_output",
        )?;
        Ok((
            io0 == VI_TRUE,
            io1 == VI_TRUE,
            io2 == VI_TRUE,
            io3 == VI_TRUE,
        ))
    }

    /// Get the logic state of all 4 Digital IO input pins.
    ///
    /// # Returns
    ///
    /// A tuple: `(io0, io1, io2, io3)` where `true` = High, `false` = Low.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn get_dig_io_pin_input(&self) -> Result<(bool, bool, bool, bool), TlpmError> {
        tracing::debug!("getting digital io pin inputs");
        let mut io0: sys::ViBoolean = 0;
        let mut io1: sys::ViBoolean = 0;
        let mut io2: sys::ViBoolean = 0;
        let mut io3: sys::ViBoolean = 0;
        self.check_status(
            unsafe {
                sys::TLPMX_getDigIoPinInput(self.session, &mut io0, &mut io1, &mut io2, &mut io3)
            },
            "get_dig_io_pin_input",
        )?;
        Ok((
            io0 == VI_TRUE,
            io1 == VI_TRUE,
            io2 == VI_TRUE,
            io3 == VI_TRUE,
        ))
    }

    /// Read the analog output slope range.
    ///
    /// # Arguments
    ///
    /// * `channel` - The sensor channel (typically `1`).
    ///
    /// # Returns
    ///
    /// A tuple containing `(min_slope, max_slope)`.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn get_analog_output_slope_range(&self, channel: u16) -> Result<(f64, f64), TlpmError> {
        tracing::debug!("getting analog output slope range on channel {}", channel);
        let mut min_slope = 0.0;
        let mut max_slope = 0.0;
        self.check_status(
            unsafe {
                sys::TLPMX_getAnalogOutputSlopeRange(
                    self.session,
                    &mut min_slope,
                    &mut max_slope,
                    channel,
                )
            },
            "get_analog_output_slope_range",
        )?;
        Ok((min_slope, max_slope))
    }

    /// Read the analog output voltage.
    ///
    /// # Arguments
    ///
    /// * `attribute` - The `TlpmAttribute` to query (e.g., Set, Min, Max).
    /// * `channel` - The sensor channel (typically `1`).
    ///
    /// # Returns
    ///
    /// The analog output voltage.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn get_analog_output_voltage(
        &self,
        attribute: TlpmAttribute,
        channel: u16,
    ) -> Result<f64, TlpmError> {
        tracing::debug!("getting analog output voltage on channel {}", channel);
        let mut voltage = 0.0;
        self.check_status(
            unsafe {
                sys::TLPMX_getAnalogOutputVoltage(
                    self.session,
                    attribute as i16,
                    &mut voltage,
                    channel,
                )
            },
            "get_analog_output_voltage",
        )?;
        Ok(voltage)
    }

    /// Read the analog output voltage range.
    ///
    /// # Arguments
    ///
    /// * `channel` - The sensor channel (typically `1`).
    ///
    /// # Returns
    ///
    /// A tuple containing `(min_voltage, max_voltage)`.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn get_analog_output_voltage_range(&self, channel: u16) -> Result<(f64, f64), TlpmError> {
        tracing::debug!("getting analog output voltage range on channel {}", channel);
        let mut min_volt = 0.0;
        let mut max_volt = 0.0;
        self.check_status(
            unsafe {
                sys::TLPMX_getAnalogOutputVoltageRange(
                    self.session,
                    &mut min_volt,
                    &mut max_volt,
                    channel,
                )
            },
            "get_analog_output_voltage_range",
        )?;
        Ok((min_volt, max_volt))
    }

    /// Read the position analog output slope range.
    ///
    /// # Arguments
    ///
    /// * `channel` - The sensor channel (typically `1`).
    ///
    /// # Returns
    ///
    /// A tuple containing `(min_slope, max_slope)`.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn get_position_analog_output_slope_range(
        &self,
        channel: u16,
    ) -> Result<(f64, f64), TlpmError> {
        tracing::debug!(
            "getting position analog output slope range on channel {}",
            channel
        );
        let mut min_slope = 0.0;
        let mut max_slope = 0.0;
        self.check_status(
            unsafe {
                sys::TLPMX_getPositionAnalogOutputSlopeRange(
                    self.session,
                    &mut min_slope,
                    &mut max_slope,
                    channel,
                )
            },
            "get_position_analog_output_slope_range",
        )?;
        Ok((min_slope, max_slope))
    }

    /// Read the position analog output voltage for X and Y axes.
    ///
    /// # Arguments
    ///
    /// * `attribute` - The `TlpmAttribute` to query.
    /// * `channel` - The sensor channel (typically `1`).
    ///
    /// # Returns
    ///
    /// A tuple containing `(voltage_x, voltage_y)`.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn get_position_analog_output_voltage(
        &self,
        attribute: TlpmAttribute,
        channel: u16,
    ) -> Result<(f64, f64), TlpmError> {
        tracing::debug!(
            "getting position analog output voltage on channel {}",
            channel
        );
        let mut voltage_x = 0.0;
        let mut voltage_y = 0.0;
        self.check_status(
            unsafe {
                sys::TLPMX_getPositionAnalogOutputVoltage(
                    self.session,
                    attribute as i16,
                    &mut voltage_x,
                    &mut voltage_y,
                    channel,
                )
            },
            "get_position_analog_output_voltage",
        )?;
        Ok((voltage_x, voltage_y))
    }

    /// Read the position analog output voltage range.
    ///
    /// # Arguments
    ///
    /// * `channel` - The sensor channel (typically `1`).
    ///
    /// # Returns
    ///
    /// A tuple containing `(min_voltage, max_voltage)`.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn get_position_analog_output_voltage_range(
        &self,
        channel: u16,
    ) -> Result<(f64, f64), TlpmError> {
        tracing::debug!(
            "getting position analog output voltage range on channel {}",
            channel
        );
        let mut min_volt = 0.0;
        let mut max_volt = 0.0;
        self.check_status(
            unsafe {
                sys::TLPMX_getPositionAnalogOutputVoltageRange(
                    self.session,
                    &mut min_volt,
                    &mut max_volt,
                    channel,
                )
            },
            "get_position_analog_output_voltage_range",
        )?;
        Ok((min_volt, max_volt))
    }

    /// Get the output logic state of all 4 Digital IO pins.
    ///
    /// # Returns
    ///
    /// A tuple: `(io0, io1, io2, io3)` where `true` = High, `false` = Low.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn get_dig_io_output(&self) -> Result<(bool, bool, bool, bool), TlpmError> {
        tracing::debug!("getting digital io outputs");
        let mut io0: sys::ViBoolean = 0;
        let mut io1: sys::ViBoolean = 0;
        let mut io2: sys::ViBoolean = 0;
        let mut io3: sys::ViBoolean = 0;
        self.check_status(
            unsafe {
                sys::TLPMX_getDigIoOutput(self.session, &mut io0, &mut io1, &mut io2, &mut io3)
            },
            "get_dig_io_output",
        )?;
        Ok((
            io0 == VI_TRUE,
            io1 == VI_TRUE,
            io2 == VI_TRUE,
            io3 == VI_TRUE,
        ))
    }
}
