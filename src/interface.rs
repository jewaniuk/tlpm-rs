use crate::enums::I2cMode;
use crate::error::TlpmError;
use crate::{PowerMeter, sys};
use std::ffi::{CStr, CString};

impl PowerMeter {
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

    /// Read data via the instrument's I2C interface.
    ///
    /// # Arguments
    ///
    /// * `address` - The I2C address to read from.
    /// * `count` - The number of bytes to read.
    ///
    /// # Returns
    ///
    /// The packed data read from the I2C bus.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn i2c_read(&self, address: u32, count: u32) -> Result<u32, TlpmError> {
        tracing::debug!("reading {} bytes from i2c address {:#X}", count, address);
        let mut data_read: u32 = 0;
        self.check_status(
            unsafe { sys::TLPMX_I2CRead(self.session, address, count, &mut data_read) },
            "i2c_read",
        )?;
        Ok(data_read)
    }

    /// Write hexadecimal data to the instrument's I2C interface.
    ///
    /// # Arguments
    ///
    /// * `address` - The I2C address to write to.
    /// * `hex_data` - A hexadecimal string representation of the data to write.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::StringConversion` for invalid strings, or `TlpmError::VisaError`.
    pub fn i2c_write(&self, address: u32, hex_data: &str) -> Result<(), TlpmError> {
        tracing::debug!("writing to i2c address {:#X}: {}", address, hex_data);
        let c_str = std::ffi::CString::new(hex_data)
            .map_err(|_| TlpmError::StringConversion("invalid hex string".to_string()))?;
        self.check_status(
            unsafe { sys::TLPMX_I2CWrite(self.session, address, c_str.as_ptr() as *mut _) },
            "i2c_write",
        )
    }

    /// Perform a combined write-then-read operation on the I2C interface.
    ///
    /// # Arguments
    ///
    /// * `address` - The I2C address.
    /// * `hex_send_data` - The hexadecimal string data to write.
    /// * `count` - The number of bytes to read after writing.
    ///
    /// # Returns
    ///
    /// The packed data read from the I2C bus.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::StringConversion` for invalid strings, or `TlpmError::VisaError`.
    pub fn i2c_write_read(
        &self,
        address: u32,
        hex_send_data: &str,
        count: u32,
    ) -> Result<u32, TlpmError> {
        tracing::debug!(
            "i2c write/read at address {:#X} (count: {})",
            address,
            count
        );
        let c_str = std::ffi::CString::new(hex_send_data)
            .map_err(|_| TlpmError::StringConversion("invalid hex string".to_string()))?;
        let mut data_read: u32 = 0;
        self.check_status(
            unsafe {
                sys::TLPMX_I2CWriteRead(
                    self.session,
                    address,
                    c_str.as_ptr() as *mut _,
                    count,
                    &mut data_read,
                )
            },
            "i2c_write_read",
        )?;
        Ok(data_read)
    }

    /// Write a raw SCPI command string directly to the instrument.
    ///
    /// # Arguments
    ///
    /// * `command` - The SCPI command string to send.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::StringConversion` for invalid strings, or `TlpmError::VisaError`.
    pub fn write_raw(&self, command: &str) -> Result<(), TlpmError> {
        tracing::debug!("writing raw SCPI command: {}", command);
        let c_str = std::ffi::CString::new(command)
            .map_err(|_| TlpmError::StringConversion("invalid command string".to_string()))?;
        self.check_status(
            unsafe { sys::TLPMX_writeRaw(self.session, c_str.as_ptr() as *mut _) },
            "write_raw",
        )
    }

    /// Read raw response data directly from the instrument.
    ///
    /// # Arguments
    ///
    /// * `size` - The maximum number of bytes to read.
    ///
    /// # Returns
    ///
    /// A tuple containing `(response_string, bytes_read)`.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn read_raw(&self, size: u32) -> Result<(String, u32), TlpmError> {
        tracing::debug!("reading raw data (max size: {})", size);
        let mut buffer = vec![0i8; size as usize];
        let mut return_count: u32 = 0;
        self.check_status(
            unsafe {
                sys::TLPMX_readRaw(self.session, buffer.as_mut_ptr(), size, &mut return_count)
            },
            "read_raw",
        )?;
        let msg = unsafe { std::ffi::CStr::from_ptr(buffer.as_ptr()) }
            .to_string_lossy()
            .into_owned();
        Ok((msg, return_count))
    }

    /// Write directly to a specific instrument status register.
    ///
    /// # Arguments
    ///
    /// * `reg` - The register address/index.
    /// * `value` - The value to write.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn write_register(&self, reg: i16, value: i16) -> Result<(), TlpmError> {
        tracing::debug!("writing {} to register {}", value, reg);
        self.check_status(
            unsafe { sys::TLPMX_writeRegister(self.session, reg, value) },
            "write_register",
        )
    }

    /// Read a value directly from a specific instrument status register.
    ///
    /// # Arguments
    ///
    /// * `reg` - The register address/index.
    ///
    /// # Returns
    ///
    /// The integer value read from the register.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn read_register(&self, reg: i16) -> Result<i16, TlpmError> {
        tracing::debug!("reading from register {}", reg);
        let mut value: i16 = 0;
        self.check_status(
            unsafe { sys::TLPMX_readRegister(self.session, reg, &mut value) },
            "read_register",
        )?;
        Ok(value)
    }

    /// Preset the instrument status registers to their default state.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn preset_register(&self) -> Result<(), TlpmError> {
        tracing::debug!("presetting registers");
        self.check_status(
            unsafe { sys::TLPMX_presetRegister(self.session) },
            "preset_register",
        )
    }
}
