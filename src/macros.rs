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
            let c_value = if value {
                crate::VI_TRUE
            } else {
                crate::VI_FALSE
            };
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
            Ok(c_value == crate::VI_TRUE)
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
            let c_value = if value {
                crate::VI_TRUE
            } else {
                crate::VI_FALSE
            };
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
            Ok(c_value == crate::VI_TRUE)
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

// macro for stamping out single-point data acquisition methods
macro_rules! impl_measure {
    ($method_name:ident, $sys_func:ident, $doc_desc:expr, $unit:expr) => {
        #[doc = concat!("Read the current ", $doc_desc, " from the connected sensor.")]
        ///
        /// # Arguments
        /// * `channel` - The sensor channel to read from (typically `1`).
        ///
        /// # Returns
        #[doc = concat!("The measured ", $doc_desc, " in ", $unit, ".")]
        ///
        /// # Errors
        /// Returns a `TlpmError::VisaError` if the device responds with an error code.
        pub fn $method_name(&self, channel: u16) -> Result<f64, TlpmError> {
            tracing::debug!(concat!("measuring ", $doc_desc, " on channel {}"), channel);
            let mut value: f64 = 0.0;
            self.check_status(
                unsafe { sys::$sys_func(self.session, &mut value, channel) },
                stringify!($method_name),
            )?;
            Ok(value)
        }
    };
}
