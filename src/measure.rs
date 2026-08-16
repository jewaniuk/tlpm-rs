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

    /// Reset the fast array measurement configuration.
    ///
    /// # Arguments
    ///
    /// * `channel` - The sensor channel (typically `1`).
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn reset_fast_array_measurement(&self, channel: u16) -> Result<(), TlpmError> {
        tracing::debug!("resetting fast array measurement on channel {}", channel);
        self.check_status(
            unsafe { sys::TLPMX_resetFastArrayMeasurement(self.session, channel) },
            "reset_fast_array_measurement",
        )
    }

    /// Configure a specific measurement mode for the fast array.
    ///
    /// # Arguments
    ///
    /// * `measurement` - The measurement type code (e.g., power, voltage).
    /// * `channel` - The sensor channel (typically `1`).
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn conf_fast_array_measurement(
        &self,
        measurement: u16,
        channel: u16,
    ) -> Result<(), TlpmError> {
        tracing::debug!(
            "configuring fast array measurement (type: {}) on channel {}",
            measurement,
            channel
        );
        self.check_status(
            unsafe { sys::TLPMX_confFastArrayMeasurement(self.session, measurement, channel) },
            "conf_fast_array_measurement",
        )
    }

    /// Configure the fast array to measure optical power.
    ///
    /// # Arguments
    ///
    /// * `channel` - The sensor channel (typically `1`).
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn conf_power_fast_array_measurement(&self, channel: u16) -> Result<(), TlpmError> {
        tracing::debug!(
            "configuring power fast array measurement on channel {}",
            channel
        );
        self.check_status(
            unsafe { sys::TLPMX_confPowerFastArrayMeasurement(self.session, channel) },
            "conf_power_fast_array_measurement",
        )
    }

    /// Configure the fast array to measure electrical current.
    ///
    /// # Arguments
    ///
    /// * `channel` - The sensor channel (typically `1`).
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn conf_current_fast_array_measurement(&self, channel: u16) -> Result<(), TlpmError> {
        tracing::debug!(
            "configuring current fast array measurement on channel {}",
            channel
        );
        self.check_status(
            unsafe { sys::TLPMX_confCurrentFastArrayMeasurement(self.session, channel) },
            "conf_current_fast_array_measurement",
        )
    }

    /// Configure the fast array to measure voltage.
    ///
    /// # Arguments
    ///
    /// * `channel` - The sensor channel (typically `1`).
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn conf_voltage_fast_array_measurement(&self, channel: u16) -> Result<(), TlpmError> {
        tracing::debug!(
            "configuring voltage fast array measurement on channel {}",
            channel
        );
        self.check_status(
            unsafe { sys::TLPMX_confVoltageFastArrayMeasurement(self.session, channel) },
            "conf_voltage_fast_array_measurement",
        )
    }

    /// Configure the fast array to measure power density.
    ///
    /// # Arguments
    ///
    /// * `channel` - The sensor channel (typically `1`).
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn conf_p_density_fast_array_measurement(&self, channel: u16) -> Result<(), TlpmError> {
        tracing::debug!(
            "configuring power density fast array measurement on channel {}",
            channel
        );
        self.check_status(
            unsafe { sys::TLPMX_confPDensityFastArrayMeasurement(self.session, channel) },
            "conf_p_density_fast_array_measurement",
        )
    }

    /// Configure the fast array to measure energy.
    ///
    /// # Arguments
    ///
    /// * `channel` - The sensor channel (typically `1`).
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn conf_energy_fast_array_measurement(&self, channel: u16) -> Result<(), TlpmError> {
        tracing::debug!(
            "configuring energy fast array measurement on channel {}",
            channel
        );
        self.check_status(
            unsafe { sys::TLPMX_confEnergyFastArrayMeasurement(self.session, channel) },
            "conf_energy_fast_array_measurement",
        )
    }

    /// Configure the fast array to measure energy density.
    ///
    /// # Arguments
    ///
    /// * `channel` - The sensor channel (typically `1`).
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn conf_e_density_fast_array_measurement(&self, channel: u16) -> Result<(), TlpmError> {
        tracing::debug!(
            "configuring energy density fast array measurement on channel {}",
            channel
        );
        self.check_status(
            unsafe { sys::TLPMX_confEDensityFastArrayMeasurement(self.session, channel) },
            "conf_e_density_fast_array_measurement",
        )
    }

    /// Disable an active array measurement on the specified channel.
    ///
    /// # Arguments
    ///
    /// * `channel` - The sensor channel (typically `1`).
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn disable_array_measurement_channel(&self, channel: u16) -> Result<(), TlpmError> {
        tracing::debug!("disabling array measurement on channel {}", channel);
        self.check_status(
            unsafe { sys::TLPMX_disableArrayMeasurementChannel(self.session, channel) },
            "disable_array_measurement_channel",
        )
    }

    /// Fetch the next chunk of fast array measurement data.
    ///
    /// # Arguments
    ///
    /// * `max_samples` - The maximum number of samples to fetch in this call.
    /// * `channel` - The sensor channel (typically `1`).
    ///
    /// # Returns
    ///
    /// A tuple containing `(timestamps, values)` where timestamps are in milliseconds.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn get_next_fast_array_measurement(
        &self,
        max_samples: u32,
        channel: u16,
    ) -> Result<(Vec<u32>, Vec<f32>), TlpmError> {
        tracing::debug!(
            "getting next fast array measurement (max samples: {}) on channel {}",
            max_samples,
            channel
        );
        let mut timestamps = Vec::with_capacity(max_samples as usize);
        let mut values = Vec::with_capacity(max_samples as usize);
        let mut count_out: u32 = max_samples;

        self.check_status(
            unsafe {
                sys::TLPMX_getNextFastArrayMeasurement(
                    self.session,
                    &mut count_out,
                    timestamps.as_mut_ptr(),
                    values.as_mut_ptr(),
                    channel,
                )
            },
            "get_next_fast_array_measurement",
        )?;

        unsafe {
            timestamps.set_len(count_out as usize);
            values.set_len(count_out as usize);
        }

        Ok((timestamps, values))
    }

    /// Fetch the next chunk of fast array measurement data with relative timestamps.
    ///
    /// # Arguments
    ///
    /// * `max_samples` - The maximum number of samples to fetch in this call.
    /// * `channel` - The sensor channel (typically `1`).
    ///
    /// # Returns
    ///
    /// A tuple containing `(relative_timestamps, values)`.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn get_next_fast_array_measurement_relative_time(
        &self,
        max_samples: u32,
        channel: u16,
    ) -> Result<(Vec<u32>, Vec<f32>), TlpmError> {
        tracing::debug!(
            "getting next fast array measurement with relative time (max samples: {}) on channel {}",
            max_samples,
            channel
        );
        let mut timestamps = Vec::with_capacity(max_samples as usize);
        let mut values = Vec::with_capacity(max_samples as usize);
        let mut count_out: u32 = max_samples;

        self.check_status(
            unsafe {
                sys::TLPMX_getNextFastArrayMeasurementRelativeTime(
                    self.session,
                    &mut count_out,
                    timestamps.as_mut_ptr(),
                    values.as_mut_ptr(),
                    channel,
                )
            },
            "get_next_fast_array_measurement_relative_time",
        )?;

        unsafe {
            timestamps.set_len(count_out as usize);
            values.set_len(count_out as usize);
        }

        Ok((timestamps, values))
    }

    /// Configure the burst array to measure optical power.
    ///
    /// # Arguments
    ///
    /// * `channel` - The sensor channel (typically `1`).
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn conf_burst_array_meas_power_channel(&self, channel: u16) -> Result<(), TlpmError> {
        tracing::debug!("configuring power burst array on channel {}", channel);
        self.check_status(
            unsafe { sys::TLPMX_confBurstArrayMeasPowerChannel(self.session, channel) },
            "conf_burst_array_meas_power_channel",
        )
    }

    /// Configure the burst array to measure current.
    ///
    /// # Arguments
    ///
    /// * `channel` - The sensor channel (typically `1`).
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn conf_burst_array_meas_current_channel(&self, channel: u16) -> Result<(), TlpmError> {
        tracing::debug!("configuring current burst array on channel {}", channel);
        self.check_status(
            unsafe { sys::TLPMX_confBurstArrayMeasCurrentChannel(self.session, channel) },
            "conf_burst_array_meas_current_channel",
        )
    }

    /// Configure the burst array to measure voltage.
    ///
    /// # Arguments
    ///
    /// * `channel` - The sensor channel (typically `1`).
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn conf_burst_array_meas_voltage_channel(&self, channel: u16) -> Result<(), TlpmError> {
        tracing::debug!("configuring voltage burst array on channel {}", channel);
        self.check_status(
            unsafe { sys::TLPMX_confBurstArrayMeasVoltageChannel(self.session, channel) },
            "conf_burst_array_meas_voltage_channel",
        )
    }

    /// Configure the hardware trigger settings for burst array measurements.
    ///
    /// # Arguments
    ///
    /// * `trg_source` - The trigger source index.
    /// * `init_delay` - The delay before triggering in microseconds.
    /// * `burst_count` - The number of samples per burst.
    /// * `averaging` - The averaging mode/count.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn conf_burst_array_meas_trigger(
        &self,
        trg_source: u32,
        init_delay: u32,
        burst_count: u32,
        averaging: u32,
    ) -> Result<(), TlpmError> {
        tracing::debug!(
            "configuring burst array trigger (source: {}, delay: {}, count: {}, avg: {})",
            trg_source,
            init_delay,
            burst_count,
            averaging
        );
        self.check_status(
            unsafe {
                sys::TLPMX_confBurstArrayMeasTrigger(
                    self.session,
                    trg_source,
                    init_delay,
                    burst_count,
                    averaging,
                )
            },
            "conf_burst_array_meas_trigger",
        )
    }

    /// Start a configured burst array measurement.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn start_burst_array_measurement(&self) -> Result<(), TlpmError> {
        tracing::debug!("starting burst array measurement");
        self.check_status(
            unsafe { sys::TLPMX_startBurstArrayMeasurement(self.session) },
            "start_burst_array_measurement",
        )
    }

    /// Query the number of samples currently available in the burst array buffer.
    ///
    /// # Returns
    ///
    /// The number of available samples ready to be fetched.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn get_burst_array_samples_count(&self) -> Result<u32, TlpmError> {
        tracing::debug!("getting burst array samples count");
        let mut count: u32 = 0;
        self.check_status(
            unsafe { sys::TLPMX_getBurstArraySamplesCount(self.session, &mut count) },
            "get_burst_array_samples_count",
        )?;
        Ok(count)
    }

    /// Query the maximum allocated size of the burst array buffer.
    ///
    /// # Returns
    ///
    /// The maximum capacity of the instrument's burst buffer.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn get_burst_array_size(&self) -> Result<u32, TlpmError> {
        tracing::debug!("getting burst array buffer size");
        let mut size: u32 = 0;
        self.check_status(
            unsafe { sys::TLPMX_getBurstArraySize(self.session, &mut size) },
            "get_burst_array_size",
        )?;
        Ok(size)
    }

    /// Fetch data from the burst array buffer.
    ///
    /// Note: `values2` will be populated with secondary channel data on dual-channel
    /// meters, or remain largely unutilized on single-channel configurations.
    ///
    /// # Arguments
    ///
    /// * `start_index` - The memory offset to begin reading from.
    /// * `sample_count` - The exact number of samples to read.
    ///
    /// # Returns
    ///
    /// A tuple containing `(timestamps, values1, values2)`.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn get_burst_array_samples(
        &self,
        start_index: u32,
        sample_count: u32,
    ) -> Result<(Vec<u32>, Vec<f32>, Vec<f32>), TlpmError> {
        tracing::debug!(
            "getting burst array samples (start: {}, count: {})",
            start_index,
            sample_count
        );
        let mut timestamps = Vec::with_capacity(sample_count as usize);
        let mut values1 = Vec::with_capacity(sample_count as usize);
        let mut values2 = Vec::with_capacity(sample_count as usize);

        self.check_status(
            unsafe {
                sys::TLPMX_getBurstArraySamples(
                    self.session,
                    start_index,
                    sample_count,
                    timestamps.as_mut_ptr(),
                    values1.as_mut_ptr(),
                    values2.as_mut_ptr(),
                )
            },
            "get_burst_array_samples",
        )?;

        // Safe because the C-API guarantees that `sample_count` elements are written to the provided contiguous memory.
        unsafe {
            timestamps.set_len(sample_count as usize);
            values1.set_len(sample_count as usize);
            values2.set_len(sample_count as usize);
        }

        Ok((timestamps, values1, values2))
    }
}
