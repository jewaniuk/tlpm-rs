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

/// Safe wrapper around a Thorlabs power meter session.
///
/// This struct manages the lifetime of the VISA session used to control a Thorlabs
/// power meter. It abstracts the unsafe C-API, providing a safe interface to external Rust users.
pub struct PowerMeter {
    session: sys::ViSession,
    // phantom data tells the rust compiler that this type is not thread safe (!Send and !Sync)
    _marker: PhantomData<*const ()>,
}

// macro for stamping out boolean state getters and setters
macro_rules! impl_bool_property {
    ($setter_name:ident, $getter_name:ident, $sys_setter:ident, $sys_getter:ident, $doc_name:expr) => {
        #[doc = concat!("Set the ", $doc_name, ".")]
        ///
        /// # Arguments
        ///
        /// * `value` - The new state to apply.
        /// * `channel` - The sensor channel (typically `1`).
        ///
        /// # Errors
        ///
        /// Retrurns a `Tlpm::VisaError` if the device responds with an error code.
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
        ///
        /// * `channel` - The sensor channel (typically `1`).
        ///
        /// # Returns
        ///
        /// The current boolean state.
        ///
        /// # Errors
        ///
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
}

// macro for stamping out numeric parameter getters and setters
macro_rules! impl_numeric_property {
    ($setter_name:ident, $getter_name:ident, $sys_setter:ident, $sys_getter:ident, $ty:ty, $doc_name:expr) => {
        #[doc = concat!("Set the ", $doc_name, ".")]
        ///
        /// # Arguments
        ///
        /// * `value` - The new numeric value to apply.
        /// * `channel` - The sensor channel (typically `1`).
        ///
        /// # Errors
        ///
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
        ///
        /// * `attribute` - The attribute to query (e.g., Set, Min, Max, Default).
        /// * `channel` - The sensor channel (typically `1`).
        ///
        /// # Returns
        ///
        /// The queried numeric value.
        ///
        /// # Errors
        ///
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

    impl_bool_property!(
        set_input_filter_state,
        get_input_filter_state,
        TLPMX_setInputFilterState,
        TLPMX_getInputFilterState,
        "input filter state"
    );

    impl_bool_property!(
        set_accel_state,
        get_accel_state,
        TLPMX_setAccelState,
        TLPMX_getAccelState,
        "acceleration state"
    );

    impl_numeric_property!(
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

    impl_numeric_property!(
        set_wavelength,
        get_wavelength,
        TLPMX_setWavelength,
        TLPMX_getWavelength,
        f64,
        "wavelength"
    );

    impl_numeric_property!(
        set_attenuation,
        get_attenuation,
        TLPMX_setAttenuation,
        TLPMX_getAttenuation,
        f64,
        "attenuation"
    );

    impl_numeric_property!(
        set_beam_dia,
        get_beam_dia,
        TLPMX_setBeamDia,
        TLPMX_getBeamDia,
        f64,
        "beam diameter"
    );

    impl_numeric_property!(
        set_photodiode_responsivity,
        get_photodiode_responsivity,
        TLPMX_setPhotodiodeResponsivity,
        TLPMX_getPhotodiodeResponsivity,
        f64,
        "photodiode responsivity"
    );

    impl_numeric_property!(
        set_thermopile_responsivity,
        get_thermopile_responsivity,
        TLPMX_setThermopileResponsivity,
        TLPMX_getThermopileResponsivity,
        f64,
        "thermopile responsivity"
    );

    impl_numeric_property!(
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

    impl_bool_property!(
        set_power_auto_range,
        get_power_auto_range,
        TLPMX_setPowerAutoRange,
        TLPMX_getPowerAutorange,
        "power auto-range mode"
    );

    impl_numeric_property!(
        set_power_range,
        get_power_range,
        TLPMX_setPowerRange,
        TLPMX_getPowerRange,
        f64,
        "power range"
    );

    impl_bool_property!(
        set_power_ref_state,
        get_power_ref_state,
        TLPMX_setPowerRefState,
        TLPMX_getPowerRefState,
        "power reference state"
    );

    impl_numeric_property!(
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

    impl_bool_property!(
        set_current_auto_range,
        get_current_auto_range,
        TLPMX_setCurrentAutoRange,
        TLPMX_getCurrentAutorange,
        "current auto-range mode"
    );

    impl_numeric_property!(
        set_current_range,
        get_current_range,
        TLPMX_setCurrentRange,
        TLPMX_getCurrentRange,
        f64,
        "current range"
    );

    impl_bool_property!(
        set_current_ref_state,
        get_current_ref_state,
        TLPMX_setCurrentRefState,
        TLPMX_getCurrentRefState,
        "current reference state"
    );

    impl_numeric_property!(
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

    impl_bool_property!(
        set_voltage_auto_range,
        get_voltage_auto_range,
        TLPMX_setVoltageAutoRange,
        TLPMX_getVoltageAutorange,
        "voltage auto-range mode"
    );

    impl_numeric_property!(
        set_voltage_range,
        get_voltage_range,
        TLPMX_setVoltageRange,
        TLPMX_getVoltageRange,
        f64,
        "voltage range"
    );

    impl_bool_property!(
        set_voltage_ref_state,
        get_voltage_ref_state,
        TLPMX_setVoltageRefState,
        TLPMX_getVoltageRefState,
        "voltage reference state"
    );

    impl_numeric_property!(
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

    impl_bool_property!(
        set_energy_auto_range,
        get_energy_auto_range,
        TLPMX_setEnergyAutoRange,
        TLPMX_getEnergyAutorange,
        "energy auto-range mode"
    );

    impl_numeric_property!(
        set_energy_range,
        get_energy_range,
        TLPMX_setEnergyRange,
        TLPMX_getEnergyRange,
        f64,
        "energy range"
    );

    impl_bool_property!(
        set_energy_ref_state,
        get_energy_ref_state,
        TLPMX_setEnergyRefState,
        TLPMX_getEnergyRefState,
        "energy reference state"
    );

    impl_numeric_property!(
        set_energy_ref,
        get_energy_ref,
        TLPMX_setEnergyRef,
        TLPMX_getEnergyRef,
        f64,
        "energy reference value"
    );
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
