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

    // --- macro invocations for configuration methods ---

    impl_bool_property!(
        set_power_auto_range,
        get_power_auto_range,
        TLPMX_setPowerAutoRange,
        TLPMX_getPowerAutorange,
        "power auto-range"
    );

    impl_numeric_property!(
        set_power_range,
        get_power_range,
        TLPMX_setPowerRange,
        TLPMX_getPowerRange,
        f64,
        "power_range"
    );

    impl_numeric_property!(
        set_avg_time,
        get_avg_time,
        TLPMX_setAvgTime,
        TLPMX_getAvgTime,
        f64,
        "averaging time"
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
