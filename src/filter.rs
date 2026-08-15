use crate::enums::{FreqMode, PeakFilter, TlpmAttribute};
use crate::error::TlpmError;
use crate::{PowerMeter, sys};

impl PowerMeter {
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

    impl_property!(
        numeric,
        channel,
        set_avg_cnt,
        get_avg_cnt,
        TLPMX_setAvgCnt,
        TLPMX_getAvgCnt,
        i16,
        "average count"
    );

    impl_property!(
        numeric,
        attr_channel,
        set_accel_tau,
        get_accel_tau,
        TLPMX_setAccelTau,
        TLPMX_getAccelTau,
        f64,
        "acceleration tau"
    );

    impl_property!(
        numeric,
        channel,
        set_accel_mode,
        get_accel_mode,
        TLPMX_setAccelMode,
        TLPMX_getAccelMode,
        u16,
        "acceleration mode"
    );

    impl_property!(
        numeric,
        global,
        set_filter_position,
        get_filter_position,
        TLPMX_setFilterPosition,
        TLPMX_getFilterPosition,
        u16,
        "filter position"
    );

    impl_property!(
        bool,
        global,
        set_filter_auto_mode,
        get_filter_auto_mode,
        TLPMX_setFilterAutoMode,
        TLPMX_getFilterAutoMode,
        "filter auto mode"
    );

    impl_property!(
        numeric,
        attr_channel,
        set_peak_threshold,
        get_peak_threshold,
        TLPMX_setPeakThreshold,
        TLPMX_getPeakThreshold,
        f64,
        "peak threshold"
    );

    impl_property!(
        bool,
        channel,
        set_thermopile_pulse_integrator,
        get_thermopile_pulse_integrator,
        TLPMX_setThermopilePulseIntegrator,
        TLPMX_getThermopilePulseIntegrator,
        "thermopile pulse integrator state"
    );
}
