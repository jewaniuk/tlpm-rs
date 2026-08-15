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
    pub fn meas_emm_humidity(&self) -> Result<f64, TlpmError> {
        tracing::debug!("measuring EMM humidity");
        let mut value: f64 = 0.0;
        self.check_status(
            unsafe { sys::TLPMX_measEmmHumidity(self.session, &mut value) },
            "meas_emm_humidity",
        )?;
        Ok(value)
    }
}
