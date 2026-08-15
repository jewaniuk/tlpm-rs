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
    /// * `position_x` - The X-axis zero position.
    /// * `position_y` - The Y-axis zero position.
    /// * `channel` - The sensor channel (typically `1`).
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
    /// # Returns
    /// A tuple containing `(position_x, position_y)`.
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
    pub fn start_zero_pos(&self, channel: u16) -> Result<(), TlpmError> {
        tracing::debug!("starting zero position measurement on channel {}", channel);
        self.check_status(
            unsafe { sys::TLPMX_startZeroPos(self.session, channel) },
            "start_zero_pos",
        )
    }

    /// Cancel an actively running zero position measurement.
    pub fn cancel_zero_pos(&self, channel: u16) -> Result<(), TlpmError> {
        tracing::debug!("canceling zero position measurement on channel {}", channel);
        self.check_status(
            unsafe { sys::TLPMX_cancelZeroPos(self.session, channel) },
            "cancel_zero_pos",
        )
    }
}
