use crate::error::TlpmError;
use crate::sys;
use std::ffi::CStr;

/// Information about a discovered Thorlabs PM-series instrument.
#[derive(Debug, Clone)]
pub struct DiscoveredResource {
    pub resource_string: String,
    pub model_name: String,
    pub serial_number: String,
    pub manufacturer: String,
    pub device_available: bool,
}

/// Scan the host system to find all connected Thorlabs PM-series instruments.
///
/// This function queries the internal VISA Resource Manager. It does not require
/// an active instrument session to run.
///
/// # Returns
///
/// A vector of `DiscoveredResource` structs detailing all found devices.
///
/// # Errors
///
/// Returns a `TlpmError::VisaError` if the resource manager fails to execute the search.
pub fn find_resources() -> Result<Vec<DiscoveredResource>, TlpmError> {
    tracing::debug!("initiating resource discovery scan");
    let mut resource_count: u32 = 0;

    // Passing 0 (VI_NULL) tells the driver to use the default Resource Manager
    let status = unsafe { sys::TLPMX_findRsrc(0, &mut resource_count) };

    if status < 0 {
        return Err(TlpmError::VisaError {
            code: status,
            action: "find_resources".to_string(),
            message: "failed to execute hardware discovery scan".to_string(),
        });
    }

    let mut resources = Vec::with_capacity(resource_count as usize);

    for i in 0..resource_count {
        let mut rsrc_name = [0i8; sys::TLPM_BUFFER_SIZE as usize];
        let mut model_name = [0i8; sys::TLPM_BUFFER_SIZE as usize];
        let mut serial_number = [0i8; sys::TLPM_BUFFER_SIZE as usize];
        let mut manufacturer = [0i8; sys::TLPM_BUFFER_SIZE as usize];
        let mut available: sys::ViBoolean = 0;

        // Retrieve the VISA resource string used for connection
        unsafe {
            sys::TLPMX_getRsrcName(0, i, rsrc_name.as_mut_ptr());
        }

        // Retrieve the human-readable metadata for the instrument
        unsafe {
            sys::TLPMX_getRsrcInfo(
                0,
                i,
                model_name.as_mut_ptr(),
                serial_number.as_mut_ptr(),
                manufacturer.as_mut_ptr(),
                &mut available,
            );
        }

        let to_string = |buf: &[i8]| -> String {
            unsafe { CStr::from_ptr(buf.as_ptr()) }
                .to_string_lossy()
                .into_owned()
        };

        resources.push(DiscoveredResource {
            resource_string: to_string(&rsrc_name),
            model_name: to_string(&model_name),
            serial_number: to_string(&serial_number),
            manufacturer: to_string(&manufacturer),
            device_available: available == crate::VI_TRUE,
        });
    }

    tracing::debug!("discovery scan complete: found {} devices", resources.len());
    Ok(resources)
}
