pub mod error;

use crate::error::TlpmError;
use std::ffi::{CStr, CString};
use std::marker::PhantomData;

// suppress warnings for generated c bindings
#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
mod sys {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

// visa boolean constants
const VI_TRUE: sys::ViBoolean = 1;
const VI_FALSE: sys::ViBoolean = 0;

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

/// Safe wrapper around a Thorlabs power meter session.
///
/// This struct manages the lifetime of the VISA session used to control a Thorlabs
/// power meter. It abstracts the unsafe C-API, providing a safe interface to external Rust users.
pub struct PowerMeter {
    session: sys::ViSession,
    // phantom data tells the rust compiler that this type is not thread safe (!Send and !Sync)
    _marker: PhantomData<*const ()>,
}

// unified macro for generating boilerplate getter and setter properties
macro_rules! impl_property {
    // 1. boolean properties that require a channel
    (bool, channel, $setter_name:ident, $getter_name:ident, $sys_setter:ident, $sys_getter:ident, $doc_name:expr) => {
        #[doc = concat!("Set the ", $doc_name, ".")]
        ///
        /// # Arguments
        /// * `value` - The new state to apply.
        /// * `channel` - The sensor channel (typically `1`).
        ///
        /// # Errors
        /// Returns a `TlpmError::VisaError` if the device responds with an error code.
        pub fn $setter_name(&self, value: bool, channel: u16) -> Result<(), TlpmError> {
            tracing::debug!(
                concat!("setting ", $doc_name, " to {} on channel {}"),
                value,
                channel
            );
            let c_value = if value { VI_TRUE } else { VI_FALSE };
            self.check_status(
                unsafe { sys::$sys_setter(self.session, c_value, channel) },
                stringify!($setter_name),
            )
        }

        #[doc = concat!("Get the ", $doc_name, " state.")]
        ///
        /// # Arguments
        /// * `channel` - The sensor channel (typically `1`).
        ///
        /// # Returns
        /// The current boolean state.
        ///
        /// # Errors
        /// Returns a `TlpmError::VisaError` if the device responds with an error code.
        pub fn $getter_name(&self, channel: u16) -> Result<bool, TlpmError> {
            tracing::debug!(concat!("getting ", $doc_name, " on channel {}"), channel);
            let mut c_value: sys::ViBoolean = 0;
            self.check_status(
                unsafe { sys::$sys_getter(self.session, &mut c_value, channel) },
                stringify!($getter_name),
            )?;
            Ok(c_value == VI_TRUE)
        }
    };

    // 2. boolean global properties (no channel)
    (bool, global, $setter_name:ident, $getter_name:ident, $sys_setter:ident, $sys_getter:ident, $doc_name:expr) => {
        #[doc = concat!("Set the ", $doc_name, ".")]
        ///
        /// # Arguments
        /// * `value` - The new state to apply.
        ///
        /// # Errors
        /// Returns a `TlpmError::VisaError` if the device responds with an error code.
        pub fn $setter_name(&self, value: bool) -> Result<(), TlpmError> {
            tracing::debug!(concat!("setting ", $doc_name, " to {}"), value);
            let c_value = if value { VI_TRUE } else { VI_FALSE };
            self.check_status(
                unsafe { sys::$sys_setter(self.session, c_value) },
                stringify!($setter_name),
            )
        }

        #[doc = concat!("Get the ", $doc_name, " state.")]
        ///
        /// # Returns
        /// The current boolean state.
        ///
        /// # Errors
        /// Returns a `TlpmError::VisaError` if the device responds with an error code.
        pub fn $getter_name(&self) -> Result<bool, TlpmError> {
            tracing::debug!(concat!("getting ", $doc_name));
            let mut c_value: sys::ViBoolean = 0;
            self.check_status(
                unsafe { sys::$sys_getter(self.session, &mut c_value) },
                stringify!($getter_name),
            )?;
            Ok(c_value == VI_TRUE)
        }
    };

    // 3. numeric properties that require an attribute and a channel
    (numeric, attr_channel, $setter_name:ident, $getter_name:ident, $sys_setter:ident, $sys_getter:ident, $ty:ty, $doc_name:expr) => {
        #[doc = concat!("Set the ", $doc_name, ".")]
        ///
        /// # Arguments
        /// * `value` - The new numeric value to apply.
        /// * `channel` - The sensor channel (typically `1`).
        ///
        /// # Errors
        /// Returns a `TlpmError::VisaError` if the device responds with an error code.
        pub fn $setter_name(&self, value: $ty, channel: u16) -> Result<(), TlpmError> {
            tracing::debug!(concat!("setting ", $doc_name, " on channel {}"), channel);
            self.check_status(
                unsafe { sys::$sys_setter(self.session, value, channel) },
                stringify!($setter_name),
            )
        }

        #[doc = concat!("Get the ", $doc_name, ".")]
        ///
        /// # Arguments
        /// * `attribute` - The attribute to query (e.g., Set, Min, Max, Default).
        /// * `channel` - The sensor channel (typically `1`).
        ///
        /// # Returns
        /// The queried numeric value.
        ///
        /// # Errors
        /// Returns a `TlpmError::VisaError` if the device responds with an error code.
        pub fn $getter_name(
            &self,
            attribute: TlpmAttribute,
            channel: u16,
        ) -> Result<$ty, TlpmError> {
            tracing::debug!(concat!("getting ", $doc_name, " on channel {}"), channel);
            let mut value: $ty = Default::default();
            self.check_status(
                unsafe { sys::$sys_getter(self.session, attribute as i16, &mut value, channel) },
                stringify!($getter_name),
            )?;
            Ok(value)
        }
    };

    // 4. simple numeric global properties (no attribute, no channel)
    (numeric, global, $setter_name:ident, $getter_name:ident, $sys_setter:ident, $sys_getter:ident, $ty:ty, $doc_name:expr) => {
        #[doc = concat!("Set the ", $doc_name, ".")]
        ///
        /// # Arguments
        /// * `value` - The new numeric value to apply.
        ///
        /// # Errors
        /// Returns a `TlpmError::VisaError` if the device responds with an error code.
        pub fn $setter_name(&self, value: $ty) -> Result<(), TlpmError> {
            tracing::debug!(concat!("setting ", $doc_name));
            self.check_status(
                unsafe { sys::$sys_setter(self.session, value) },
                stringify!($setter_name),
            )
        }

        #[doc = concat!("Get the ", $doc_name, ".")]
        ///
        /// # Returns
        /// The queried numeric value.
        ///
        /// # Errors
        /// Returns a `TlpmError::VisaError` if the device responds with an error code.
        pub fn $getter_name(&self) -> Result<$ty, TlpmError> {
            tracing::debug!(concat!("getting ", $doc_name));
            let mut value: $ty = Default::default();
            self.check_status(
                unsafe { sys::$sys_getter(self.session, &mut value) },
                stringify!($getter_name),
            )?;
            Ok(value)
        }
    };

    // 5. string global properties (network and hostname configurations)
    (string, global, $setter_name:ident, $getter_name:ident, $sys_setter:ident, $sys_getter:ident, $doc_name:expr) => {
        #[doc = concat!("Set the ", $doc_name, ".")]
        ///
        /// # Arguments
        /// * `value` - The string value to apply.
        ///
        /// # Errors
        /// Returns a `TlpmError::StringConversion` if the string contains a null byte,
        /// or a `TlpmError::VisaError` if the device responds with an error code.
        pub fn $setter_name(&self, value: &str) -> Result<(), TlpmError> {
            tracing::debug!(concat!("setting ", $doc_name));
            let c_value = CString::new(value).map_err(|_| {
                TlpmError::StringConversion("string contains null byte".to_string())
            })?;
            self.check_status(
                unsafe { sys::$sys_setter(self.session, c_value.as_ptr() as *mut _) },
                stringify!($setter_name),
            )
        }

        #[doc = concat!("Get the ", $doc_name, ".")]
        ///
        /// # Returns
        /// The queried string value.
        ///
        /// # Errors
        /// Returns a `TlpmError::VisaError` if the device responds with an error code.
        pub fn $getter_name(&self) -> Result<String, TlpmError> {
            tracing::debug!(concat!("getting ", $doc_name));
            let mut buffer = [0i8; sys::TLPM_BUFFER_SIZE as usize];
            self.check_status(
                unsafe { sys::$sys_getter(self.session, buffer.as_mut_ptr()) },
                stringify!($getter_name),
            )?;
            let c_str = unsafe { CStr::from_ptr(buffer.as_ptr()) };
            Ok(c_str.to_string_lossy().into_owned())
        }
    };
}

impl PowerMeter {
    /// Initialize a new session with the Thorlabs power meter.
    ///
    /// # Examples
    ///
    /// ```
    /// let power_meter = PowerMeter::init("USB0::0x1313::0x8078::P000000::INSTR", true, true);
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the initialization fails.
    pub fn new(resource_name: &str, id_query: bool, reset_device: bool) -> Result<Self, TlpmError> {
        tracing::debug!("initializing power meter at resource: {}", resource_name);

        let c_resource_name = CString::new(resource_name)
            .map_err(|_| TlpmError::InvalidResourceName(resource_name.to_string()))?;

        let mut session: sys::ViSession = 0;
        let c_id_query = if id_query { VI_TRUE } else { VI_FALSE };
        let c_reset_device = if reset_device { VI_TRUE } else { VI_FALSE };

        let status = unsafe {
            sys::TLPMX_init(
                c_resource_name.as_ptr() as *mut _,
                c_id_query,
                c_reset_device,
                &mut session,
            )
        };

        // thorlabs visa functions return less than 0 for errors, 0 for success, and greater than 0 for warnings
        if status < 0 {
            return Err(TlpmError::VisaError {
                code: status,
                action: "new".to_string(),
                message: "initialization failed".to_string(),
            });
        }

        tracing::debug!("succesfully initialized session: {}", session);

        Ok(Self {
            session,
            _marker: PhantomData,
        })
    }

    /// Retrieve the instrument's identification information.
    ///
    /// # Returns
    ///
    /// A tuple containing the manufacturer, device name, serial number, and firmware revision.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn identification_query(&self) -> Result<(String, String, String, String), TlpmError> {
        let mut manufacturer = [0i8; sys::TLPM_BUFFER_SIZE as usize];
        let mut device_name = [0i8; sys::TLPM_BUFFER_SIZE as usize];
        let mut serial_number = [0i8; sys::TLPM_BUFFER_SIZE as usize];
        let mut firmware = [0i8; sys::TLPM_BUFFER_SIZE as usize];

        self.check_status(
            unsafe {
                sys::TLPMX_identificationQuery(
                    self.session,
                    manufacturer.as_mut_ptr(),
                    device_name.as_mut_ptr(),
                    serial_number.as_mut_ptr(),
                    firmware.as_mut_ptr(),
                )
            },
            "identification_query",
        )?;

        // helper closure to safely convert the populated c style arrays into owned rust strings
        let to_string = |buf: &[i8]| -> String {
            unsafe { CStr::from_ptr(buf.as_ptr()) }
                .to_string_lossy()
                .into_owned()
        };

        Ok((
            to_string(&manufacturer),
            to_string(&device_name),
            to_string(&serial_number),
            to_string(&firmware),
        ))
    }

    /// Reset the Thorlabs power meter to its default parameters.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn reset(&self) -> Result<(), TlpmError> {
        self.check_status(unsafe { sys::TLPMX_reset(self.session) }, "reset")
    }

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

    /// Initiate the dark current/zero adjustment procedure.
    ///
    /// This process measures the dark current of the sensor and stores it for
    /// subsequent measurements. The laser beam should be blocked before calling this.
    ///
    /// # Arguments
    ///
    /// * `channel` - The sensor channel to adjust (typically `1`).
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn start_dark_adjust(&self, channel: u16) -> Result<(), TlpmError> {
        tracing::debug!("starting dark adjust on channel {}", channel);
        self.check_status(
            unsafe { sys::TLPMX_startDarkAdjust(self.session, channel) },
            "start_dark_adjust",
        )
    }

    /// Check the state of the dark adjustment procedure.
    ///
    /// # Arguments
    ///
    /// * `channel` - The sensor channel to check (typically `1`).
    ///
    /// # Returns
    ///
    /// `true` if the adjustment is currently running, `false` if it is finished.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn get_dark_adjust_state(&self, channel: u16) -> Result<bool, TlpmError> {
        tracing::debug!("getting dark adjust state on channel {}", channel);
        let mut state: i16 = 0;
        self.check_status(
            unsafe { sys::TLPMX_getDarkAdjustState(self.session, &mut state, channel) },
            "get_dark_adjust_state",
        )?;
        // sys::TLPM_STAT_DARK_ADJUST_RUNNING is 1, TLPM_STAT_DARK_ADJUST_FINISHED is 0
        Ok(state == sys::TLPM_STAT_DARK_ADJUST_RUNNING as i16)
    }

    // helper method to translate a visa status into a rust result with context
    fn check_status(&self, status: sys::ViStatus, action: &str) -> Result<(), TlpmError> {
        if status < 0 {
            let message = self.get_error_message(status);
            tracing::debug!(
                "visa error during {}: {} (code: {})",
                action,
                message,
                status
            );
            Err(TlpmError::VisaError {
                code: status,
                action: action.to_string(),
                message: self.get_error_message(status),
            })
        } else {
            // ignore warnings
            Ok(())
        }
    }

    // queries the thorlabs driver for the human readable error description
    fn get_error_message(&self, error_code: sys::ViStatus) -> String {
        let mut buffer: [sys::ViChar; sys::TLPM_ERR_DESCR_BUFFER_SIZE as usize] =
            [0; sys::TLPM_ERR_DESCR_BUFFER_SIZE as usize];

        let status =
            unsafe { sys::TLPMX_errorMessage(self.session, error_code, buffer.as_mut_ptr()) };

        if status < 0 {
            return "failed to retrieve error message from driver".to_string();
        }

        let c_str = unsafe { CStr::from_ptr(buffer.as_ptr()) };
        c_str.to_string_lossy().into_owned()
    }

    // =======================================================================
    // input and averaging configuration
    // =======================================================================

    impl_property!(
        bool,
        channel,
        set_input_filter_state,
        get_input_filter_state,
        TLPMX_setInputFilterState,
        TLPMX_getInputFilterState,
        "input filter state"
    );

    impl_property!(
        bool,
        channel,
        set_accel_state,
        get_accel_state,
        TLPMX_setAccelState,
        TLPMX_getAccelState,
        "acceleration state"
    );

    impl_property!(
        numeric,
        attr_channel,
        set_avg_time,
        get_avg_time,
        TLPMX_setAvgTime,
        TLPMX_getAvgTime,
        f64,
        "averaging time"
    );

    // =======================================================================
    // corrections and responsivity configuration
    // =======================================================================

    impl_property!(
        numeric,
        attr_channel,
        set_wavelength,
        get_wavelength,
        TLPMX_setWavelength,
        TLPMX_getWavelength,
        f64,
        "wavelength"
    );

    impl_property!(
        numeric,
        attr_channel,
        set_attenuation,
        get_attenuation,
        TLPMX_setAttenuation,
        TLPMX_getAttenuation,
        f64,
        "attenuation"
    );

    impl_property!(
        numeric,
        attr_channel,
        set_beam_dia,
        get_beam_dia,
        TLPMX_setBeamDia,
        TLPMX_getBeamDia,
        f64,
        "beam diameter"
    );

    impl_property!(
        numeric,
        attr_channel,
        set_photodiode_responsivity,
        get_photodiode_responsivity,
        TLPMX_setPhotodiodeResponsivity,
        TLPMX_getPhotodiodeResponsivity,
        f64,
        "photodiode responsivity"
    );

    impl_property!(
        numeric,
        attr_channel,
        set_thermopile_responsivity,
        get_thermopile_responsivity,
        TLPMX_setThermopileResponsivity,
        TLPMX_getThermopileResponsivity,
        f64,
        "thermopile responsivity"
    );

    impl_property!(
        numeric,
        attr_channel,
        set_pyrosensor_responsivity,
        get_pyrosensor_responsivity,
        TLPMX_setPyrosensorResponsivity,
        TLPMX_getPyrosensorResponsivity,
        f64,
        "pyrosensor responsivity"
    );

    // =======================================================================
    // power measurement configuration
    // =======================================================================

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

    // =======================================================================
    // current measurement configuration
    // =======================================================================

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

    // =======================================================================
    // voltage measurement configuration
    // =======================================================================

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

    // =======================================================================
    // energy measurement configuration
    // =======================================================================

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

    // =======================================================================
    // frequency measurement configuration
    // =======================================================================

    /// Set the frequency measurement mode.
    ///
    /// # Arguments
    ///
    /// * `mode` - The `FreqMode` to configure (e.g., `Cw` or `Peak`).
    /// * `channel` - The sensor channel (typically `1`).
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn set_freq_mode(&self, mode: FreqMode, channel: u16) -> Result<(), TlpmError> {
        tracing::debug!("setting freq mode to {:?} on channel {}", mode, channel);
        self.check_status(
            unsafe { sys::TLPMX_setFreqMode(self.session, mode as u16, channel) },
            "set_freq_mode",
        )
    }

    /// Read the currently configured frequency measurement mode.
    ///
    /// # Arguments
    ///
    /// * `channel` - The sensor channel (typically `1`).
    ///
    /// # Returns
    ///
    /// The currently configured `FreqMode`.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code,
    /// or `TlpmError::InvalidEnumValue` if an unrecognized mode code is returned.
    pub fn get_freq_mode(&self, channel: u16) -> Result<FreqMode, TlpmError> {
        tracing::debug!("getting freq mode on channel {}", channel);
        let mut mode_code: u16 = 0;

        self.check_status(
            unsafe { sys::TLPMX_getFreqMode(self.session, &mut mode_code, channel) },
            "get_freq_mode",
        )?;

        FreqMode::try_from(mode_code)
    }

    // =======================================================================
    // peak detector configuration
    // =======================================================================

    /// Set the peak filter configuration.
    ///
    /// # Arguments
    ///
    /// * `filter` - The `PeakFilter` state to apply.
    /// * `channel` - The sensor channel (typically `1`).
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn set_peak_filter(&self, filter: PeakFilter, channel: u16) -> Result<(), TlpmError> {
        tracing::debug!("setting peak filter to {:?} on channel {}", filter, channel);
        self.check_status(
            unsafe { sys::TLPMX_setPeakFilter(self.session, filter as i16, channel) },
            "set_peak_filter",
        )
    }

    /// Read the currently configured peak filter.
    ///
    /// # Arguments
    ///
    /// * `channel` - The sensor channel (typically `1`).
    ///
    /// # Returns
    ///
    /// The currently configured `PeakFilter`.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code,
    /// or `TlpmError::InvalidEnumValue` if an unrecognized filter code is returned.
    pub fn get_peak_filter(&self, channel: u16) -> Result<PeakFilter, TlpmError> {
        tracing::debug!("getting peak filter on channel {}", channel);
        let mut filter_code: i16 = 0;

        self.check_status(
            unsafe { sys::TLPMX_getPeakFilter(self.session, &mut filter_code, channel) },
            "get_peak_filter",
        )?;

        PeakFilter::try_from(filter_code)
    }

    // =======================================================================
    // analog output configuration
    // =======================================================================

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

    // =======================================================================
    // i2c configuration
    // =======================================================================

    /// Set the I2C operation mode.
    ///
    /// # Arguments
    ///
    /// * `mode` - The `I2cMode` to configure.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn set_i2c_mode(&self, mode: I2cMode) -> Result<(), TlpmError> {
        tracing::debug!("setting i2c mode to {:?}", mode);
        self.check_status(
            unsafe { sys::TLPMX_setI2CMode(self.session, mode as u16) },
            "set_i2c_mode",
        )
    }

    /// Read the currently configured I2C operation mode.
    ///
    /// # Returns
    ///
    /// The currently configured `I2cMode`.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code,
    /// or `TlpmError::InvalidEnumValue` if an unrecognized mode code is returned.
    pub fn get_i2c_mode(&self) -> Result<I2cMode, TlpmError> {
        tracing::debug!("getting i2c mode");
        // the getter expects a signed integer pointer
        let mut mode_code: i16 = 0;

        self.check_status(
            unsafe { sys::TLPMX_getI2CMode(self.session, &mut mode_code) },
            "get_i2c_mode",
        )?;

        // cast back to unsigned for the try_from conversion
        I2cMode::try_from(mode_code as u16)
    }

    // =======================================================================
    // fan configuration
    // =======================================================================

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

    // =======================================================================
    // digital io configuration
    // =======================================================================

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

    // =======================================================================
    // serial interface and communication configuration
    // =======================================================================

    impl_property!(
        numeric,
        global,
        set_device_baudrate,
        get_device_baudrate,
        TLPMX_setDeviceBaudrate,
        TLPMX_getDeviceBaudrate,
        u32,
        "device baudrate"
    );

    impl_property!(
        numeric,
        global,
        set_driver_baudrate,
        get_driver_baudrate,
        TLPMX_setDriverBaudrate,
        TLPMX_getDriverBaudrate,
        u32,
        "driver baudrate"
    );

    impl_property!(
        numeric,
        global,
        set_timeout_value,
        get_timeout_value,
        TLPMX_setTimeoutValue,
        TLPMX_getTimeoutValue,
        u32,
        "communication timeout value in milliseconds"
    );

    // =======================================================================
    // ethernet and network configuration
    // =======================================================================

    impl_property!(
        string,
        global,
        set_ip_address,
        get_ip_address,
        TLPMX_setIPAddress,
        TLPMX_getIPAddress,
        "ip address"
    );

    impl_property!(
        string,
        global,
        set_ip_mask,
        get_ip_mask,
        TLPMX_setIPMask,
        TLPMX_getIPMask,
        "subnet mask"
    );

    impl_property!(
        string,
        global,
        set_gateway,
        get_gateway,
        TLPMX_setGateway,
        TLPMX_getGateway,
        "default gateway"
    );

    impl_property!(
        string,
        global,
        set_hostname,
        get_hostname,
        TLPMX_setHostname,
        TLPMX_getHostname,
        "hostname"
    );

    impl_property!(
        bool,
        global,
        set_dhcp,
        get_dhcp,
        TLPMX_setDHCP,
        TLPMX_getDHCP,
        "dhcp state"
    );

    impl_property!(
        numeric,
        global,
        set_web_port,
        get_web_port,
        TLPMX_setWebPort,
        TLPMX_getWebPort,
        u32,
        "web server port"
    );

    impl_property!(
        numeric,
        global,
        set_scpi_port,
        get_scpi_port,
        TLPMX_setSCPIPort,
        TLPMX_getSCPIPort,
        u32,
        "scpi port"
    );

    impl_property!(
        numeric,
        global,
        set_dfu_port,
        get_dfu_port,
        TLPMX_setDFUPort,
        TLPMX_getDFUPort,
        u32,
        "dfu port"
    );

    impl_property!(
        bool,
        global,
        set_lan_propagation,
        get_lan_propagation,
        TLPMX_setLANPropagation,
        TLPMX_getLANPropagation,
        "lan propagation state"
    );

    impl_property!(
        bool,
        global,
        set_enable_net_search,
        get_enable_net_search,
        TLPMX_setEnableNetSearch,
        TLPMX_getEnableNetSearch,
        "enable network search state"
    );

    impl_property!(
        bool,
        global,
        set_look_for_info_on_search,
        get_look_for_info_on_search,
        TLPMX_setLookForInfoOnSearch,
        TLPMX_getLookForInfoOnSearch,
        "look for info on search state"
    );

    impl_property!(
        bool,
        global,
        set_enable_bth_search,
        get_enable_bth_search,
        TLPMX_setEnableBthSearch,
        TLPMX_getEnableBthSearch,
        "enable bluetooth search state"
    );

    /// Retrieve the instrument's MAC address.
    ///
    /// # Returns
    ///
    /// The MAC address string.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn get_mac_address(&self) -> Result<String, TlpmError> {
        tracing::debug!("getting mac address");
        let mut buffer = [0i8; sys::TLPM_BUFFER_SIZE as usize];
        self.check_status(
            unsafe { sys::TLPMX_getMACAddress(self.session, buffer.as_mut_ptr()) },
            "get_mac_address",
        )?;
        let c_str = unsafe { CStr::from_ptr(buffer.as_ptr()) };
        Ok(c_str.to_string_lossy().into_owned())
    }

    /// Set the network search mask.
    ///
    /// # Arguments
    ///
    /// * `net_mask` - The search mask string to apply.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::StringConversion` if the string contains a null byte,
    /// or a `TlpmError::VisaError` if the device responds with an error code.
    pub fn set_net_search_mask(&self, net_mask: &str) -> Result<(), TlpmError> {
        tracing::debug!("setting network search mask");
        let c_value = CString::new(net_mask)
            .map_err(|_| TlpmError::StringConversion("string contains null byte".to_string()))?;
        self.check_status(
            unsafe { sys::TLPMX_setNetSearchMask(self.session, c_value.as_ptr() as *mut _) },
            "set_net_search_mask",
        )
    }

    // =======================================================================
    // display and system configuration
    // =======================================================================

    impl_property!(
        numeric,
        global,
        set_disp_brightness,
        get_disp_brightness,
        TLPMX_setDispBrightness,
        TLPMX_getDispBrightness,
        f64,
        "display brightness"
    );

    impl_property!(
        numeric,
        global,
        set_disp_contrast,
        get_disp_contrast,
        TLPMX_setDispContrast,
        TLPMX_getDispContrast,
        f64,
        "display contrast"
    );

    impl_property!(
        numeric,
        global,
        set_line_frequency,
        get_line_frequency,
        TLPMX_setLineFrequency,
        TLPMX_getLineFrequency,
        i16,
        "line frequency"
    );

    impl_property!(
        bool,
        global,
        set_summertime,
        get_summertime,
        TLPMX_setSummertime,
        TLPMX_getSummertime,
        "summertime (daylight saving) state"
    );

    impl_property!(
        string,
        global,
        set_display_name,
        get_display_name,
        TLPMX_setDisplayName,
        TLPMX_getDisplayName,
        "display name"
    );

    /// Set the system time of the device.
    ///
    /// # Arguments
    ///
    /// * `year` - The year (e.g., 2026).
    /// * `month` - The month (1-12).
    /// * `day` - The day (1-31).
    /// * `hour` - The hour (0-23).
    /// * `minute` - The minute (0-59).
    /// * `second` - The second (0-59).
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn set_time(
        &self,
        year: i16,
        month: i16,
        day: i16,
        hour: i16,
        minute: i16,
        second: i16,
    ) -> Result<(), TlpmError> {
        tracing::debug!("setting system time");
        self.check_status(
            unsafe { sys::TLPMX_setTime(self.session, year, month, day, hour, minute, second) },
            "set_time",
        )
    }

    /// Retrieve the system time of the device.
    ///
    /// # Returns
    ///
    /// A tuple containing `(year, month, day, hour, minute, second)`.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn get_time(&self) -> Result<(i16, i16, i16, i16, i16, i16), TlpmError> {
        tracing::debug!("getting system time");
        let mut year: i16 = 0;
        let mut month: i16 = 0;
        let mut day: i16 = 0;
        let mut hour: i16 = 0;
        let mut minute: i16 = 0;
        let mut second: i16 = 0;

        self.check_status(
            unsafe {
                sys::TLPMX_getTime(
                    self.session,
                    &mut year,
                    &mut month,
                    &mut day,
                    &mut hour,
                    &mut minute,
                    &mut second,
                )
            },
            "get_time",
        )?;

        Ok((year, month, day, hour, minute, second))
    }

    /// Retrieve the instrument's battery voltage.
    ///
    /// # Returns
    ///
    /// The battery voltage in Volts.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn get_battery_voltage(&self) -> Result<f64, TlpmError> {
        tracing::debug!("getting battery voltage");
        let mut voltage: f64 = 0.0;
        self.check_status(
            unsafe { sys::TLPMX_getBatteryVoltage(self.session, &mut voltage) },
            "get_battery_voltage",
        )?;
        Ok(voltage)
    }

    /// Produce a beep sound from the device.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn beep(&self) -> Result<(), TlpmError> {
        tracing::debug!("triggering device beep");
        self.check_status(unsafe { sys::TLPMX_beep(self.session) }, "beep")
    }

    // =======================================================================
    // shutter and hardware control
    // =======================================================================

    impl_property!(
        bool,
        global,
        set_shutter_position,
        get_shutter_position,
        TLPMX_setShutterPosition,
        TLPMX_getShutterPosition,
        "shutter position"
    );

    // =======================================================================
    // fan specific controls
    // =======================================================================

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

    // =======================================================================
    // laser, encryption, and system info
    // =======================================================================

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

    /// Set the device encryption configuration.
    pub fn set_encryption(
        &self,
        old_password: &str,
        new_password: &str,
        encryption_enabled: bool,
    ) -> Result<(), TlpmError> {
        tracing::debug!("setting encryption state");
        let c_old = CString::new(old_password)
            .map_err(|_| TlpmError::StringConversion("invalid old password".to_string()))?;
        let c_new = CString::new(new_password)
            .map_err(|_| TlpmError::StringConversion("invalid new password".to_string()))?;
        let c_enc = if encryption_enabled {
            VI_TRUE
        } else {
            VI_FALSE
        };

        self.check_status(
            unsafe {
                sys::TLPMX_setEncryption(
                    self.session,
                    c_old.as_ptr() as *mut _,
                    c_new.as_ptr() as *mut _,
                    c_enc,
                )
            },
            "set_encryption",
        )
    }

    /// Retrieve the current encryption configuration.
    ///
    /// # Returns
    /// A tuple containing `(password_string, encryption_enabled)`.
    pub fn get_encryption(&self) -> Result<(String, bool), TlpmError> {
        tracing::debug!("getting encryption state");
        let mut password = [0i8; sys::TLPM_BUFFER_SIZE as usize];
        let mut is_encrypted: sys::ViBoolean = 0;

        self.check_status(
            unsafe {
                sys::TLPMX_getEncryption(self.session, password.as_mut_ptr(), &mut is_encrypted)
            },
            "get_encryption",
        )?;

        let c_str = unsafe { CStr::from_ptr(password.as_ptr()) };
        Ok((
            c_str.to_string_lossy().into_owned(),
            is_encrypted == VI_TRUE,
        ))
    }

    /// Retrieve the number of channels available on the connected instrument.
    pub fn get_channels(&self) -> Result<u16, TlpmError> {
        tracing::debug!("getting available channel count");
        let mut channel_count: u16 = 0;
        self.check_status(
            unsafe { sys::TLPMX_getChannels(self.session, &mut channel_count) },
            "get_channels",
        )?;
        Ok(channel_count)
    }

    /// Retrieve the calibration message for the sensor.
    pub fn get_calibration_msg(&self, channel: u16) -> Result<String, TlpmError> {
        tracing::debug!("getting calibration message on channel {}", channel);
        let mut message = [0i8; sys::TLPM_BUFFER_SIZE as usize];
        self.check_status(
            unsafe { sys::TLPMX_getCalibrationMsg(self.session, message.as_mut_ptr(), channel) },
            "get_calibration_msg",
        )?;
        let c_str = unsafe { CStr::from_ptr(message.as_ptr()) };
        Ok(c_str.to_string_lossy().into_owned())
    }

    /// Retrieve comprehensive sensor information using the extended 32-bit flags.
    ///
    /// # Returns
    /// A tuple containing `(name, serial_number, message, type, subtype, flags)`.
    pub fn get_sensor_info(
        &self,
        channel: u16,
    ) -> Result<(String, String, String, i16, i16, i32), TlpmError> {
        tracing::debug!("getting sensor info on channel {}", channel);
        let mut name = [0i8; sys::TLPM_BUFFER_SIZE as usize];
        let mut snr = [0i8; sys::TLPM_BUFFER_SIZE as usize];
        let mut message = [0i8; sys::TLPM_BUFFER_SIZE as usize];
        let mut p_type: i16 = 0;
        let mut p_stype: i16 = 0;
        let mut p_flags: i32 = 0;

        self.check_status(
            unsafe {
                sys::TLPMX_getSensorInfoExt(
                    self.session,
                    name.as_mut_ptr(),
                    snr.as_mut_ptr(),
                    message.as_mut_ptr(),
                    &mut p_type,
                    &mut p_stype,
                    &mut p_flags,
                    channel,
                )
            },
            "get_sensor_info",
        )?;

        let to_string = |buf: &[i8]| -> String {
            unsafe { CStr::from_ptr(buf.as_ptr()) }
                .to_string_lossy()
                .into_owned()
        };

        Ok((
            to_string(&name),
            to_string(&snr),
            to_string(&message),
            p_type,
            p_stype,
            p_flags,
        ))
    }
}

impl Drop for PowerMeter {
    fn drop(&mut self) {
        if self.session != 0 {
            unsafe {
                // ensure the session is cleanly closed when the power meter goes out of scope
                sys::TLPMX_close(self.session);
            }
        }
    }
}
