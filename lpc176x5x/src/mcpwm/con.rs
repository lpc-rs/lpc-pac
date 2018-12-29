#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CON {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `RUN0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RUN0R {
    #[doc = "Stop."]
    STOP_,
    #[doc = "Run."]
    RUN_,
}
impl RUN0R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            RUN0R::STOP_ => false,
            RUN0R::RUN_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RUN0R {
        match value {
            false => RUN0R::STOP_,
            true => RUN0R::RUN_,
        }
    }
    #[doc = "Checks if the value of the field is `STOP_`"]
    #[inline]
    pub fn is_stop_(&self) -> bool {
        *self == RUN0R::STOP_
    }
    #[doc = "Checks if the value of the field is `RUN_`"]
    #[inline]
    pub fn is_run_(&self) -> bool {
        *self == RUN0R::RUN_
    }
}
#[doc = "Possible values of the field `CENTER0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CENTER0R {
    #[doc = "Edge-aligned."]
    EDGE_ALIGNED_,
    #[doc = "Center-aligned."]
    CENTER_ALIGNED_,
}
impl CENTER0R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CENTER0R::EDGE_ALIGNED_ => false,
            CENTER0R::CENTER_ALIGNED_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CENTER0R {
        match value {
            false => CENTER0R::EDGE_ALIGNED_,
            true => CENTER0R::CENTER_ALIGNED_,
        }
    }
    #[doc = "Checks if the value of the field is `EDGE_ALIGNED_`"]
    #[inline]
    pub fn is_edge_aligned_(&self) -> bool {
        *self == CENTER0R::EDGE_ALIGNED_
    }
    #[doc = "Checks if the value of the field is `CENTER_ALIGNED_`"]
    #[inline]
    pub fn is_center_aligned_(&self) -> bool {
        *self == CENTER0R::CENTER_ALIGNED_
    }
}
#[doc = "Possible values of the field `POLA0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POLA0R {
    #[doc = "Passive state is LOW, active state is HIGH."]
    PASSIVE_STATE_IS_LOW,
    #[doc = "Passive state is HIGH, active state is LOW."]
    PASSIVE_STATE_IS_HIG,
}
impl POLA0R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            POLA0R::PASSIVE_STATE_IS_LOW => false,
            POLA0R::PASSIVE_STATE_IS_HIG => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> POLA0R {
        match value {
            false => POLA0R::PASSIVE_STATE_IS_LOW,
            true => POLA0R::PASSIVE_STATE_IS_HIG,
        }
    }
    #[doc = "Checks if the value of the field is `PASSIVE_STATE_IS_LOW`"]
    #[inline]
    pub fn is_passive_state_is_low(&self) -> bool {
        *self == POLA0R::PASSIVE_STATE_IS_LOW
    }
    #[doc = "Checks if the value of the field is `PASSIVE_STATE_IS_HIG`"]
    #[inline]
    pub fn is_passive_state_is_hig(&self) -> bool {
        *self == POLA0R::PASSIVE_STATE_IS_HIG
    }
}
#[doc = "Possible values of the field `DTE0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTE0R {
    #[doc = "Dead-time disabled."]
    DEAD_TIME_DISABLED_,
    #[doc = "Dead-time enabled."]
    DEAD_TIME_ENABLED_,
}
impl DTE0R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            DTE0R::DEAD_TIME_DISABLED_ => false,
            DTE0R::DEAD_TIME_ENABLED_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DTE0R {
        match value {
            false => DTE0R::DEAD_TIME_DISABLED_,
            true => DTE0R::DEAD_TIME_ENABLED_,
        }
    }
    #[doc = "Checks if the value of the field is `DEAD_TIME_DISABLED_`"]
    #[inline]
    pub fn is_dead_time_disabled_(&self) -> bool {
        *self == DTE0R::DEAD_TIME_DISABLED_
    }
    #[doc = "Checks if the value of the field is `DEAD_TIME_ENABLED_`"]
    #[inline]
    pub fn is_dead_time_enabled_(&self) -> bool {
        *self == DTE0R::DEAD_TIME_ENABLED_
    }
}
#[doc = "Possible values of the field `DISUP0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISUP0R {
    #[doc = "Functional registers are updated from the write registers at the end of each PWM cycle."]
    UPDATE,
    #[doc = "Functional registers remain the same as long as the timer is running."]
    NOUPDATE,
}
impl DISUP0R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            DISUP0R::UPDATE => false,
            DISUP0R::NOUPDATE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DISUP0R {
        match value {
            false => DISUP0R::UPDATE,
            true => DISUP0R::NOUPDATE,
        }
    }
    #[doc = "Checks if the value of the field is `UPDATE`"]
    #[inline]
    pub fn is_update(&self) -> bool {
        *self == DISUP0R::UPDATE
    }
    #[doc = "Checks if the value of the field is `NOUPDATE`"]
    #[inline]
    pub fn is_noupdate(&self) -> bool {
        *self == DISUP0R::NOUPDATE
    }
}
#[doc = "Possible values of the field `RUN1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RUN1R {
    #[doc = "Stop."]
    STOP_,
    #[doc = "Run."]
    RUN_,
}
impl RUN1R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            RUN1R::STOP_ => false,
            RUN1R::RUN_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RUN1R {
        match value {
            false => RUN1R::STOP_,
            true => RUN1R::RUN_,
        }
    }
    #[doc = "Checks if the value of the field is `STOP_`"]
    #[inline]
    pub fn is_stop_(&self) -> bool {
        *self == RUN1R::STOP_
    }
    #[doc = "Checks if the value of the field is `RUN_`"]
    #[inline]
    pub fn is_run_(&self) -> bool {
        *self == RUN1R::RUN_
    }
}
#[doc = "Possible values of the field `CENTER1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CENTER1R {
    #[doc = "Edge-aligned."]
    EDGE_ALIGNED_,
    #[doc = "Center-aligned."]
    CENTER_ALIGNED_,
}
impl CENTER1R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CENTER1R::EDGE_ALIGNED_ => false,
            CENTER1R::CENTER_ALIGNED_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CENTER1R {
        match value {
            false => CENTER1R::EDGE_ALIGNED_,
            true => CENTER1R::CENTER_ALIGNED_,
        }
    }
    #[doc = "Checks if the value of the field is `EDGE_ALIGNED_`"]
    #[inline]
    pub fn is_edge_aligned_(&self) -> bool {
        *self == CENTER1R::EDGE_ALIGNED_
    }
    #[doc = "Checks if the value of the field is `CENTER_ALIGNED_`"]
    #[inline]
    pub fn is_center_aligned_(&self) -> bool {
        *self == CENTER1R::CENTER_ALIGNED_
    }
}
#[doc = "Possible values of the field `POLA1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POLA1R {
    #[doc = "Passive state is LOW, active state is HIGH."]
    PASSIVE_STATE_IS_LOW,
    #[doc = "Passive state is HIGH, active state is LOW."]
    PASSIVE_STATE_IS_HIG,
}
impl POLA1R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            POLA1R::PASSIVE_STATE_IS_LOW => false,
            POLA1R::PASSIVE_STATE_IS_HIG => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> POLA1R {
        match value {
            false => POLA1R::PASSIVE_STATE_IS_LOW,
            true => POLA1R::PASSIVE_STATE_IS_HIG,
        }
    }
    #[doc = "Checks if the value of the field is `PASSIVE_STATE_IS_LOW`"]
    #[inline]
    pub fn is_passive_state_is_low(&self) -> bool {
        *self == POLA1R::PASSIVE_STATE_IS_LOW
    }
    #[doc = "Checks if the value of the field is `PASSIVE_STATE_IS_HIG`"]
    #[inline]
    pub fn is_passive_state_is_hig(&self) -> bool {
        *self == POLA1R::PASSIVE_STATE_IS_HIG
    }
}
#[doc = "Possible values of the field `DTE1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTE1R {
    #[doc = "Dead-time disabled."]
    DEAD_TIME_DISABLED_,
    #[doc = "Dead-time enabled."]
    DEAD_TIME_ENABLED_,
}
impl DTE1R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            DTE1R::DEAD_TIME_DISABLED_ => false,
            DTE1R::DEAD_TIME_ENABLED_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DTE1R {
        match value {
            false => DTE1R::DEAD_TIME_DISABLED_,
            true => DTE1R::DEAD_TIME_ENABLED_,
        }
    }
    #[doc = "Checks if the value of the field is `DEAD_TIME_DISABLED_`"]
    #[inline]
    pub fn is_dead_time_disabled_(&self) -> bool {
        *self == DTE1R::DEAD_TIME_DISABLED_
    }
    #[doc = "Checks if the value of the field is `DEAD_TIME_ENABLED_`"]
    #[inline]
    pub fn is_dead_time_enabled_(&self) -> bool {
        *self == DTE1R::DEAD_TIME_ENABLED_
    }
}
#[doc = "Possible values of the field `DISUP1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISUP1R {
    #[doc = "Functional registers are updated from the write registers at the end of each PWM cycle."]
    UPDATE,
    #[doc = "Functional registers remain the same as long as the timer is running."]
    NOUPDATE,
}
impl DISUP1R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            DISUP1R::UPDATE => false,
            DISUP1R::NOUPDATE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DISUP1R {
        match value {
            false => DISUP1R::UPDATE,
            true => DISUP1R::NOUPDATE,
        }
    }
    #[doc = "Checks if the value of the field is `UPDATE`"]
    #[inline]
    pub fn is_update(&self) -> bool {
        *self == DISUP1R::UPDATE
    }
    #[doc = "Checks if the value of the field is `NOUPDATE`"]
    #[inline]
    pub fn is_noupdate(&self) -> bool {
        *self == DISUP1R::NOUPDATE
    }
}
#[doc = "Possible values of the field `RUN2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RUN2R {
    #[doc = "Stop."]
    STOP_,
    #[doc = "Run."]
    RUN_,
}
impl RUN2R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            RUN2R::STOP_ => false,
            RUN2R::RUN_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RUN2R {
        match value {
            false => RUN2R::STOP_,
            true => RUN2R::RUN_,
        }
    }
    #[doc = "Checks if the value of the field is `STOP_`"]
    #[inline]
    pub fn is_stop_(&self) -> bool {
        *self == RUN2R::STOP_
    }
    #[doc = "Checks if the value of the field is `RUN_`"]
    #[inline]
    pub fn is_run_(&self) -> bool {
        *self == RUN2R::RUN_
    }
}
#[doc = "Possible values of the field `CENTER2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CENTER2R {
    #[doc = "Edge-aligned."]
    EDGE_ALIGNED_,
    #[doc = "Center-aligned."]
    CENTER_ALIGNED_,
}
impl CENTER2R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CENTER2R::EDGE_ALIGNED_ => false,
            CENTER2R::CENTER_ALIGNED_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CENTER2R {
        match value {
            false => CENTER2R::EDGE_ALIGNED_,
            true => CENTER2R::CENTER_ALIGNED_,
        }
    }
    #[doc = "Checks if the value of the field is `EDGE_ALIGNED_`"]
    #[inline]
    pub fn is_edge_aligned_(&self) -> bool {
        *self == CENTER2R::EDGE_ALIGNED_
    }
    #[doc = "Checks if the value of the field is `CENTER_ALIGNED_`"]
    #[inline]
    pub fn is_center_aligned_(&self) -> bool {
        *self == CENTER2R::CENTER_ALIGNED_
    }
}
#[doc = "Possible values of the field `POLA2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POLA2R {
    #[doc = "Passive state is LOW, active state is HIGH."]
    PASSIVE_STATE_IS_LOW,
    #[doc = "Passive state is HIGH, active state is LOW."]
    PASSIVE_STATE_IS_HIG,
}
impl POLA2R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            POLA2R::PASSIVE_STATE_IS_LOW => false,
            POLA2R::PASSIVE_STATE_IS_HIG => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> POLA2R {
        match value {
            false => POLA2R::PASSIVE_STATE_IS_LOW,
            true => POLA2R::PASSIVE_STATE_IS_HIG,
        }
    }
    #[doc = "Checks if the value of the field is `PASSIVE_STATE_IS_LOW`"]
    #[inline]
    pub fn is_passive_state_is_low(&self) -> bool {
        *self == POLA2R::PASSIVE_STATE_IS_LOW
    }
    #[doc = "Checks if the value of the field is `PASSIVE_STATE_IS_HIG`"]
    #[inline]
    pub fn is_passive_state_is_hig(&self) -> bool {
        *self == POLA2R::PASSIVE_STATE_IS_HIG
    }
}
#[doc = "Possible values of the field `DTE2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTE2R {
    #[doc = "Dead-time disabled."]
    DEAD_TIME_DISABLED_,
    #[doc = "Dead-time enabled."]
    DEAD_TIME_ENABLED_,
}
impl DTE2R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            DTE2R::DEAD_TIME_DISABLED_ => false,
            DTE2R::DEAD_TIME_ENABLED_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DTE2R {
        match value {
            false => DTE2R::DEAD_TIME_DISABLED_,
            true => DTE2R::DEAD_TIME_ENABLED_,
        }
    }
    #[doc = "Checks if the value of the field is `DEAD_TIME_DISABLED_`"]
    #[inline]
    pub fn is_dead_time_disabled_(&self) -> bool {
        *self == DTE2R::DEAD_TIME_DISABLED_
    }
    #[doc = "Checks if the value of the field is `DEAD_TIME_ENABLED_`"]
    #[inline]
    pub fn is_dead_time_enabled_(&self) -> bool {
        *self == DTE2R::DEAD_TIME_ENABLED_
    }
}
#[doc = "Possible values of the field `DISUP2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISUP2R {
    #[doc = "Functional registers are updated from the write registers at the end of each PWM cycle."]
    UPDATE,
    #[doc = "Functional registers remain the same as long as the timer is running."]
    NOUPDATE,
}
impl DISUP2R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            DISUP2R::UPDATE => false,
            DISUP2R::NOUPDATE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DISUP2R {
        match value {
            false => DISUP2R::UPDATE,
            true => DISUP2R::NOUPDATE,
        }
    }
    #[doc = "Checks if the value of the field is `UPDATE`"]
    #[inline]
    pub fn is_update(&self) -> bool {
        *self == DISUP2R::UPDATE
    }
    #[doc = "Checks if the value of the field is `NOUPDATE`"]
    #[inline]
    pub fn is_noupdate(&self) -> bool {
        *self == DISUP2R::NOUPDATE
    }
}
#[doc = "Possible values of the field `INVBDC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INVBDCR {
    #[doc = "The MCOB outputs have opposite polarity from the MCOA outputs (aside from dead time)."]
    OPPOSITE,
    #[doc = "The MCOB outputs have the same basic polarity as the MCOA outputs. (see Section 24.8.6)"]
    SAME,
}
impl INVBDCR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            INVBDCR::OPPOSITE => false,
            INVBDCR::SAME => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INVBDCR {
        match value {
            false => INVBDCR::OPPOSITE,
            true => INVBDCR::SAME,
        }
    }
    #[doc = "Checks if the value of the field is `OPPOSITE`"]
    #[inline]
    pub fn is_opposite(&self) -> bool {
        *self == INVBDCR::OPPOSITE
    }
    #[doc = "Checks if the value of the field is `SAME`"]
    #[inline]
    pub fn is_same(&self) -> bool {
        *self == INVBDCR::SAME
    }
}
#[doc = "Possible values of the field `ACMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMODER {
    #[doc = "3-phase AC-mode off: Each PWM channel uses its own timer-counter and period register."]
    _3_PHASE_AC_MODE_OFF,
    #[doc = "3-phase AC-mode on: All PWM channels use the timer-counter and period register of channel 0."]
    _3_PHASE_AC_MODE_ON_,
}
impl ACMODER {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            ACMODER::_3_PHASE_AC_MODE_OFF => false,
            ACMODER::_3_PHASE_AC_MODE_ON_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ACMODER {
        match value {
            false => ACMODER::_3_PHASE_AC_MODE_OFF,
            true => ACMODER::_3_PHASE_AC_MODE_ON_,
        }
    }
    #[doc = "Checks if the value of the field is `_3_PHASE_AC_MODE_OFF`"]
    #[inline]
    pub fn is_3_phase_ac_mode_off(&self) -> bool {
        *self == ACMODER::_3_PHASE_AC_MODE_OFF
    }
    #[doc = "Checks if the value of the field is `_3_PHASE_AC_MODE_ON_`"]
    #[inline]
    pub fn is_3_phase_ac_mode_on_(&self) -> bool {
        *self == ACMODER::_3_PHASE_AC_MODE_ON_
    }
}
#[doc = "Possible values of the field `DCMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCMODER {
    #[doc = "3-phase DC mode off: PWM channels are independent (unless bit ACMODE = 1)"]
    _3_PHASE_DC_MODE_OFF,
    #[doc = "3-phase DC mode on: The internal MCOA0 output is routed through the CP register (i.e. a mask) register to all six PWM outputs."]
    _3_PHASE_DC_MODE_ON_,
}
impl DCMODER {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            DCMODER::_3_PHASE_DC_MODE_OFF => false,
            DCMODER::_3_PHASE_DC_MODE_ON_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DCMODER {
        match value {
            false => DCMODER::_3_PHASE_DC_MODE_OFF,
            true => DCMODER::_3_PHASE_DC_MODE_ON_,
        }
    }
    #[doc = "Checks if the value of the field is `_3_PHASE_DC_MODE_OFF`"]
    #[inline]
    pub fn is_3_phase_dc_mode_off(&self) -> bool {
        *self == DCMODER::_3_PHASE_DC_MODE_OFF
    }
    #[doc = "Checks if the value of the field is `_3_PHASE_DC_MODE_ON_`"]
    #[inline]
    pub fn is_3_phase_dc_mode_on_(&self) -> bool {
        *self == DCMODER::_3_PHASE_DC_MODE_ON_
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Stops/starts timer channel 0."]
    #[inline]
    pub fn run0(&self) -> RUN0R {
        RUN0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Edge/center aligned operation for channel 0."]
    #[inline]
    pub fn center0(&self) -> CENTER0R {
        CENTER0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Selects polarity of the MCOA0 and MCOB0 pins."]
    #[inline]
    pub fn pola0(&self) -> POLA0R {
        POLA0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Controls the dead-time feature for channel 0."]
    #[inline]
    pub fn dte0(&self) -> DTE0R {
        DTE0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Enable/disable updates of functional registers for channel 0 (see Section 24.8.2)."]
    #[inline]
    pub fn disup0(&self) -> DISUP0R {
        DISUP0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Stops/starts timer channel 1."]
    #[inline]
    pub fn run1(&self) -> RUN1R {
        RUN1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Edge/center aligned operation for channel 1."]
    #[inline]
    pub fn center1(&self) -> CENTER1R {
        CENTER1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Selects polarity of the MCOA1 and MCOB1 pins."]
    #[inline]
    pub fn pola1(&self) -> POLA1R {
        POLA1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Controls the dead-time feature for channel 1."]
    #[inline]
    pub fn dte1(&self) -> DTE1R {
        DTE1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Enable/disable updates of functional registers for channel 1 (see Section 24.8.2)."]
    #[inline]
    pub fn disup1(&self) -> DISUP1R {
        DISUP1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Stops/starts timer channel 2."]
    #[inline]
    pub fn run2(&self) -> RUN2R {
        RUN2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Edge/center aligned operation for channel 2."]
    #[inline]
    pub fn center2(&self) -> CENTER2R {
        CENTER2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Selects polarity of the MCOA2 and MCOB2 pins."]
    #[inline]
    pub fn pola2(&self) -> POLA2R {
        POLA2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Controls the dead-time feature for channel 1."]
    #[inline]
    pub fn dte2(&self) -> DTE2R {
        DTE2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Enable/disable updates of functional registers for channel 2 (see Section 24.8.2)."]
    #[inline]
    pub fn disup2(&self) -> DISUP2R {
        DISUP2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Controls the polarity of the MCOB outputs for all 3 channels. This bit is typically set to 1 only in 3-phase DC mode."]
    #[inline]
    pub fn invbdc(&self) -> INVBDCR {
        INVBDCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - 3-phase AC mode select (see Section 24.8.7)."]
    #[inline]
    pub fn acmode(&self) -> ACMODER {
        ACMODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - 3-phase DC mode select (see Section 24.8.6)."]
    #[inline]
    pub fn dcmode(&self) -> DCMODER {
        DCMODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
