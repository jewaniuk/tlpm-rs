pub mod error;

use crate::error::TlpmError;
use std::ffi::{CStr, CString};

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

/// safe wrapper around a thorlabs power meter session
pub struct PowerMeter {
    session: sys::ViSession,
}

impl PowerMeter {
    /// initialize a new session with the thorlabs power meter
    /// `resource_name` is the VISA resource string (e.g., "USB0::0x1313::P000000::INSTR")
    /// `id_query` is whether to query the id of the device
    /// `reset_device` is whether to reset the device settings
    pub fn init(
        resource_name: &str,
        id_query: bool,
        reset_device: bool,
    ) -> Result<Self, TlpmError> {
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

        // thorlabs/visa functions return < 0 for errors, 0 for success, and > 0 for warnings
        if status < 0 {
            return Err(TlpmError::VisaError {
                code: status,
                message: "initialization failed".to_string(),
            });
        }

        Ok(Self { session })
    }

    /// reset the device to its default parameters
    pub fn reset(&self) -> Result<(), TlpmError> {
        self.check_status(unsafe { sys::TLPMX_reset(self.session) })
    }

    /// helper method to translate a `ViStatus` into a rust `Result`
    fn check_status(&self, status: sys::ViStatus) -> Result<(), TlpmError> {
        if status < 0 {
            Err(TlpmError::VisaError {
                code: status,
                message: self.get_error_message(status),
            })
        } else {
            // ignore warnings
            Ok(())
        }
    }

    /// queries the thorlabs driver for the human-readable error description
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
}

impl Drop for PowerMeter {
    fn drop(&mut self) {
        if self.session != 0 {
            unsafe {
                // ensure the session is cleanly closed when the `PowerMeter` goes out of scope
                sys::TLPMX_close(self.session);
            }
        }
    }
}
