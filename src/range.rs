use crate::enums::{PowerUnit, TlpmAttribute};
use crate::error::TlpmError;
use crate::{PowerMeter, sys};

impl PowerMeter {
    /// Set the unit of measure for optical power.
    ///
    /// # Arguments
    ///
    /// * `unit` - The `PowerUnit` to configure (e.g., `Watt` or `Dbm`).
    /// * `channel` - The sensor channel (typically `1`).
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn set_power_unit(&self, unit: PowerUnit, channel: u16) -> Result<(), TlpmError> {
        tracing::debug!("setting power unit to {:?} on channel {}", unit, channel);
        self.check_status(
            unsafe { sys::TLPMX_setPowerUnit(self.session, unit as i16, channel) },
            "set_power_unit",
        )
    }

    /// Read the currently configured unit of measure for optical power.
    ///
    /// # Arguments
    ///
    /// * `channel` - The sensor channel (typically `1`).
    ///
    /// # Returns
    ///
    /// The currently configured `PowerUnit`.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code,
    /// or `TlpmError::InvalidEnumValue` if an unrecognized unit code is returned.
    pub fn get_power_unit(&self, channel: u16) -> Result<PowerUnit, TlpmError> {
        tracing::debug!("getting power unit on channel {}", channel);
        let mut unit_code: i16 = 0;

        self.check_status(
            unsafe { sys::TLPMX_getPowerUnit(self.session, &mut unit_code, channel) },
            "get_power_unit",
        )?;

        PowerUnit::try_from(unit_code)
    }

    impl_property!(
        bool,
        channel,
        set_power_auto_range,
        get_power_auto_range,
        TLPMX_setPowerAutoRange,
        TLPMX_getPowerAutorange,
        "power auto-range mode"
    );

    impl_property!(
        numeric,
        attr_channel,
        set_power_range,
        get_power_range,
        TLPMX_setPowerRange,
        TLPMX_getPowerRange,
        f64,
        "power range"
    );

    impl_property!(
        bool,
        channel,
        set_power_ref_state,
        get_power_ref_state,
        TLPMX_setPowerRefState,
        TLPMX_getPowerRefState,
        "power reference state"
    );

    impl_property!(
        numeric,
        attr_channel,
        set_power_ref,
        get_power_ref,
        TLPMX_setPowerRef,
        TLPMX_getPowerRef,
        f64,
        "power reference value"
    );

    impl_property!(
        bool,
        channel,
        set_current_auto_range,
        get_current_auto_range,
        TLPMX_setCurrentAutoRange,
        TLPMX_getCurrentAutorange,
        "current auto-range mode"
    );

    impl_property!(
        numeric,
        attr_channel,
        set_current_range,
        get_current_range,
        TLPMX_setCurrentRange,
        TLPMX_getCurrentRange,
        f64,
        "current range"
    );

    impl_property!(
        bool,
        channel,
        set_current_ref_state,
        get_current_ref_state,
        TLPMX_setCurrentRefState,
        TLPMX_getCurrentRefState,
        "current reference state"
    );

    impl_property!(
        numeric,
        attr_channel,
        set_current_ref,
        get_current_ref,
        TLPMX_setCurrentRef,
        TLPMX_getCurrentRef,
        f64,
        "current reference value"
    );

    impl_property!(
        bool,
        channel,
        set_voltage_auto_range,
        get_voltage_auto_range,
        TLPMX_setVoltageAutoRange,
        TLPMX_getVoltageAutorange,
        "voltage auto-range mode"
    );

    impl_property!(
        numeric,
        attr_channel,
        set_voltage_range,
        get_voltage_range,
        TLPMX_setVoltageRange,
        TLPMX_getVoltageRange,
        f64,
        "voltage range"
    );

    impl_property!(
        bool,
        channel,
        set_voltage_ref_state,
        get_voltage_ref_state,
        TLPMX_setVoltageRefState,
        TLPMX_getVoltageRefState,
        "voltage reference state"
    );

    impl_property!(
        numeric,
        attr_channel,
        set_voltage_ref,
        get_voltage_ref,
        TLPMX_setVoltageRef,
        TLPMX_getVoltageRef,
        f64,
        "voltage reference value"
    );

    impl_property!(
        bool,
        channel,
        set_energy_auto_range,
        get_energy_auto_range,
        TLPMX_setEnergyAutoRange,
        TLPMX_getEnergyAutorange,
        "energy auto-range mode"
    );

    impl_property!(
        numeric,
        attr_channel,
        set_energy_range,
        get_energy_range,
        TLPMX_setEnergyRange,
        TLPMX_getEnergyRange,
        f64,
        "energy range"
    );

    impl_property!(
        bool,
        channel,
        set_energy_ref_state,
        get_energy_ref_state,
        TLPMX_setEnergyRefState,
        TLPMX_getEnergyRefState,
        "energy reference state"
    );

    impl_property!(
        numeric,
        attr_channel,
        set_energy_ref,
        get_energy_ref,
        TLPMX_setEnergyRef,
        TLPMX_getEnergyRef,
        f64,
        "energy reference value"
    );

    /// Trigger a power range search to automatically find the optimal power measurement range.
    ///
    /// # Arguments
    /// * `channel` - The sensor channel (typically `1`).
    ///
    /// # Errors
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn set_power_range_search(&self, channel: u16) -> Result<(), TlpmError> {
        tracing::debug!("setting power range search on channel {}", channel);
        self.check_status(
            unsafe { sys::TLPMX_setPowerRangeSearch(self.session, channel) },
            "set_power_range_search",
        )
    }

    /// Trigger a current range search to automatically find the optimal current measurement range.
    ///
    /// # Arguments
    /// * `channel` - The sensor channel (typically `1`).
    ///
    /// # Errors
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn set_current_range_search(&self, channel: u16) -> Result<(), TlpmError> {
        tracing::debug!("setting current range search on channel {}", channel);
        self.check_status(
            unsafe { sys::TLPMX_setCurrentRangeSearch(self.session, channel) },
            "set_current_range_search",
        )
    }

    /// Trigger a voltage range search to automatically find the optimal voltage measurement range.
    ///
    /// # Arguments
    /// * `channel` - The sensor channel (typically `1`).
    ///
    /// # Errors
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn set_voltage_range_search(&self, channel: u16) -> Result<(), TlpmError> {
        tracing::debug!("setting voltage range search on channel {}", channel);
        self.check_status(
            unsafe { sys::TLPMX_setVoltageRangeSearch(self.session, channel) },
            "set_voltage_range_search",
        )
    }

    /// Read the lower and upper frequency bounds for the active sensor.
    ///
    /// # Arguments
    ///
    /// * `channel` - The sensor channel (typically `1`).
    ///
    /// # Returns
    ///
    /// A tuple containing `(lower_frequency, upper_frequency)` in Hertz.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn get_freq_range(&self, channel: u16) -> Result<(f64, f64), TlpmError> {
        tracing::debug!("getting frequency range on channel {}", channel);
        let mut lower: f64 = 0.0;
        let mut upper: f64 = 0.0;
        self.check_status(
            unsafe { sys::TLPMX_getFreqRange(self.session, &mut lower, &mut upper, channel) },
            "get_freq_range",
        )?;
        Ok((lower, upper))
    }

    /// Read all available current measurement ranges for the connected sensor.
    ///
    /// # Arguments
    ///
    /// * `channel` - The sensor channel (typically `1`).
    ///
    /// # Returns
    ///
    /// A vector of `f64` values representing the available current ranges in Amperes.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn get_current_ranges(&self, channel: u16) -> Result<Vec<f64>, TlpmError> {
        tracing::debug!("getting current ranges on channel {}", channel);
        // Allocate a generous buffer; instrument ranges rarely exceed 20 discrete steps
        let mut values = [0.0f64; 64];
        let mut count: u16 = 0;
        self.check_status(
            unsafe {
                sys::TLPMX_getCurrentRanges(self.session, values.as_mut_ptr(), &mut count, channel)
            },
            "get_current_ranges",
        )?;
        Ok(values[..count as usize].to_vec())
    }

    /// Read all available voltage measurement ranges for the connected sensor.
    ///
    /// # Arguments
    ///
    /// * `channel` - The sensor channel (typically `1`).
    ///
    /// # Returns
    ///
    /// A vector of `f64` values representing the available voltage ranges in Volts.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn get_voltage_ranges(&self, channel: u16) -> Result<Vec<f64>, TlpmError> {
        tracing::debug!("getting voltage ranges on channel {}", channel);
        let mut values = [0.0f64; 64];
        let mut count: u16 = 0;
        self.check_status(
            unsafe {
                sys::TLPMX_getVoltageRanges(self.session, values.as_mut_ptr(), &mut count, channel)
            },
            "get_voltage_ranges",
        )?;
        Ok(values[..count as usize].to_vec())
    }
}
