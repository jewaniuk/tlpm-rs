use crate::enums::TlpmAttribute;
use crate::error::TlpmError;
use crate::{PowerMeter, sys};

impl PowerMeter {
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

    /// Cancel an actively running dark adjustment.
    ///
    /// # Arguments
    ///
    /// * `channel` - The sensor channel to cancel the adjustment on (typically `1`).
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn cancel_dark_adjust(&self, channel: u16) -> Result<(), TlpmError> {
        tracing::debug!("canceling dark adjust on channel {}", channel);
        self.check_status(
            unsafe { sys::TLPMX_cancelDarkAdjust(self.session, channel) },
            "cancel_dark_adjust",
        )
    }

    impl_property!(
        numeric,
        channel,
        set_dark_offset,
        get_dark_offset,
        TLPMX_setDarkOffset,
        TLPMX_getDarkOffset,
        f64,
        "dark offset"
    );

    /// Set the X and Y zero position offsets (e.g., for position-sensing detectors).
    ///
    /// # Arguments
    ///
    /// * `position_x` - The X-axis zero position.
    /// * `position_y` - The Y-axis zero position.
    /// * `channel` - The sensor channel (typically `1`).
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn set_zero_pos(
        &self,
        position_x: f64,
        position_y: f64,
        channel: u16,
    ) -> Result<(), TlpmError> {
        tracing::debug!(
            "setting zero position (x: {}, y: {}) on channel {}",
            position_x,
            position_y,
            channel
        );
        self.check_status(
            unsafe { sys::TLPMX_setZeroPos(self.session, position_x, position_y, channel) },
            "set_zero_pos",
        )
    }

    /// Read the current X and Y zero position offsets.
    ///
    /// # Arguments
    ///
    /// * `channel` - The sensor channel (typically `1`).
    ///
    /// # Returns
    ///
    /// A tuple containing `(position_x, position_y)`.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn get_zero_pos(&self, channel: u16) -> Result<(f64, f64), TlpmError> {
        tracing::debug!("getting zero position on channel {}", channel);
        let mut pos_x: f64 = 0.0;
        let mut pos_y: f64 = 0.0;
        self.check_status(
            unsafe { sys::TLPMX_getZeroPos(self.session, &mut pos_x, &mut pos_y, channel) },
            "get_zero_pos",
        )?;
        Ok((pos_x, pos_y))
    }

    /// Start a zero position measurement.
    ///
    /// # Arguments
    ///
    /// * `channel` - The sensor channel (typically `1`).
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn start_zero_pos(&self, channel: u16) -> Result<(), TlpmError> {
        tracing::debug!("starting zero position measurement on channel {}", channel);
        self.check_status(
            unsafe { sys::TLPMX_startZeroPos(self.session, channel) },
            "start_zero_pos",
        )
    }

    /// Cancel an actively running zero position measurement.
    ///
    /// # Arguments
    ///
    /// * `channel` - The sensor channel (typically `1`).
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn cancel_zero_pos(&self, channel: u16) -> Result<(), TlpmError> {
        tracing::debug!("canceling zero position measurement on channel {}", channel);
        self.check_status(
            unsafe { sys::TLPMX_cancelZeroPos(self.session, channel) },
            "cancel_zero_pos",
        )
    }

    /// Set the R0 and Beta coefficients for an external NTC thermistor.
    ///
    /// # Arguments
    ///
    /// * `r0_coeff` - The R0 coefficient.
    /// * `beta_coeff` - The Beta coefficient.
    /// * `channel` - The sensor channel (typically `1`).
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn set_ext_ntc_parameter(
        &self,
        r0_coeff: f64,
        beta_coeff: f64,
        channel: u16,
    ) -> Result<(), TlpmError> {
        tracing::debug!(
            "setting ext ntc parameter (r0: {}, beta: {}) on channel {}",
            r0_coeff,
            beta_coeff,
            channel
        );
        self.check_status(
            unsafe { sys::TLPMX_setExtNtcParameter(self.session, r0_coeff, beta_coeff, channel) },
            "set_ext_ntc_parameter",
        )
    }

    /// Get the R0 and Beta coefficients for an external NTC thermistor.
    ///
    /// # Arguments
    ///
    /// * `attribute` - The `TlpmAttribute` to query (e.g., Set, Min, Max).
    /// * `channel` - The sensor channel (typically `1`).
    ///
    /// # Returns
    ///
    /// A tuple containing `(r0_coefficient, beta_coefficient)`.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn get_ext_ntc_parameter(
        &self,
        attribute: TlpmAttribute,
        channel: u16,
    ) -> Result<(f64, f64), TlpmError> {
        tracing::debug!("getting ext ntc parameter on channel {}", channel);
        let mut r0 = 0.0;
        let mut beta = 0.0;
        self.check_status(
            unsafe {
                sys::TLPMX_getExtNtcParameter(
                    self.session,
                    attribute as i16,
                    &mut r0,
                    &mut beta,
                    channel,
                )
            },
            "get_ext_ntc_parameter",
        )?;
        Ok((r0, beta))
    }

    /// Re-initialize the connected sensor.
    ///
    /// # Arguments
    ///
    /// * `channel` - The sensor channel to re-initialize (typically `1`).
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn reinit_sensor(&self, channel: u16) -> Result<(), TlpmError> {
        tracing::debug!("reinitializing sensor on channel {}", channel);
        self.check_status(
            unsafe { sys::TLPMX_reinitSensor(self.session, channel) },
            "reinit_sensor",
        )
    }

    /// Set the active state of a specific User Power Calibration point index.
    ///
    /// # Arguments
    ///
    /// * `index` - The calibration point index (1 to 5).
    /// * `state` - `true` to enable, `false` to disable.
    /// * `channel` - The sensor channel (typically `1`).
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn set_power_calibration_points_state(
        &self,
        index: u16,
        state: bool,
        channel: u16,
    ) -> Result<(), TlpmError> {
        tracing::debug!(
            "setting power cal point {} state to {} on channel {}",
            index,
            state,
            channel
        );
        let c_state = if state {
            crate::VI_TRUE
        } else {
            crate::VI_FALSE
        };
        self.check_status(
            unsafe {
                sys::TLPMX_setPowerCalibrationPointsState(self.session, index, c_state, channel)
            },
            "set_power_calibration_points_state",
        )
    }

    /// Read the active state of a specific User Power Calibration point index.
    ///
    /// # Arguments
    ///
    /// * `index` - The calibration point index (1 to 5).
    /// * `channel` - The sensor channel (typically `1`).
    ///
    /// # Returns
    ///
    /// `true` if enabled, `false` if disabled.
    ///
    /// # Errors
    ///
    /// Returns a `TlpmError::VisaError` if the device responds with an error code.
    pub fn get_power_calibration_points_state(
        &self,
        index: u16,
        channel: u16,
    ) -> Result<bool, TlpmError> {
        tracing::debug!(
            "getting power cal point {} state on channel {}",
            index,
            channel
        );
        let mut state: sys::ViBoolean = 0;
        self.check_status(
            unsafe {
                sys::TLPMX_getPowerCalibrationPointsState(self.session, index, &mut state, channel)
            },
            "get_power_calibration_points_state",
        )?;
        Ok(state == crate::VI_TRUE)
    }
}
