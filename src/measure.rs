use crate::error::TlpmError;
use crate::{PowerMeter, sys};

impl PowerMeter {
    /// Read the current optical power from the connected sensor.
    ///
    /// # Arguments
    ///
    /// * `channel` - The sensor channel to read from (typically `1`).
    ///
    /// # Returns
    ///
    /// The measured power in the currently configured unit, which may be W or dBm.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn meas_power(&self, channel: u16) -> Result<f64, TlpmError> {
        tracing::debug!("measuring power on channel {}", channel);
        let mut power: f64 = 0.0;

        self.check_status(
            unsafe { sys::TLPMX_measPower(self.session, &mut power, channel) },
            "meas_power",
        )?;

        Ok(power)
    }

    /// Read the current electrical current from the connected sensor.
    ///
    /// # Arguments
    ///
    /// * `channel` - The sensor channel to read from (typically `1`).
    ///
    /// # Returns
    ///
    /// The measured current in Amperes.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn meas_current(&self, channel: u16) -> Result<f64, TlpmError> {
        tracing::debug!("measuring current on channel {}", channel);
        let mut current: f64 = 0.0;
        self.check_status(
            unsafe { sys::TLPMX_measCurrent(self.session, &mut current, channel) },
            "meas_current",
        )?;
        Ok(current)
    }

    /// Read the current voltage from the connected sensor.
    ///
    /// # Arguments
    ///
    /// * `channel` - The sensor channel to read from (typically `1`).
    ///
    /// # Returns
    ///
    /// The measured voltage in Volts.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn meas_voltage(&self, channel: u16) -> Result<f64, TlpmError> {
        tracing::debug!("measuring voltage on channel {}", channel);
        let mut voltage: f64 = 0.0;
        self.check_status(
            unsafe { sys::TLPMX_measVoltage(self.session, &mut voltage, channel) },
            "meas_voltage",
        )?;
        Ok(voltage)
    }

    /// Read the pulse energy from the connected sensor.
    ///
    /// # Arguments
    ///
    /// * `channel` - The sensor channel to read from (typically `1`).
    ///
    /// # Returns
    ///
    /// The measured energy in Joules.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn meas_energy(&self, channel: u16) -> Result<f64, TlpmError> {
        tracing::debug!("measuring energy on channel {}", channel);
        let mut energy: f64 = 0.0;
        self.check_status(
            unsafe { sys::TLPMX_measEnergy(self.session, &mut energy, channel) },
            "meas_energy",
        )?;
        Ok(energy)
    }
}
