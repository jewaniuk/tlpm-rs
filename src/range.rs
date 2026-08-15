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
}
