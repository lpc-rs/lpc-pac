#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INTEN {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `SEQA_INTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEQA_INTENR {
    #[doc = "Disabled. The sequence A interrupt/DMA trigger is disabled."]
    DISABLED,
    #[doc = "Enabled. The sequence A interrupt/DMA trigger is enabled and will be asserted either upon completion of each individual conversion performed as part of sequence A, or upon completion of the entire A sequence of conversions, depending on the MODE bit in the SEQA_CTRL register."]
    ENABLED,
}
impl SEQA_INTENR {
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
            SEQA_INTENR::DISABLED => false,
            SEQA_INTENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SEQA_INTENR {
        match value {
            false => SEQA_INTENR::DISABLED,
            true => SEQA_INTENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == SEQA_INTENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == SEQA_INTENR::ENABLED
    }
}
#[doc = "Possible values of the field `SEQB_INTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEQB_INTENR {
    #[doc = "Disabled. The sequence B interrupt/DMA trigger is disabled."]
    DISABLED,
    #[doc = "Enabled. The sequence B interrupt/DMA trigger is enabled and will be asserted either upon completion of each individual conversion performed as part of sequence B, or upon completion of the entire B sequence of conversions, depending on the MODE bit in the SEQB_CTRL register."]
    ENABLED,
}
impl SEQB_INTENR {
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
            SEQB_INTENR::DISABLED => false,
            SEQB_INTENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SEQB_INTENR {
        match value {
            false => SEQB_INTENR::DISABLED,
            true => SEQB_INTENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == SEQB_INTENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == SEQB_INTENR::ENABLED
    }
}
#[doc = "Possible values of the field `OVR_INTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVR_INTENR {
    #[doc = "Disabled. The overrun interrupt is disabled."]
    DISABLED,
    #[doc = "Enabled. The overrun interrupt is enabled. Detection of an overrun condition on any of the 12 channel data registers will cause an overrun interrupt request. In addition, if the MODE bit for a particular sequence is 0, then an overrun in the global data register for that sequence will also cause this interrupt request to be asserted."]
    ENABLED,
}
impl OVR_INTENR {
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
            OVR_INTENR::DISABLED => false,
            OVR_INTENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OVR_INTENR {
        match value {
            false => OVR_INTENR::DISABLED,
            true => OVR_INTENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == OVR_INTENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == OVR_INTENR::ENABLED
    }
}
#[doc = "Possible values of the field `ADCMPINTEN0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCMPINTEN0R {
    #[doc = "Disabled."]
    DISABLED,
    #[doc = "Outside threshold."]
    OUTSIDE_THRESHOLD,
    #[doc = "Crossing threshold."]
    CROSSING_THRESHOLD,
}
impl ADCMPINTEN0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ADCMPINTEN0R::DISABLED => 0,
            ADCMPINTEN0R::OUTSIDE_THRESHOLD => 1,
            ADCMPINTEN0R::CROSSING_THRESHOLD => 2,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ADCMPINTEN0R {
        match value {
            0 => ADCMPINTEN0R::DISABLED,
            1 => ADCMPINTEN0R::OUTSIDE_THRESHOLD,
            2 => ADCMPINTEN0R::CROSSING_THRESHOLD,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ADCMPINTEN0R::DISABLED
    }
    #[doc = "Checks if the value of the field is `OUTSIDE_THRESHOLD`"]
    #[inline]
    pub fn is_outside_threshold(&self) -> bool {
        *self == ADCMPINTEN0R::OUTSIDE_THRESHOLD
    }
    #[doc = "Checks if the value of the field is `CROSSING_THRESHOLD`"]
    #[inline]
    pub fn is_crossing_threshold(&self) -> bool {
        *self == ADCMPINTEN0R::CROSSING_THRESHOLD
    }
}
#[doc = "Possible values of the field `ADCMPINTEN1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCMPINTEN1R {
    #[doc = "Disabled."]
    DISABLED,
    #[doc = "Outside threshold."]
    OUTSIDE_THRESHOLD,
    #[doc = "Crossing threshold."]
    CROSSING_THRESHOLD,
}
impl ADCMPINTEN1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ADCMPINTEN1R::DISABLED => 0,
            ADCMPINTEN1R::OUTSIDE_THRESHOLD => 1,
            ADCMPINTEN1R::CROSSING_THRESHOLD => 2,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ADCMPINTEN1R {
        match value {
            0 => ADCMPINTEN1R::DISABLED,
            1 => ADCMPINTEN1R::OUTSIDE_THRESHOLD,
            2 => ADCMPINTEN1R::CROSSING_THRESHOLD,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ADCMPINTEN1R::DISABLED
    }
    #[doc = "Checks if the value of the field is `OUTSIDE_THRESHOLD`"]
    #[inline]
    pub fn is_outside_threshold(&self) -> bool {
        *self == ADCMPINTEN1R::OUTSIDE_THRESHOLD
    }
    #[doc = "Checks if the value of the field is `CROSSING_THRESHOLD`"]
    #[inline]
    pub fn is_crossing_threshold(&self) -> bool {
        *self == ADCMPINTEN1R::CROSSING_THRESHOLD
    }
}
#[doc = "Possible values of the field `ADCMPINTEN2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCMPINTEN2R {
    #[doc = "Disabled."]
    DISABLED,
    #[doc = "Outside threshold."]
    OUTSIDE_THRESHOLD,
    #[doc = "Crossing threshold."]
    CROSSING_THRESHOLD,
}
impl ADCMPINTEN2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ADCMPINTEN2R::DISABLED => 0,
            ADCMPINTEN2R::OUTSIDE_THRESHOLD => 1,
            ADCMPINTEN2R::CROSSING_THRESHOLD => 2,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ADCMPINTEN2R {
        match value {
            0 => ADCMPINTEN2R::DISABLED,
            1 => ADCMPINTEN2R::OUTSIDE_THRESHOLD,
            2 => ADCMPINTEN2R::CROSSING_THRESHOLD,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ADCMPINTEN2R::DISABLED
    }
    #[doc = "Checks if the value of the field is `OUTSIDE_THRESHOLD`"]
    #[inline]
    pub fn is_outside_threshold(&self) -> bool {
        *self == ADCMPINTEN2R::OUTSIDE_THRESHOLD
    }
    #[doc = "Checks if the value of the field is `CROSSING_THRESHOLD`"]
    #[inline]
    pub fn is_crossing_threshold(&self) -> bool {
        *self == ADCMPINTEN2R::CROSSING_THRESHOLD
    }
}
#[doc = "Possible values of the field `ADCMPINTEN3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCMPINTEN3R {
    #[doc = "Disabled."]
    DISABLED,
    #[doc = "Outside threshold."]
    OUTSIDE_THRESHOLD,
    #[doc = "Crossing threshold."]
    CROSSING_THRESHOLD,
}
impl ADCMPINTEN3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ADCMPINTEN3R::DISABLED => 0,
            ADCMPINTEN3R::OUTSIDE_THRESHOLD => 1,
            ADCMPINTEN3R::CROSSING_THRESHOLD => 2,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ADCMPINTEN3R {
        match value {
            0 => ADCMPINTEN3R::DISABLED,
            1 => ADCMPINTEN3R::OUTSIDE_THRESHOLD,
            2 => ADCMPINTEN3R::CROSSING_THRESHOLD,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ADCMPINTEN3R::DISABLED
    }
    #[doc = "Checks if the value of the field is `OUTSIDE_THRESHOLD`"]
    #[inline]
    pub fn is_outside_threshold(&self) -> bool {
        *self == ADCMPINTEN3R::OUTSIDE_THRESHOLD
    }
    #[doc = "Checks if the value of the field is `CROSSING_THRESHOLD`"]
    #[inline]
    pub fn is_crossing_threshold(&self) -> bool {
        *self == ADCMPINTEN3R::CROSSING_THRESHOLD
    }
}
#[doc = "Possible values of the field `ADCMPINTEN4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCMPINTEN4R {
    #[doc = "Disabled."]
    DISABLED,
    #[doc = "Outside threshold."]
    OUTSIDE_THRESHOLD,
    #[doc = "Crossing threshold."]
    CROSSING_THRESHOLD,
}
impl ADCMPINTEN4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ADCMPINTEN4R::DISABLED => 0,
            ADCMPINTEN4R::OUTSIDE_THRESHOLD => 1,
            ADCMPINTEN4R::CROSSING_THRESHOLD => 2,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ADCMPINTEN4R {
        match value {
            0 => ADCMPINTEN4R::DISABLED,
            1 => ADCMPINTEN4R::OUTSIDE_THRESHOLD,
            2 => ADCMPINTEN4R::CROSSING_THRESHOLD,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ADCMPINTEN4R::DISABLED
    }
    #[doc = "Checks if the value of the field is `OUTSIDE_THRESHOLD`"]
    #[inline]
    pub fn is_outside_threshold(&self) -> bool {
        *self == ADCMPINTEN4R::OUTSIDE_THRESHOLD
    }
    #[doc = "Checks if the value of the field is `CROSSING_THRESHOLD`"]
    #[inline]
    pub fn is_crossing_threshold(&self) -> bool {
        *self == ADCMPINTEN4R::CROSSING_THRESHOLD
    }
}
#[doc = "Possible values of the field `ADCMPINTEN5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCMPINTEN5R {
    #[doc = "Disabled."]
    DISABLED,
    #[doc = "Outside threshold."]
    OUTSIDE_THRESHOLD,
    #[doc = "Crossing threshold."]
    CROSSING_THRESHOLD,
}
impl ADCMPINTEN5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ADCMPINTEN5R::DISABLED => 0,
            ADCMPINTEN5R::OUTSIDE_THRESHOLD => 1,
            ADCMPINTEN5R::CROSSING_THRESHOLD => 2,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ADCMPINTEN5R {
        match value {
            0 => ADCMPINTEN5R::DISABLED,
            1 => ADCMPINTEN5R::OUTSIDE_THRESHOLD,
            2 => ADCMPINTEN5R::CROSSING_THRESHOLD,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ADCMPINTEN5R::DISABLED
    }
    #[doc = "Checks if the value of the field is `OUTSIDE_THRESHOLD`"]
    #[inline]
    pub fn is_outside_threshold(&self) -> bool {
        *self == ADCMPINTEN5R::OUTSIDE_THRESHOLD
    }
    #[doc = "Checks if the value of the field is `CROSSING_THRESHOLD`"]
    #[inline]
    pub fn is_crossing_threshold(&self) -> bool {
        *self == ADCMPINTEN5R::CROSSING_THRESHOLD
    }
}
#[doc = "Possible values of the field `ADCMPINTEN6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCMPINTEN6R {
    #[doc = "Disabled."]
    DISABLED,
    #[doc = "Outside threshold."]
    OUTSIDE_THRESHOLD,
    #[doc = "Crossing threshold."]
    CROSSING_THRESHOLD,
}
impl ADCMPINTEN6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ADCMPINTEN6R::DISABLED => 0,
            ADCMPINTEN6R::OUTSIDE_THRESHOLD => 1,
            ADCMPINTEN6R::CROSSING_THRESHOLD => 2,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ADCMPINTEN6R {
        match value {
            0 => ADCMPINTEN6R::DISABLED,
            1 => ADCMPINTEN6R::OUTSIDE_THRESHOLD,
            2 => ADCMPINTEN6R::CROSSING_THRESHOLD,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ADCMPINTEN6R::DISABLED
    }
    #[doc = "Checks if the value of the field is `OUTSIDE_THRESHOLD`"]
    #[inline]
    pub fn is_outside_threshold(&self) -> bool {
        *self == ADCMPINTEN6R::OUTSIDE_THRESHOLD
    }
    #[doc = "Checks if the value of the field is `CROSSING_THRESHOLD`"]
    #[inline]
    pub fn is_crossing_threshold(&self) -> bool {
        *self == ADCMPINTEN6R::CROSSING_THRESHOLD
    }
}
#[doc = "Possible values of the field `ADCMPINTEN7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCMPINTEN7R {
    #[doc = "Disabled."]
    DISABLED,
    #[doc = "Outside threshold."]
    OUTSIDE_THRESHOLD,
    #[doc = "Crossing threshold."]
    CROSSING_THRESHOLD,
}
impl ADCMPINTEN7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ADCMPINTEN7R::DISABLED => 0,
            ADCMPINTEN7R::OUTSIDE_THRESHOLD => 1,
            ADCMPINTEN7R::CROSSING_THRESHOLD => 2,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ADCMPINTEN7R {
        match value {
            0 => ADCMPINTEN7R::DISABLED,
            1 => ADCMPINTEN7R::OUTSIDE_THRESHOLD,
            2 => ADCMPINTEN7R::CROSSING_THRESHOLD,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ADCMPINTEN7R::DISABLED
    }
    #[doc = "Checks if the value of the field is `OUTSIDE_THRESHOLD`"]
    #[inline]
    pub fn is_outside_threshold(&self) -> bool {
        *self == ADCMPINTEN7R::OUTSIDE_THRESHOLD
    }
    #[doc = "Checks if the value of the field is `CROSSING_THRESHOLD`"]
    #[inline]
    pub fn is_crossing_threshold(&self) -> bool {
        *self == ADCMPINTEN7R::CROSSING_THRESHOLD
    }
}
#[doc = "Possible values of the field `ADCMPINTEN8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCMPINTEN8R {
    #[doc = "Disabled."]
    DISABLED,
    #[doc = "Outside threshold."]
    OUTSIDE_THRESHOLD,
    #[doc = "Crossing threshold."]
    CROSSING_THRESHOLD,
}
impl ADCMPINTEN8R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ADCMPINTEN8R::DISABLED => 0,
            ADCMPINTEN8R::OUTSIDE_THRESHOLD => 1,
            ADCMPINTEN8R::CROSSING_THRESHOLD => 2,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ADCMPINTEN8R {
        match value {
            0 => ADCMPINTEN8R::DISABLED,
            1 => ADCMPINTEN8R::OUTSIDE_THRESHOLD,
            2 => ADCMPINTEN8R::CROSSING_THRESHOLD,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ADCMPINTEN8R::DISABLED
    }
    #[doc = "Checks if the value of the field is `OUTSIDE_THRESHOLD`"]
    #[inline]
    pub fn is_outside_threshold(&self) -> bool {
        *self == ADCMPINTEN8R::OUTSIDE_THRESHOLD
    }
    #[doc = "Checks if the value of the field is `CROSSING_THRESHOLD`"]
    #[inline]
    pub fn is_crossing_threshold(&self) -> bool {
        *self == ADCMPINTEN8R::CROSSING_THRESHOLD
    }
}
#[doc = "Possible values of the field `ADCMPINTEN9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCMPINTEN9R {
    #[doc = "Disabled."]
    DISABLED,
    #[doc = "Outside threshold."]
    OUTSIDE_THRESHOLD,
    #[doc = "Crossing threshold."]
    CROSSING_THRESHOLD,
}
impl ADCMPINTEN9R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ADCMPINTEN9R::DISABLED => 0,
            ADCMPINTEN9R::OUTSIDE_THRESHOLD => 1,
            ADCMPINTEN9R::CROSSING_THRESHOLD => 2,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ADCMPINTEN9R {
        match value {
            0 => ADCMPINTEN9R::DISABLED,
            1 => ADCMPINTEN9R::OUTSIDE_THRESHOLD,
            2 => ADCMPINTEN9R::CROSSING_THRESHOLD,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ADCMPINTEN9R::DISABLED
    }
    #[doc = "Checks if the value of the field is `OUTSIDE_THRESHOLD`"]
    #[inline]
    pub fn is_outside_threshold(&self) -> bool {
        *self == ADCMPINTEN9R::OUTSIDE_THRESHOLD
    }
    #[doc = "Checks if the value of the field is `CROSSING_THRESHOLD`"]
    #[inline]
    pub fn is_crossing_threshold(&self) -> bool {
        *self == ADCMPINTEN9R::CROSSING_THRESHOLD
    }
}
#[doc = "Possible values of the field `ADCMPINTEN10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCMPINTEN10R {
    #[doc = "Disabled."]
    DISABLED,
    #[doc = "Outside threshold."]
    OUTSIDE_THRESHOLD,
    #[doc = "Crossing threshold."]
    CROSSING_THRESHOLD,
}
impl ADCMPINTEN10R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ADCMPINTEN10R::DISABLED => 0,
            ADCMPINTEN10R::OUTSIDE_THRESHOLD => 1,
            ADCMPINTEN10R::CROSSING_THRESHOLD => 2,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ADCMPINTEN10R {
        match value {
            0 => ADCMPINTEN10R::DISABLED,
            1 => ADCMPINTEN10R::OUTSIDE_THRESHOLD,
            2 => ADCMPINTEN10R::CROSSING_THRESHOLD,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ADCMPINTEN10R::DISABLED
    }
    #[doc = "Checks if the value of the field is `OUTSIDE_THRESHOLD`"]
    #[inline]
    pub fn is_outside_threshold(&self) -> bool {
        *self == ADCMPINTEN10R::OUTSIDE_THRESHOLD
    }
    #[doc = "Checks if the value of the field is `CROSSING_THRESHOLD`"]
    #[inline]
    pub fn is_crossing_threshold(&self) -> bool {
        *self == ADCMPINTEN10R::CROSSING_THRESHOLD
    }
}
#[doc = "Possible values of the field `ADCMPINTEN11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCMPINTEN11R {
    #[doc = "Disabled."]
    DISABLED,
    #[doc = "Outside threshold."]
    OUTSIDE_THRESHOLD,
    #[doc = "Crossing threshold."]
    CROSSING_THRESHOLD,
}
impl ADCMPINTEN11R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ADCMPINTEN11R::DISABLED => 0,
            ADCMPINTEN11R::OUTSIDE_THRESHOLD => 1,
            ADCMPINTEN11R::CROSSING_THRESHOLD => 2,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ADCMPINTEN11R {
        match value {
            0 => ADCMPINTEN11R::DISABLED,
            1 => ADCMPINTEN11R::OUTSIDE_THRESHOLD,
            2 => ADCMPINTEN11R::CROSSING_THRESHOLD,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ADCMPINTEN11R::DISABLED
    }
    #[doc = "Checks if the value of the field is `OUTSIDE_THRESHOLD`"]
    #[inline]
    pub fn is_outside_threshold(&self) -> bool {
        *self == ADCMPINTEN11R::OUTSIDE_THRESHOLD
    }
    #[doc = "Checks if the value of the field is `CROSSING_THRESHOLD`"]
    #[inline]
    pub fn is_crossing_threshold(&self) -> bool {
        *self == ADCMPINTEN11R::CROSSING_THRESHOLD
    }
}
#[doc = "Values that can be written to the field `SEQA_INTEN`"]
pub enum SEQA_INTENW {
    #[doc = "Disabled. The sequence A interrupt/DMA trigger is disabled."]
    DISABLED,
    #[doc = "Enabled. The sequence A interrupt/DMA trigger is enabled and will be asserted either upon completion of each individual conversion performed as part of sequence A, or upon completion of the entire A sequence of conversions, depending on the MODE bit in the SEQA_CTRL register."]
    ENABLED,
}
impl SEQA_INTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SEQA_INTENW::DISABLED => false,
            SEQA_INTENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SEQA_INTENW<'a> {
    w: &'a mut W,
}
impl<'a> _SEQA_INTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SEQA_INTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. The sequence A interrupt/DMA trigger is disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SEQA_INTENW::DISABLED)
    }
    #[doc = "Enabled. The sequence A interrupt/DMA trigger is enabled and will be asserted either upon completion of each individual conversion performed as part of sequence A, or upon completion of the entire A sequence of conversions, depending on the MODE bit in the SEQA_CTRL register."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SEQA_INTENW::ENABLED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SEQB_INTEN`"]
pub enum SEQB_INTENW {
    #[doc = "Disabled. The sequence B interrupt/DMA trigger is disabled."]
    DISABLED,
    #[doc = "Enabled. The sequence B interrupt/DMA trigger is enabled and will be asserted either upon completion of each individual conversion performed as part of sequence B, or upon completion of the entire B sequence of conversions, depending on the MODE bit in the SEQB_CTRL register."]
    ENABLED,
}
impl SEQB_INTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SEQB_INTENW::DISABLED => false,
            SEQB_INTENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SEQB_INTENW<'a> {
    w: &'a mut W,
}
impl<'a> _SEQB_INTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SEQB_INTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. The sequence B interrupt/DMA trigger is disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SEQB_INTENW::DISABLED)
    }
    #[doc = "Enabled. The sequence B interrupt/DMA trigger is enabled and will be asserted either upon completion of each individual conversion performed as part of sequence B, or upon completion of the entire B sequence of conversions, depending on the MODE bit in the SEQB_CTRL register."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SEQB_INTENW::ENABLED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OVR_INTEN`"]
pub enum OVR_INTENW {
    #[doc = "Disabled. The overrun interrupt is disabled."]
    DISABLED,
    #[doc = "Enabled. The overrun interrupt is enabled. Detection of an overrun condition on any of the 12 channel data registers will cause an overrun interrupt request. In addition, if the MODE bit for a particular sequence is 0, then an overrun in the global data register for that sequence will also cause this interrupt request to be asserted."]
    ENABLED,
}
impl OVR_INTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OVR_INTENW::DISABLED => false,
            OVR_INTENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OVR_INTENW<'a> {
    w: &'a mut W,
}
impl<'a> _OVR_INTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OVR_INTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. The overrun interrupt is disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OVR_INTENW::DISABLED)
    }
    #[doc = "Enabled. The overrun interrupt is enabled. Detection of an overrun condition on any of the 12 channel data registers will cause an overrun interrupt request. In addition, if the MODE bit for a particular sequence is 0, then an overrun in the global data register for that sequence will also cause this interrupt request to be asserted."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OVR_INTENW::ENABLED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ADCMPINTEN0`"]
pub enum ADCMPINTEN0W {
    #[doc = "Disabled."]
    DISABLED,
    #[doc = "Outside threshold."]
    OUTSIDE_THRESHOLD,
    #[doc = "Crossing threshold."]
    CROSSING_THRESHOLD,
}
impl ADCMPINTEN0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADCMPINTEN0W::DISABLED => 0,
            ADCMPINTEN0W::OUTSIDE_THRESHOLD => 1,
            ADCMPINTEN0W::CROSSING_THRESHOLD => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADCMPINTEN0W<'a> {
    w: &'a mut W,
}
impl<'a> _ADCMPINTEN0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADCMPINTEN0W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADCMPINTEN0W::DISABLED)
    }
    #[doc = "Outside threshold."]
    #[inline]
    pub fn outside_threshold(self) -> &'a mut W {
        self.variant(ADCMPINTEN0W::OUTSIDE_THRESHOLD)
    }
    #[doc = "Crossing threshold."]
    #[inline]
    pub fn crossing_threshold(self) -> &'a mut W {
        self.variant(ADCMPINTEN0W::CROSSING_THRESHOLD)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ADCMPINTEN1`"]
pub enum ADCMPINTEN1W {
    #[doc = "Disabled."]
    DISABLED,
    #[doc = "Outside threshold."]
    OUTSIDE_THRESHOLD,
    #[doc = "Crossing threshold."]
    CROSSING_THRESHOLD,
}
impl ADCMPINTEN1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADCMPINTEN1W::DISABLED => 0,
            ADCMPINTEN1W::OUTSIDE_THRESHOLD => 1,
            ADCMPINTEN1W::CROSSING_THRESHOLD => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADCMPINTEN1W<'a> {
    w: &'a mut W,
}
impl<'a> _ADCMPINTEN1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADCMPINTEN1W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADCMPINTEN1W::DISABLED)
    }
    #[doc = "Outside threshold."]
    #[inline]
    pub fn outside_threshold(self) -> &'a mut W {
        self.variant(ADCMPINTEN1W::OUTSIDE_THRESHOLD)
    }
    #[doc = "Crossing threshold."]
    #[inline]
    pub fn crossing_threshold(self) -> &'a mut W {
        self.variant(ADCMPINTEN1W::CROSSING_THRESHOLD)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ADCMPINTEN2`"]
pub enum ADCMPINTEN2W {
    #[doc = "Disabled."]
    DISABLED,
    #[doc = "Outside threshold."]
    OUTSIDE_THRESHOLD,
    #[doc = "Crossing threshold."]
    CROSSING_THRESHOLD,
}
impl ADCMPINTEN2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADCMPINTEN2W::DISABLED => 0,
            ADCMPINTEN2W::OUTSIDE_THRESHOLD => 1,
            ADCMPINTEN2W::CROSSING_THRESHOLD => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADCMPINTEN2W<'a> {
    w: &'a mut W,
}
impl<'a> _ADCMPINTEN2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADCMPINTEN2W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADCMPINTEN2W::DISABLED)
    }
    #[doc = "Outside threshold."]
    #[inline]
    pub fn outside_threshold(self) -> &'a mut W {
        self.variant(ADCMPINTEN2W::OUTSIDE_THRESHOLD)
    }
    #[doc = "Crossing threshold."]
    #[inline]
    pub fn crossing_threshold(self) -> &'a mut W {
        self.variant(ADCMPINTEN2W::CROSSING_THRESHOLD)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ADCMPINTEN3`"]
pub enum ADCMPINTEN3W {
    #[doc = "Disabled."]
    DISABLED,
    #[doc = "Outside threshold."]
    OUTSIDE_THRESHOLD,
    #[doc = "Crossing threshold."]
    CROSSING_THRESHOLD,
}
impl ADCMPINTEN3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADCMPINTEN3W::DISABLED => 0,
            ADCMPINTEN3W::OUTSIDE_THRESHOLD => 1,
            ADCMPINTEN3W::CROSSING_THRESHOLD => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADCMPINTEN3W<'a> {
    w: &'a mut W,
}
impl<'a> _ADCMPINTEN3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADCMPINTEN3W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADCMPINTEN3W::DISABLED)
    }
    #[doc = "Outside threshold."]
    #[inline]
    pub fn outside_threshold(self) -> &'a mut W {
        self.variant(ADCMPINTEN3W::OUTSIDE_THRESHOLD)
    }
    #[doc = "Crossing threshold."]
    #[inline]
    pub fn crossing_threshold(self) -> &'a mut W {
        self.variant(ADCMPINTEN3W::CROSSING_THRESHOLD)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ADCMPINTEN4`"]
pub enum ADCMPINTEN4W {
    #[doc = "Disabled."]
    DISABLED,
    #[doc = "Outside threshold."]
    OUTSIDE_THRESHOLD,
    #[doc = "Crossing threshold."]
    CROSSING_THRESHOLD,
}
impl ADCMPINTEN4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADCMPINTEN4W::DISABLED => 0,
            ADCMPINTEN4W::OUTSIDE_THRESHOLD => 1,
            ADCMPINTEN4W::CROSSING_THRESHOLD => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADCMPINTEN4W<'a> {
    w: &'a mut W,
}
impl<'a> _ADCMPINTEN4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADCMPINTEN4W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADCMPINTEN4W::DISABLED)
    }
    #[doc = "Outside threshold."]
    #[inline]
    pub fn outside_threshold(self) -> &'a mut W {
        self.variant(ADCMPINTEN4W::OUTSIDE_THRESHOLD)
    }
    #[doc = "Crossing threshold."]
    #[inline]
    pub fn crossing_threshold(self) -> &'a mut W {
        self.variant(ADCMPINTEN4W::CROSSING_THRESHOLD)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ADCMPINTEN5`"]
pub enum ADCMPINTEN5W {
    #[doc = "Disabled."]
    DISABLED,
    #[doc = "Outside threshold."]
    OUTSIDE_THRESHOLD,
    #[doc = "Crossing threshold."]
    CROSSING_THRESHOLD,
}
impl ADCMPINTEN5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADCMPINTEN5W::DISABLED => 0,
            ADCMPINTEN5W::OUTSIDE_THRESHOLD => 1,
            ADCMPINTEN5W::CROSSING_THRESHOLD => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADCMPINTEN5W<'a> {
    w: &'a mut W,
}
impl<'a> _ADCMPINTEN5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADCMPINTEN5W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADCMPINTEN5W::DISABLED)
    }
    #[doc = "Outside threshold."]
    #[inline]
    pub fn outside_threshold(self) -> &'a mut W {
        self.variant(ADCMPINTEN5W::OUTSIDE_THRESHOLD)
    }
    #[doc = "Crossing threshold."]
    #[inline]
    pub fn crossing_threshold(self) -> &'a mut W {
        self.variant(ADCMPINTEN5W::CROSSING_THRESHOLD)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ADCMPINTEN6`"]
pub enum ADCMPINTEN6W {
    #[doc = "Disabled."]
    DISABLED,
    #[doc = "Outside threshold."]
    OUTSIDE_THRESHOLD,
    #[doc = "Crossing threshold."]
    CROSSING_THRESHOLD,
}
impl ADCMPINTEN6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADCMPINTEN6W::DISABLED => 0,
            ADCMPINTEN6W::OUTSIDE_THRESHOLD => 1,
            ADCMPINTEN6W::CROSSING_THRESHOLD => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADCMPINTEN6W<'a> {
    w: &'a mut W,
}
impl<'a> _ADCMPINTEN6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADCMPINTEN6W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADCMPINTEN6W::DISABLED)
    }
    #[doc = "Outside threshold."]
    #[inline]
    pub fn outside_threshold(self) -> &'a mut W {
        self.variant(ADCMPINTEN6W::OUTSIDE_THRESHOLD)
    }
    #[doc = "Crossing threshold."]
    #[inline]
    pub fn crossing_threshold(self) -> &'a mut W {
        self.variant(ADCMPINTEN6W::CROSSING_THRESHOLD)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ADCMPINTEN7`"]
pub enum ADCMPINTEN7W {
    #[doc = "Disabled."]
    DISABLED,
    #[doc = "Outside threshold."]
    OUTSIDE_THRESHOLD,
    #[doc = "Crossing threshold."]
    CROSSING_THRESHOLD,
}
impl ADCMPINTEN7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADCMPINTEN7W::DISABLED => 0,
            ADCMPINTEN7W::OUTSIDE_THRESHOLD => 1,
            ADCMPINTEN7W::CROSSING_THRESHOLD => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADCMPINTEN7W<'a> {
    w: &'a mut W,
}
impl<'a> _ADCMPINTEN7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADCMPINTEN7W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADCMPINTEN7W::DISABLED)
    }
    #[doc = "Outside threshold."]
    #[inline]
    pub fn outside_threshold(self) -> &'a mut W {
        self.variant(ADCMPINTEN7W::OUTSIDE_THRESHOLD)
    }
    #[doc = "Crossing threshold."]
    #[inline]
    pub fn crossing_threshold(self) -> &'a mut W {
        self.variant(ADCMPINTEN7W::CROSSING_THRESHOLD)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ADCMPINTEN8`"]
pub enum ADCMPINTEN8W {
    #[doc = "Disabled."]
    DISABLED,
    #[doc = "Outside threshold."]
    OUTSIDE_THRESHOLD,
    #[doc = "Crossing threshold."]
    CROSSING_THRESHOLD,
}
impl ADCMPINTEN8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADCMPINTEN8W::DISABLED => 0,
            ADCMPINTEN8W::OUTSIDE_THRESHOLD => 1,
            ADCMPINTEN8W::CROSSING_THRESHOLD => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADCMPINTEN8W<'a> {
    w: &'a mut W,
}
impl<'a> _ADCMPINTEN8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADCMPINTEN8W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADCMPINTEN8W::DISABLED)
    }
    #[doc = "Outside threshold."]
    #[inline]
    pub fn outside_threshold(self) -> &'a mut W {
        self.variant(ADCMPINTEN8W::OUTSIDE_THRESHOLD)
    }
    #[doc = "Crossing threshold."]
    #[inline]
    pub fn crossing_threshold(self) -> &'a mut W {
        self.variant(ADCMPINTEN8W::CROSSING_THRESHOLD)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ADCMPINTEN9`"]
pub enum ADCMPINTEN9W {
    #[doc = "Disabled."]
    DISABLED,
    #[doc = "Outside threshold."]
    OUTSIDE_THRESHOLD,
    #[doc = "Crossing threshold."]
    CROSSING_THRESHOLD,
}
impl ADCMPINTEN9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADCMPINTEN9W::DISABLED => 0,
            ADCMPINTEN9W::OUTSIDE_THRESHOLD => 1,
            ADCMPINTEN9W::CROSSING_THRESHOLD => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADCMPINTEN9W<'a> {
    w: &'a mut W,
}
impl<'a> _ADCMPINTEN9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADCMPINTEN9W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADCMPINTEN9W::DISABLED)
    }
    #[doc = "Outside threshold."]
    #[inline]
    pub fn outside_threshold(self) -> &'a mut W {
        self.variant(ADCMPINTEN9W::OUTSIDE_THRESHOLD)
    }
    #[doc = "Crossing threshold."]
    #[inline]
    pub fn crossing_threshold(self) -> &'a mut W {
        self.variant(ADCMPINTEN9W::CROSSING_THRESHOLD)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ADCMPINTEN10`"]
pub enum ADCMPINTEN10W {
    #[doc = "Disabled."]
    DISABLED,
    #[doc = "Outside threshold."]
    OUTSIDE_THRESHOLD,
    #[doc = "Crossing threshold."]
    CROSSING_THRESHOLD,
}
impl ADCMPINTEN10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADCMPINTEN10W::DISABLED => 0,
            ADCMPINTEN10W::OUTSIDE_THRESHOLD => 1,
            ADCMPINTEN10W::CROSSING_THRESHOLD => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADCMPINTEN10W<'a> {
    w: &'a mut W,
}
impl<'a> _ADCMPINTEN10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADCMPINTEN10W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADCMPINTEN10W::DISABLED)
    }
    #[doc = "Outside threshold."]
    #[inline]
    pub fn outside_threshold(self) -> &'a mut W {
        self.variant(ADCMPINTEN10W::OUTSIDE_THRESHOLD)
    }
    #[doc = "Crossing threshold."]
    #[inline]
    pub fn crossing_threshold(self) -> &'a mut W {
        self.variant(ADCMPINTEN10W::CROSSING_THRESHOLD)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ADCMPINTEN11`"]
pub enum ADCMPINTEN11W {
    #[doc = "Disabled."]
    DISABLED,
    #[doc = "Outside threshold."]
    OUTSIDE_THRESHOLD,
    #[doc = "Crossing threshold."]
    CROSSING_THRESHOLD,
}
impl ADCMPINTEN11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADCMPINTEN11W::DISABLED => 0,
            ADCMPINTEN11W::OUTSIDE_THRESHOLD => 1,
            ADCMPINTEN11W::CROSSING_THRESHOLD => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADCMPINTEN11W<'a> {
    w: &'a mut W,
}
impl<'a> _ADCMPINTEN11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADCMPINTEN11W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADCMPINTEN11W::DISABLED)
    }
    #[doc = "Outside threshold."]
    #[inline]
    pub fn outside_threshold(self) -> &'a mut W {
        self.variant(ADCMPINTEN11W::OUTSIDE_THRESHOLD)
    }
    #[doc = "Crossing threshold."]
    #[inline]
    pub fn crossing_threshold(self) -> &'a mut W {
        self.variant(ADCMPINTEN11W::CROSSING_THRESHOLD)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Sequence A interrupt enable."]
    #[inline]
    pub fn seqa_inten(&self) -> SEQA_INTENR {
        SEQA_INTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Sequence B interrupt enable."]
    #[inline]
    pub fn seqb_inten(&self) -> SEQB_INTENR {
        SEQB_INTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Overrun interrupt enable."]
    #[inline]
    pub fn ovr_inten(&self) -> OVR_INTENR {
        OVR_INTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 3:4 - Threshold comparison interrupt enable."]
    #[inline]
    pub fn adcmpinten0(&self) -> ADCMPINTEN0R {
        ADCMPINTEN0R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 5:6 - Threshold comparison interrupt enable."]
    #[inline]
    pub fn adcmpinten1(&self) -> ADCMPINTEN1R {
        ADCMPINTEN1R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 7:8 - Threshold comparison interrupt enable."]
    #[inline]
    pub fn adcmpinten2(&self) -> ADCMPINTEN2R {
        ADCMPINTEN2R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 9:10 - Threshold comparison interrupt enable."]
    #[inline]
    pub fn adcmpinten3(&self) -> ADCMPINTEN3R {
        ADCMPINTEN3R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 11:12 - Threshold comparison interrupt enable."]
    #[inline]
    pub fn adcmpinten4(&self) -> ADCMPINTEN4R {
        ADCMPINTEN4R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 13:14 - Threshold comparison interrupt enable."]
    #[inline]
    pub fn adcmpinten5(&self) -> ADCMPINTEN5R {
        ADCMPINTEN5R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 15:16 - Threshold comparison interrupt enable."]
    #[inline]
    pub fn adcmpinten6(&self) -> ADCMPINTEN6R {
        ADCMPINTEN6R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 17:18 - Threshold comparison interrupt enable."]
    #[inline]
    pub fn adcmpinten7(&self) -> ADCMPINTEN7R {
        ADCMPINTEN7R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 19:20 - Threshold comparison interrupt enable."]
    #[inline]
    pub fn adcmpinten8(&self) -> ADCMPINTEN8R {
        ADCMPINTEN8R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 21:22 - Threshold comparison interrupt enable."]
    #[inline]
    pub fn adcmpinten9(&self) -> ADCMPINTEN9R {
        ADCMPINTEN9R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 23:24 - Threshold comparison interrupt enable."]
    #[inline]
    pub fn adcmpinten10(&self) -> ADCMPINTEN10R {
        ADCMPINTEN10R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 25:26 - Threshold comparison interrupt enable."]
    #[inline]
    pub fn adcmpinten11(&self) -> ADCMPINTEN11R {
        ADCMPINTEN11R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Sequence A interrupt enable."]
    #[inline]
    pub fn seqa_inten(&mut self) -> _SEQA_INTENW {
        _SEQA_INTENW { w: self }
    }
    #[doc = "Bit 1 - Sequence B interrupt enable."]
    #[inline]
    pub fn seqb_inten(&mut self) -> _SEQB_INTENW {
        _SEQB_INTENW { w: self }
    }
    #[doc = "Bit 2 - Overrun interrupt enable."]
    #[inline]
    pub fn ovr_inten(&mut self) -> _OVR_INTENW {
        _OVR_INTENW { w: self }
    }
    #[doc = "Bits 3:4 - Threshold comparison interrupt enable."]
    #[inline]
    pub fn adcmpinten0(&mut self) -> _ADCMPINTEN0W {
        _ADCMPINTEN0W { w: self }
    }
    #[doc = "Bits 5:6 - Threshold comparison interrupt enable."]
    #[inline]
    pub fn adcmpinten1(&mut self) -> _ADCMPINTEN1W {
        _ADCMPINTEN1W { w: self }
    }
    #[doc = "Bits 7:8 - Threshold comparison interrupt enable."]
    #[inline]
    pub fn adcmpinten2(&mut self) -> _ADCMPINTEN2W {
        _ADCMPINTEN2W { w: self }
    }
    #[doc = "Bits 9:10 - Threshold comparison interrupt enable."]
    #[inline]
    pub fn adcmpinten3(&mut self) -> _ADCMPINTEN3W {
        _ADCMPINTEN3W { w: self }
    }
    #[doc = "Bits 11:12 - Threshold comparison interrupt enable."]
    #[inline]
    pub fn adcmpinten4(&mut self) -> _ADCMPINTEN4W {
        _ADCMPINTEN4W { w: self }
    }
    #[doc = "Bits 13:14 - Threshold comparison interrupt enable."]
    #[inline]
    pub fn adcmpinten5(&mut self) -> _ADCMPINTEN5W {
        _ADCMPINTEN5W { w: self }
    }
    #[doc = "Bits 15:16 - Threshold comparison interrupt enable."]
    #[inline]
    pub fn adcmpinten6(&mut self) -> _ADCMPINTEN6W {
        _ADCMPINTEN6W { w: self }
    }
    #[doc = "Bits 17:18 - Threshold comparison interrupt enable."]
    #[inline]
    pub fn adcmpinten7(&mut self) -> _ADCMPINTEN7W {
        _ADCMPINTEN7W { w: self }
    }
    #[doc = "Bits 19:20 - Threshold comparison interrupt enable."]
    #[inline]
    pub fn adcmpinten8(&mut self) -> _ADCMPINTEN8W {
        _ADCMPINTEN8W { w: self }
    }
    #[doc = "Bits 21:22 - Threshold comparison interrupt enable."]
    #[inline]
    pub fn adcmpinten9(&mut self) -> _ADCMPINTEN9W {
        _ADCMPINTEN9W { w: self }
    }
    #[doc = "Bits 23:24 - Threshold comparison interrupt enable."]
    #[inline]
    pub fn adcmpinten10(&mut self) -> _ADCMPINTEN10W {
        _ADCMPINTEN10W { w: self }
    }
    #[doc = "Bits 25:26 - Threshold comparison interrupt enable."]
    #[inline]
    pub fn adcmpinten11(&mut self) -> _ADCMPINTEN11W {
        _ADCMPINTEN11W { w: self }
    }
}
