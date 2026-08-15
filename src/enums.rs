use crate::error::TlpmError;
use crate::sys;

/// Specifies the attribute to query when getting numeric parameters.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i16)]
pub enum TlpmAttribute {
    Set = sys::TLPM_ATTR_SET_VAL as i16,
    Min = sys::TLPM_ATTR_MIN_VAL as i16,
    Max = sys::TLPM_ATTR_MAX_VAL as i16,
    Default = sys::TLPM_ATTR_DFLT_VAL as i16,
}

/// Specifies the unit of measure for optical power.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i16)]
pub enum PowerUnit {
    Watt = sys::TLPM_POWER_UNIT_WATT as i16,
    Dbm = sys::TLPM_POWER_UNIT_DBM as i16,
}

impl TryFrom<i16> for PowerUnit {
    type Error = TlpmError;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        match value {
            x if x == sys::TLPM_POWER_UNIT_WATT as i16 => Ok(PowerUnit::Watt),
            x if x == sys::TLPM_POWER_UNIT_DBM as i16 => Ok(PowerUnit::Dbm),
            _ => Err(TlpmError::InvalidEnumValue(format!(
                "unknown power unit code: {}",
                value
            ))),
        }
    }
}

/// Specifies the frequency measurement mode.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u16)]
pub enum FreqMode {
    Cw = sys::TLPM_FREQ_MODE_CW as u16,
    Peak = sys::TLPM_FREQ_MODE_PEAK as u16,
}

impl TryFrom<u16> for FreqMode {
    type Error = TlpmError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            x if x == sys::TLPM_FREQ_MODE_CW as u16 => Ok(FreqMode::Cw),
            x if x == sys::TLPM_FREQ_MODE_PEAK as u16 => Ok(FreqMode::Peak),
            _ => Err(TlpmError::InvalidEnumValue(format!(
                "unknown frequency mode code: {}",
                value
            ))),
        }
    }
}

/// Specifies the peak filter state.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i16)]
pub enum PeakFilter {
    None = sys::TLPM_PEAK_FILTER_NONE as i16,
    Over = sys::TLPM_PEAK_FILTER_OVER as i16,
}

impl TryFrom<i16> for PeakFilter {
    type Error = TlpmError;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        match value {
            x if x == sys::TLPM_PEAK_FILTER_NONE as i16 => Ok(PeakFilter::None),
            x if x == sys::TLPM_PEAK_FILTER_OVER as i16 => Ok(PeakFilter::Over),
            _ => Err(TlpmError::InvalidEnumValue(format!(
                "unknown peak filter code: {}",
                value
            ))),
        }
    }
}

/// Specifies the analog output route strategy.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u16)]
pub enum AnalogRoute {
    Pur = sys::TLPM_ANALOG_ROUTE_PUR as u16,
    Cba = sys::TLPM_ANALOG_ROUTE_CBA as u16,
    Cma = sys::TLPM_ANALOG_ROUTE_CMA as u16,
    Gen = sys::TLPM_ANALOG_ROUTE_GEN as u16,
    Func = sys::TLPM_ANALOG_ROUTE_FUNC as u16,
    Cust = sys::TLPM_ANALOG_ROUTE_CUST as u16,
    Gdbm = sys::TLPM_ANALOG_ROUTE_GDBM as u16,
}

/// Specifies the position of the sensor switch (e.g., on a Thorlabs S130C).
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u16)]
pub enum SensorSwitchPosition {
    Position1 = sys::SENSOR_SWITCH_POS_1 as u16,
    Position2 = sys::SENSOR_SWITCH_POS_2 as u16,
}

/// Specifies the I2C operation mode.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u16)]
pub enum I2cMode {
    Inter = sys::I2C_OPER_INTER as u16,
    Slow = sys::I2C_OPER_SLOW as u16,
    Fast = sys::I2C_OPER_FAST as u16,
}

impl TryFrom<u16> for I2cMode {
    type Error = TlpmError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            x if x == sys::I2C_OPER_INTER as u16 => Ok(I2cMode::Inter),
            x if x == sys::I2C_OPER_SLOW as u16 => Ok(I2cMode::Slow),
            x if x == sys::I2C_OPER_FAST as u16 => Ok(I2cMode::Fast),
            _ => Err(TlpmError::InvalidEnumValue(format!(
                "unknown i2c mode code: {}",
                value
            ))),
        }
    }
}

/// Specifies the fan control mode.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u16)]
pub enum FanMode {
    Off = sys::FAN_OPER_OFF as u16,
    Full = sys::FAN_OPER_FULL as u16,
    OpenLoop = sys::FAN_OPER_OPEN_LOOP as u16,
    ClosedLoop = sys::FAN_OPER_CLOSED_LOOP as u16,
    TemperCtrl = sys::FAN_OPER_TEMPER_CTRL as u16,
}

impl TryFrom<u16> for FanMode {
    type Error = TlpmError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            x if x == sys::FAN_OPER_OFF as u16 => Ok(FanMode::Off),
            x if x == sys::FAN_OPER_FULL as u16 => Ok(FanMode::Full),
            x if x == sys::FAN_OPER_OPEN_LOOP as u16 => Ok(FanMode::OpenLoop),
            x if x == sys::FAN_OPER_CLOSED_LOOP as u16 => Ok(FanMode::ClosedLoop),
            x if x == sys::FAN_OPER_TEMPER_CTRL as u16 => Ok(FanMode::TemperCtrl),
            _ => Err(TlpmError::InvalidEnumValue(format!(
                "unknown fan mode code: {}",
                value
            ))),
        }
    }
}

/// Specifies the fan temperature control source.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u16)]
pub enum FanTempSource {
    Head = sys::FAN_TEMPER_SRC_HEAD as u16,
    ExtNtc = sys::FAN_TEMPER_SRC_EXT_NTC as u16,
}

impl TryFrom<u16> for FanTempSource {
    type Error = TlpmError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            x if x == sys::FAN_TEMPER_SRC_HEAD as u16 => Ok(FanTempSource::Head),
            x if x == sys::FAN_TEMPER_SRC_EXT_NTC as u16 => Ok(FanTempSource::ExtNtc),
            _ => Err(TlpmError::InvalidEnumValue(format!(
                "unknown fan temperature source code: {}",
                value
            ))),
        }
    }
}

/// Specifies the digital I/O pin mode.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u16)]
pub enum DigIoPinMode {
    Input = sys::DIGITAL_IO_CONFIG_INPUT as u16,
    Output = sys::DIGITAL_IO_CONFIG_OUTPUT as u16,
    InputAlt = sys::DIGITAL_IO_CONFIG_INPUT_ALT as u16,
    OutputAlt = sys::DIGITAL_IO_CONFIG_OUTPUT_ALT as u16,
}

impl TryFrom<u16> for DigIoPinMode {
    type Error = TlpmError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            x if x == sys::DIGITAL_IO_CONFIG_INPUT as u16 => Ok(DigIoPinMode::Input),
            x if x == sys::DIGITAL_IO_CONFIG_OUTPUT as u16 => Ok(DigIoPinMode::Output),
            x if x == sys::DIGITAL_IO_CONFIG_INPUT_ALT as u16 => Ok(DigIoPinMode::InputAlt),
            x if x == sys::DIGITAL_IO_CONFIG_OUTPUT_ALT as u16 => Ok(DigIoPinMode::OutputAlt),
            _ => Err(TlpmError::InvalidEnumValue(format!(
                "unknown digital io pin mode code: {}",
                value
            ))),
        }
    }
}
