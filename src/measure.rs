use crate::error::TlpmError;
use crate::{PowerMeter, sys};

impl PowerMeter {
    impl_measure!(
        meas_power,
        TLPMX_measPower,
        "optical power",
        "the currently configured unit (W or dBm)"
    );

    impl_measure!(
        meas_current,
        TLPMX_measCurrent,
        "electrical current",
        "Amperes"
    );

    impl_measure!(meas_voltage, TLPMX_measVoltage, "voltage", "Volts");

    impl_measure!(meas_energy, TLPMX_measEnergy, "pulse energy", "Joules");

    impl_measure!(meas_freq, TLPMX_measFreq, "frequency", "Hertz");

    impl_measure!(
        meas_power_dens,
        TLPMX_measPowerDens,
        "power density",
        "W/cm²"
    );

    impl_measure!(
        meas_energy_dens,
        TLPMX_measEnergyDens,
        "energy density",
        "J/cm²"
    );

    impl_measure!(
        meas_head_temperature,
        TLPMX_measHeadTemperature,
        "head temperature",
        "degrees Celsius"
    );

    impl_measure!(
        meas_head_resistance,
        TLPMX_measHeadResistance,
        "head resistance",
        "Ohms"
    );

    impl_measure!(
        meas_ext_ntc_temperature,
        TLPMX_measExtNtcTemperature,
        "external NTC temperature",
        "degrees Celsius"
    );

    impl_measure!(
        meas_ext_ntc_resistance,
        TLPMX_measExtNtcResistance,
        "external NTC resistance",
        "Ohms"
    );

    impl_measure!(
        meas_aux_analog_input,
        TLPMX_measAuxAnalogInput,
        "auxiliary analog input",
        "Volts"
    );

    impl_measure!(
        meas_pos_duty_cycle,
        TLPMX_measPosDutyCycle,
        "positive duty cycle",
        "percent"
    );

    impl_measure!(
        meas_neg_duty_cycle,
        TLPMX_measNegDutyCycle,
        "negative duty cycle",
        "percent"
    );

    impl_measure!(
        meas_pos_pulse_width,
        TLPMX_measPosPulseWidth,
        "positive pulse width",
        "seconds"
    );

    impl_measure!(
        meas_neg_pulse_width,
        TLPMX_measNegPulseWidth,
        "negative pulse width",
        "seconds"
    );

    /// Read the EMM temperature.
    ///
    /// # Returns
    ///
    /// The measured EMM temperature in degrees Celsius.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn meas_emm_temperature(&self) -> Result<f64, TlpmError> {
        tracing::debug!("measuring EMM temperature");
        let mut value: f64 = 0.0;
        self.check_status(
            unsafe { sys::TLPMX_measEmmTemperature(self.session, &mut value) },
            "meas_emm_temperature",
        )?;
        Ok(value)
    }

    /// Read the EMM humidity.
    ///
    /// # Returns
    ///
    /// The measured EMM humidity in percent.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn meas_emm_humidity(&self) -> Result<f64, TlpmError> {
        tracing::debug!("measuring EMM humidity");
        let mut value: f64 = 0.0;
        self.check_status(
            unsafe { sys::TLPMX_measEmmHumidity(self.session, &mut value) },
            "meas_emm_humidity",
        )?;
        Ok(value)
    }

    /// Read the X and Y positions from a 4-Quadrant detector.
    ///
    /// # Arguments
    ///
    /// * `channel` - The sensor channel (typically `1`).
    ///
    /// # Returns
    ///
    /// A tuple containing `(x_pos, y_pos)`.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn meas_4q_positions(&self, channel: u16) -> Result<(f64, f64), TlpmError> {
        tracing::debug!("measuring 4Q positions on channel {}", channel);
        let mut x: f64 = 0.0;
        let mut y: f64 = 0.0;
        self.check_status(
            unsafe { sys::TLPMX_meas4QPositions(self.session, &mut x, &mut y, channel) },
            "meas_4q_positions",
        )?;
        Ok((x, y))
    }

    /// Read the four individual quadrant voltages from a 4-Quadrant detector.
    ///
    /// # Arguments
    ///
    /// * `channel` - The sensor channel (typically `1`).
    ///
    /// # Returns
    ///
    /// A tuple containing `(voltage1, voltage2, voltage3, voltage4)`.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn meas_4q_voltages(&self, channel: u16) -> Result<(f64, f64, f64, f64), TlpmError> {
        tracing::debug!("measuring 4Q voltages on channel {}", channel);
        let mut v1: f64 = 0.0;
        let mut v2: f64 = 0.0;
        let mut v3: f64 = 0.0;
        let mut v4: f64 = 0.0;

        self.check_status(
            unsafe {
                sys::TLPMX_meas4QVoltages(self.session, &mut v1, &mut v2, &mut v3, &mut v4, channel)
            },
            "meas_4q_voltages",
        )?;
        Ok((v1, v2, v3, v4))
    }

    /// Measure both channels simultaneously on a dual-channel meter.
    ///
    /// # Arguments
    ///
    /// * `measurement_unit` - The unit to measure (e.g., 0 for Power, 1 for Current, 2 for Voltage).
    ///
    /// # Returns
    ///
    /// A tuple containing `(value_channel_1, value_channel_2)`.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn meas_dual_channel_simultaneous(
        &self,
        measurement_unit: u16,
    ) -> Result<(f64, f64), TlpmError> {
        tracing::debug!(
            "measuring dual channel simultaneously (unit: {})",
            measurement_unit
        );
        let mut val1: f64 = 0.0;
        let mut val2: f64 = 0.0;

        self.check_status(
            unsafe {
                sys::TLPMX_measDualChannelSimultaneous(
                    self.session,
                    measurement_unit,
                    &mut val1,
                    &mut val2,
                )
            },
            "meas_dual_channel_simultaneous",
        )?;
        Ok((val1, val2))
    }

    /// Retrieve the state of the measurement fetch operation.
    ///
    /// # Arguments
    ///
    /// * `channel` - The sensor channel (typically `1`).
    ///
    /// # Returns
    ///
    /// `true` if measurement data is available to fetch, `false` otherwise.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn get_fetch_state(&self, channel: u16) -> Result<bool, TlpmError> {
        tracing::debug!("getting fetch state on channel {}", channel);
        let mut state: sys::ViBoolean = 0;
        self.check_status(
            unsafe { sys::TLPMX_getFetchState(self.session, &mut state, channel) },
            "get_fetch_state",
        )?;
        Ok(state == crate::VI_TRUE)
    }

    /// Retrieve the maximum fast samplerate for the active sensor.
    ///
    /// # Arguments
    ///
    /// * `channel` - The sensor channel (typically `1`).
    ///
    /// # Returns
    ///
    /// The maximum fast samplerate in Hz.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn get_fast_max_samplerate(&self, channel: u16) -> Result<u32, TlpmError> {
        tracing::debug!("getting fast max samplerate on channel {}", channel);
        let mut rate: u32 = 0;
        self.check_status(
            unsafe { sys::TLPMX_getFastMaxSamplerate(self.session, &mut rate, channel) },
            "get_fast_max_samplerate",
        )?;
        Ok(rate)
    }
}
