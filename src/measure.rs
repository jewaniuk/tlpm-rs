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
}
