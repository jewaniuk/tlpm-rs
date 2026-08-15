use crate::error::TlpmError;
use crate::{PowerMeter, VI_FALSE, VI_TRUE, sys};
use std::ffi::{CStr, CString};
use std::marker::PhantomData;

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

    /// Reset the Thorlabs power meter to its default parameters.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn reset(&self) -> Result<(), TlpmError> {
        self.check_status(unsafe { sys::TLPMX_reset(self.session) }, "reset")
    }

    // helper method to translate a visa status into a rust result with context
    pub(crate) fn check_status(
        &self,
        status: sys::ViStatus,
        action: &str,
    ) -> Result<(), TlpmError> {
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
    pub(crate) fn get_error_message(&self, error_code: sys::ViStatus) -> String {
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
                // ensure the session is cleanly closed when the power meter goes out of scope
                sys::TLPMX_close(self.session);
            }
        }
    }
}
