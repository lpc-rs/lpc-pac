#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INTEN {
    #[doc = r"Modifies the contents of the register"]
    #[inline(always)]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        self.register.set(f(&R { bits }, &mut W { bits }).bits);
    }
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r"Writes to the register"]
    #[inline(always)]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        self.register.set(
            f(&mut W {
                bits: Self::reset_value(),
            })
            .bits,
        );
    }
    #[doc = r"Reset value of the register"]
    #[inline(always)]
    pub const fn reset_value() -> u32 {
        0
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = "Possible values of the field `SEQA_INTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEQA_INTENR {
    #[doc = "Disabled. The sequence A interrupt/DMA request is disabled."]
    DISABLED,
    #[doc = "Enabled. The sequence A interrupt/DMA request is enabled and will be asserted either upon completion of each individual conversion performed as part of sequence A, or upon completion of the entire A sequence of conversions, depending on the MODE bit in the SEQA_CTRL register."]
    ENABLED,
}
impl crate::ToBits<bool> for SEQA_INTENR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            SEQA_INTENR::DISABLED => false,
            SEQA_INTENR::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SEQA_INTEN_R = crate::FR<bool, SEQA_INTENR>;
impl SEQA_INTEN_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SEQA_INTENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SEQA_INTENR::ENABLED
    }
}
#[doc = "Values that can be written to the field `SEQA_INTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEQA_INTENW {
    #[doc = "Disabled. The sequence A interrupt/DMA request is disabled."]
    DISABLED,
    #[doc = "Enabled. The sequence A interrupt/DMA request is enabled and will be asserted either upon completion of each individual conversion performed as part of sequence A, or upon completion of the entire A sequence of conversions, depending on the MODE bit in the SEQA_CTRL register."]
    ENABLED,
}
impl SEQA_INTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            SEQA_INTENW::DISABLED => false,
            SEQA_INTENW::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SEQA_INTENW<'a> {
    w: &'a mut W,
}
impl<'a> _SEQA_INTENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEQA_INTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. The sequence A interrupt/DMA request is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SEQA_INTENW::DISABLED)
    }
    #[doc = "Enabled. The sequence A interrupt/DMA request is enabled and will be asserted either upon completion of each individual conversion performed as part of sequence A, or upon completion of the entire A sequence of conversions, depending on the MODE bit in the SEQA_CTRL register."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SEQA_INTENW::ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Possible values of the field `SEQB_INTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEQB_INTENR {
    #[doc = "Disabled. The sequence B interrupt/DMA request is disabled."]
    DISABLED,
    #[doc = "Enabled. The sequence B interrupt/DMA request is enabled and will be asserted either upon completion of each individual conversion performed as part of sequence B, or upon completion of the entire B sequence of conversions, depending on the MODE bit in the SEQB_CTRL register."]
    ENABLED,
}
impl crate::ToBits<bool> for SEQB_INTENR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            SEQB_INTENR::DISABLED => false,
            SEQB_INTENR::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SEQB_INTEN_R = crate::FR<bool, SEQB_INTENR>;
impl SEQB_INTEN_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SEQB_INTENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SEQB_INTENR::ENABLED
    }
}
#[doc = "Values that can be written to the field `SEQB_INTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEQB_INTENW {
    #[doc = "Disabled. The sequence B interrupt/DMA request is disabled."]
    DISABLED,
    #[doc = "Enabled. The sequence B interrupt/DMA request is enabled and will be asserted either upon completion of each individual conversion performed as part of sequence B, or upon completion of the entire B sequence of conversions, depending on the MODE bit in the SEQB_CTRL register."]
    ENABLED,
}
impl SEQB_INTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            SEQB_INTENW::DISABLED => false,
            SEQB_INTENW::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SEQB_INTENW<'a> {
    w: &'a mut W,
}
impl<'a> _SEQB_INTENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEQB_INTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. The sequence B interrupt/DMA request is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SEQB_INTENW::DISABLED)
    }
    #[doc = "Enabled. The sequence B interrupt/DMA request is enabled and will be asserted either upon completion of each individual conversion performed as part of sequence B, or upon completion of the entire B sequence of conversions, depending on the MODE bit in the SEQB_CTRL register."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SEQB_INTENW::ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
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
impl crate::ToBits<bool> for OVR_INTENR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            OVR_INTENR::DISABLED => false,
            OVR_INTENR::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type OVR_INTEN_R = crate::FR<bool, OVR_INTENR>;
impl OVR_INTEN_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OVR_INTENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OVR_INTENR::ENABLED
    }
}
#[doc = "Values that can be written to the field `OVR_INTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVR_INTENW {
    #[doc = "Disabled. The overrun interrupt is disabled."]
    DISABLED,
    #[doc = "Enabled. The overrun interrupt is enabled. Detection of an overrun condition on any of the 12 channel data registers will cause an overrun interrupt request. In addition, if the MODE bit for a particular sequence is 0, then an overrun in the global data register for that sequence will also cause this interrupt request to be asserted."]
    ENABLED,
}
impl OVR_INTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            OVR_INTENW::DISABLED => false,
            OVR_INTENW::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _OVR_INTENW<'a> {
    w: &'a mut W,
}
impl<'a> _OVR_INTENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OVR_INTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. The overrun interrupt is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OVR_INTENW::DISABLED)
    }
    #[doc = "Enabled. The overrun interrupt is enabled. Detection of an overrun condition on any of the 12 channel data registers will cause an overrun interrupt request. In addition, if the MODE bit for a particular sequence is 0, then an overrun in the global data register for that sequence will also cause this interrupt request to be asserted."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OVR_INTENW::ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
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
impl crate::ToBits<u8> for ADCMPINTEN0R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            ADCMPINTEN0R::DISABLED => 0,
            ADCMPINTEN0R::OUTSIDE_THRESHOLD => 1,
            ADCMPINTEN0R::CROSSING_THRESHOLD => 2,
        }
    }
}
#[doc = r"Reader of the field"]
pub type ADCMPINTEN0_R = crate::FR<u8, ADCMPINTEN0R>;
impl ADCMPINTEN0_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADCMPINTEN0R::DISABLED
    }
    #[doc = "Checks if the value of the field is `OUTSIDE_THRESHOLD`"]
    #[inline(always)]
    pub fn is_outside_threshold(&self) -> bool {
        *self == ADCMPINTEN0R::OUTSIDE_THRESHOLD
    }
    #[doc = "Checks if the value of the field is `CROSSING_THRESHOLD`"]
    #[inline(always)]
    pub fn is_crossing_threshold(&self) -> bool {
        *self == ADCMPINTEN0R::CROSSING_THRESHOLD
    }
}
#[doc = "Values that can be written to the field `ADCMPINTEN0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADCMPINTEN0W::DISABLED => 0,
            ADCMPINTEN0W::OUTSIDE_THRESHOLD => 1,
            ADCMPINTEN0W::CROSSING_THRESHOLD => 2,
        }
    }
}
#[doc = r"Proxy"]
pub struct _ADCMPINTEN0W<'a> {
    w: &'a mut W,
}
impl<'a> _ADCMPINTEN0W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCMPINTEN0W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADCMPINTEN0W::DISABLED)
    }
    #[doc = "Outside threshold."]
    #[inline(always)]
    pub fn outside_threshold(self) -> &'a mut W {
        self.variant(ADCMPINTEN0W::OUTSIDE_THRESHOLD)
    }
    #[doc = "Crossing threshold."]
    #[inline(always)]
    pub fn crossing_threshold(self) -> &'a mut W {
        self.variant(ADCMPINTEN0W::CROSSING_THRESHOLD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
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
impl crate::ToBits<u8> for ADCMPINTEN1R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            ADCMPINTEN1R::DISABLED => 0,
            ADCMPINTEN1R::OUTSIDE_THRESHOLD => 1,
            ADCMPINTEN1R::CROSSING_THRESHOLD => 2,
        }
    }
}
#[doc = r"Reader of the field"]
pub type ADCMPINTEN1_R = crate::FR<u8, ADCMPINTEN1R>;
impl ADCMPINTEN1_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADCMPINTEN1R::DISABLED
    }
    #[doc = "Checks if the value of the field is `OUTSIDE_THRESHOLD`"]
    #[inline(always)]
    pub fn is_outside_threshold(&self) -> bool {
        *self == ADCMPINTEN1R::OUTSIDE_THRESHOLD
    }
    #[doc = "Checks if the value of the field is `CROSSING_THRESHOLD`"]
    #[inline(always)]
    pub fn is_crossing_threshold(&self) -> bool {
        *self == ADCMPINTEN1R::CROSSING_THRESHOLD
    }
}
#[doc = "Values that can be written to the field `ADCMPINTEN1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADCMPINTEN1W::DISABLED => 0,
            ADCMPINTEN1W::OUTSIDE_THRESHOLD => 1,
            ADCMPINTEN1W::CROSSING_THRESHOLD => 2,
        }
    }
}
#[doc = r"Proxy"]
pub struct _ADCMPINTEN1W<'a> {
    w: &'a mut W,
}
impl<'a> _ADCMPINTEN1W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCMPINTEN1W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADCMPINTEN1W::DISABLED)
    }
    #[doc = "Outside threshold."]
    #[inline(always)]
    pub fn outside_threshold(self) -> &'a mut W {
        self.variant(ADCMPINTEN1W::OUTSIDE_THRESHOLD)
    }
    #[doc = "Crossing threshold."]
    #[inline(always)]
    pub fn crossing_threshold(self) -> &'a mut W {
        self.variant(ADCMPINTEN1W::CROSSING_THRESHOLD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
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
impl crate::ToBits<u8> for ADCMPINTEN2R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            ADCMPINTEN2R::DISABLED => 0,
            ADCMPINTEN2R::OUTSIDE_THRESHOLD => 1,
            ADCMPINTEN2R::CROSSING_THRESHOLD => 2,
        }
    }
}
#[doc = r"Reader of the field"]
pub type ADCMPINTEN2_R = crate::FR<u8, ADCMPINTEN2R>;
impl ADCMPINTEN2_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADCMPINTEN2R::DISABLED
    }
    #[doc = "Checks if the value of the field is `OUTSIDE_THRESHOLD`"]
    #[inline(always)]
    pub fn is_outside_threshold(&self) -> bool {
        *self == ADCMPINTEN2R::OUTSIDE_THRESHOLD
    }
    #[doc = "Checks if the value of the field is `CROSSING_THRESHOLD`"]
    #[inline(always)]
    pub fn is_crossing_threshold(&self) -> bool {
        *self == ADCMPINTEN2R::CROSSING_THRESHOLD
    }
}
#[doc = "Values that can be written to the field `ADCMPINTEN2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADCMPINTEN2W::DISABLED => 0,
            ADCMPINTEN2W::OUTSIDE_THRESHOLD => 1,
            ADCMPINTEN2W::CROSSING_THRESHOLD => 2,
        }
    }
}
#[doc = r"Proxy"]
pub struct _ADCMPINTEN2W<'a> {
    w: &'a mut W,
}
impl<'a> _ADCMPINTEN2W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCMPINTEN2W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADCMPINTEN2W::DISABLED)
    }
    #[doc = "Outside threshold."]
    #[inline(always)]
    pub fn outside_threshold(self) -> &'a mut W {
        self.variant(ADCMPINTEN2W::OUTSIDE_THRESHOLD)
    }
    #[doc = "Crossing threshold."]
    #[inline(always)]
    pub fn crossing_threshold(self) -> &'a mut W {
        self.variant(ADCMPINTEN2W::CROSSING_THRESHOLD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 7)) | (((value as u32) & 0x03) << 7);
        self.w
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
impl crate::ToBits<u8> for ADCMPINTEN3R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            ADCMPINTEN3R::DISABLED => 0,
            ADCMPINTEN3R::OUTSIDE_THRESHOLD => 1,
            ADCMPINTEN3R::CROSSING_THRESHOLD => 2,
        }
    }
}
#[doc = r"Reader of the field"]
pub type ADCMPINTEN3_R = crate::FR<u8, ADCMPINTEN3R>;
impl ADCMPINTEN3_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADCMPINTEN3R::DISABLED
    }
    #[doc = "Checks if the value of the field is `OUTSIDE_THRESHOLD`"]
    #[inline(always)]
    pub fn is_outside_threshold(&self) -> bool {
        *self == ADCMPINTEN3R::OUTSIDE_THRESHOLD
    }
    #[doc = "Checks if the value of the field is `CROSSING_THRESHOLD`"]
    #[inline(always)]
    pub fn is_crossing_threshold(&self) -> bool {
        *self == ADCMPINTEN3R::CROSSING_THRESHOLD
    }
}
#[doc = "Values that can be written to the field `ADCMPINTEN3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADCMPINTEN3W::DISABLED => 0,
            ADCMPINTEN3W::OUTSIDE_THRESHOLD => 1,
            ADCMPINTEN3W::CROSSING_THRESHOLD => 2,
        }
    }
}
#[doc = r"Proxy"]
pub struct _ADCMPINTEN3W<'a> {
    w: &'a mut W,
}
impl<'a> _ADCMPINTEN3W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCMPINTEN3W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADCMPINTEN3W::DISABLED)
    }
    #[doc = "Outside threshold."]
    #[inline(always)]
    pub fn outside_threshold(self) -> &'a mut W {
        self.variant(ADCMPINTEN3W::OUTSIDE_THRESHOLD)
    }
    #[doc = "Crossing threshold."]
    #[inline(always)]
    pub fn crossing_threshold(self) -> &'a mut W {
        self.variant(ADCMPINTEN3W::CROSSING_THRESHOLD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | (((value as u32) & 0x03) << 9);
        self.w
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
impl crate::ToBits<u8> for ADCMPINTEN4R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            ADCMPINTEN4R::DISABLED => 0,
            ADCMPINTEN4R::OUTSIDE_THRESHOLD => 1,
            ADCMPINTEN4R::CROSSING_THRESHOLD => 2,
        }
    }
}
#[doc = r"Reader of the field"]
pub type ADCMPINTEN4_R = crate::FR<u8, ADCMPINTEN4R>;
impl ADCMPINTEN4_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADCMPINTEN4R::DISABLED
    }
    #[doc = "Checks if the value of the field is `OUTSIDE_THRESHOLD`"]
    #[inline(always)]
    pub fn is_outside_threshold(&self) -> bool {
        *self == ADCMPINTEN4R::OUTSIDE_THRESHOLD
    }
    #[doc = "Checks if the value of the field is `CROSSING_THRESHOLD`"]
    #[inline(always)]
    pub fn is_crossing_threshold(&self) -> bool {
        *self == ADCMPINTEN4R::CROSSING_THRESHOLD
    }
}
#[doc = "Values that can be written to the field `ADCMPINTEN4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADCMPINTEN4W::DISABLED => 0,
            ADCMPINTEN4W::OUTSIDE_THRESHOLD => 1,
            ADCMPINTEN4W::CROSSING_THRESHOLD => 2,
        }
    }
}
#[doc = r"Proxy"]
pub struct _ADCMPINTEN4W<'a> {
    w: &'a mut W,
}
impl<'a> _ADCMPINTEN4W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCMPINTEN4W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADCMPINTEN4W::DISABLED)
    }
    #[doc = "Outside threshold."]
    #[inline(always)]
    pub fn outside_threshold(self) -> &'a mut W {
        self.variant(ADCMPINTEN4W::OUTSIDE_THRESHOLD)
    }
    #[doc = "Crossing threshold."]
    #[inline(always)]
    pub fn crossing_threshold(self) -> &'a mut W {
        self.variant(ADCMPINTEN4W::CROSSING_THRESHOLD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | (((value as u32) & 0x03) << 11);
        self.w
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
impl crate::ToBits<u8> for ADCMPINTEN5R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            ADCMPINTEN5R::DISABLED => 0,
            ADCMPINTEN5R::OUTSIDE_THRESHOLD => 1,
            ADCMPINTEN5R::CROSSING_THRESHOLD => 2,
        }
    }
}
#[doc = r"Reader of the field"]
pub type ADCMPINTEN5_R = crate::FR<u8, ADCMPINTEN5R>;
impl ADCMPINTEN5_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADCMPINTEN5R::DISABLED
    }
    #[doc = "Checks if the value of the field is `OUTSIDE_THRESHOLD`"]
    #[inline(always)]
    pub fn is_outside_threshold(&self) -> bool {
        *self == ADCMPINTEN5R::OUTSIDE_THRESHOLD
    }
    #[doc = "Checks if the value of the field is `CROSSING_THRESHOLD`"]
    #[inline(always)]
    pub fn is_crossing_threshold(&self) -> bool {
        *self == ADCMPINTEN5R::CROSSING_THRESHOLD
    }
}
#[doc = "Values that can be written to the field `ADCMPINTEN5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADCMPINTEN5W::DISABLED => 0,
            ADCMPINTEN5W::OUTSIDE_THRESHOLD => 1,
            ADCMPINTEN5W::CROSSING_THRESHOLD => 2,
        }
    }
}
#[doc = r"Proxy"]
pub struct _ADCMPINTEN5W<'a> {
    w: &'a mut W,
}
impl<'a> _ADCMPINTEN5W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCMPINTEN5W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADCMPINTEN5W::DISABLED)
    }
    #[doc = "Outside threshold."]
    #[inline(always)]
    pub fn outside_threshold(self) -> &'a mut W {
        self.variant(ADCMPINTEN5W::OUTSIDE_THRESHOLD)
    }
    #[doc = "Crossing threshold."]
    #[inline(always)]
    pub fn crossing_threshold(self) -> &'a mut W {
        self.variant(ADCMPINTEN5W::CROSSING_THRESHOLD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | (((value as u32) & 0x03) << 13);
        self.w
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
impl crate::ToBits<u8> for ADCMPINTEN6R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            ADCMPINTEN6R::DISABLED => 0,
            ADCMPINTEN6R::OUTSIDE_THRESHOLD => 1,
            ADCMPINTEN6R::CROSSING_THRESHOLD => 2,
        }
    }
}
#[doc = r"Reader of the field"]
pub type ADCMPINTEN6_R = crate::FR<u8, ADCMPINTEN6R>;
impl ADCMPINTEN6_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADCMPINTEN6R::DISABLED
    }
    #[doc = "Checks if the value of the field is `OUTSIDE_THRESHOLD`"]
    #[inline(always)]
    pub fn is_outside_threshold(&self) -> bool {
        *self == ADCMPINTEN6R::OUTSIDE_THRESHOLD
    }
    #[doc = "Checks if the value of the field is `CROSSING_THRESHOLD`"]
    #[inline(always)]
    pub fn is_crossing_threshold(&self) -> bool {
        *self == ADCMPINTEN6R::CROSSING_THRESHOLD
    }
}
#[doc = "Values that can be written to the field `ADCMPINTEN6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADCMPINTEN6W::DISABLED => 0,
            ADCMPINTEN6W::OUTSIDE_THRESHOLD => 1,
            ADCMPINTEN6W::CROSSING_THRESHOLD => 2,
        }
    }
}
#[doc = r"Proxy"]
pub struct _ADCMPINTEN6W<'a> {
    w: &'a mut W,
}
impl<'a> _ADCMPINTEN6W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCMPINTEN6W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADCMPINTEN6W::DISABLED)
    }
    #[doc = "Outside threshold."]
    #[inline(always)]
    pub fn outside_threshold(self) -> &'a mut W {
        self.variant(ADCMPINTEN6W::OUTSIDE_THRESHOLD)
    }
    #[doc = "Crossing threshold."]
    #[inline(always)]
    pub fn crossing_threshold(self) -> &'a mut W {
        self.variant(ADCMPINTEN6W::CROSSING_THRESHOLD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 15)) | (((value as u32) & 0x03) << 15);
        self.w
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
impl crate::ToBits<u8> for ADCMPINTEN7R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            ADCMPINTEN7R::DISABLED => 0,
            ADCMPINTEN7R::OUTSIDE_THRESHOLD => 1,
            ADCMPINTEN7R::CROSSING_THRESHOLD => 2,
        }
    }
}
#[doc = r"Reader of the field"]
pub type ADCMPINTEN7_R = crate::FR<u8, ADCMPINTEN7R>;
impl ADCMPINTEN7_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADCMPINTEN7R::DISABLED
    }
    #[doc = "Checks if the value of the field is `OUTSIDE_THRESHOLD`"]
    #[inline(always)]
    pub fn is_outside_threshold(&self) -> bool {
        *self == ADCMPINTEN7R::OUTSIDE_THRESHOLD
    }
    #[doc = "Checks if the value of the field is `CROSSING_THRESHOLD`"]
    #[inline(always)]
    pub fn is_crossing_threshold(&self) -> bool {
        *self == ADCMPINTEN7R::CROSSING_THRESHOLD
    }
}
#[doc = "Values that can be written to the field `ADCMPINTEN7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADCMPINTEN7W::DISABLED => 0,
            ADCMPINTEN7W::OUTSIDE_THRESHOLD => 1,
            ADCMPINTEN7W::CROSSING_THRESHOLD => 2,
        }
    }
}
#[doc = r"Proxy"]
pub struct _ADCMPINTEN7W<'a> {
    w: &'a mut W,
}
impl<'a> _ADCMPINTEN7W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCMPINTEN7W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADCMPINTEN7W::DISABLED)
    }
    #[doc = "Outside threshold."]
    #[inline(always)]
    pub fn outside_threshold(self) -> &'a mut W {
        self.variant(ADCMPINTEN7W::OUTSIDE_THRESHOLD)
    }
    #[doc = "Crossing threshold."]
    #[inline(always)]
    pub fn crossing_threshold(self) -> &'a mut W {
        self.variant(ADCMPINTEN7W::CROSSING_THRESHOLD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 17)) | (((value as u32) & 0x03) << 17);
        self.w
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
impl crate::ToBits<u8> for ADCMPINTEN8R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            ADCMPINTEN8R::DISABLED => 0,
            ADCMPINTEN8R::OUTSIDE_THRESHOLD => 1,
            ADCMPINTEN8R::CROSSING_THRESHOLD => 2,
        }
    }
}
#[doc = r"Reader of the field"]
pub type ADCMPINTEN8_R = crate::FR<u8, ADCMPINTEN8R>;
impl ADCMPINTEN8_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADCMPINTEN8R::DISABLED
    }
    #[doc = "Checks if the value of the field is `OUTSIDE_THRESHOLD`"]
    #[inline(always)]
    pub fn is_outside_threshold(&self) -> bool {
        *self == ADCMPINTEN8R::OUTSIDE_THRESHOLD
    }
    #[doc = "Checks if the value of the field is `CROSSING_THRESHOLD`"]
    #[inline(always)]
    pub fn is_crossing_threshold(&self) -> bool {
        *self == ADCMPINTEN8R::CROSSING_THRESHOLD
    }
}
#[doc = "Values that can be written to the field `ADCMPINTEN8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADCMPINTEN8W::DISABLED => 0,
            ADCMPINTEN8W::OUTSIDE_THRESHOLD => 1,
            ADCMPINTEN8W::CROSSING_THRESHOLD => 2,
        }
    }
}
#[doc = r"Proxy"]
pub struct _ADCMPINTEN8W<'a> {
    w: &'a mut W,
}
impl<'a> _ADCMPINTEN8W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCMPINTEN8W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADCMPINTEN8W::DISABLED)
    }
    #[doc = "Outside threshold."]
    #[inline(always)]
    pub fn outside_threshold(self) -> &'a mut W {
        self.variant(ADCMPINTEN8W::OUTSIDE_THRESHOLD)
    }
    #[doc = "Crossing threshold."]
    #[inline(always)]
    pub fn crossing_threshold(self) -> &'a mut W {
        self.variant(ADCMPINTEN8W::CROSSING_THRESHOLD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 19)) | (((value as u32) & 0x03) << 19);
        self.w
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
impl crate::ToBits<u8> for ADCMPINTEN9R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            ADCMPINTEN9R::DISABLED => 0,
            ADCMPINTEN9R::OUTSIDE_THRESHOLD => 1,
            ADCMPINTEN9R::CROSSING_THRESHOLD => 2,
        }
    }
}
#[doc = r"Reader of the field"]
pub type ADCMPINTEN9_R = crate::FR<u8, ADCMPINTEN9R>;
impl ADCMPINTEN9_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADCMPINTEN9R::DISABLED
    }
    #[doc = "Checks if the value of the field is `OUTSIDE_THRESHOLD`"]
    #[inline(always)]
    pub fn is_outside_threshold(&self) -> bool {
        *self == ADCMPINTEN9R::OUTSIDE_THRESHOLD
    }
    #[doc = "Checks if the value of the field is `CROSSING_THRESHOLD`"]
    #[inline(always)]
    pub fn is_crossing_threshold(&self) -> bool {
        *self == ADCMPINTEN9R::CROSSING_THRESHOLD
    }
}
#[doc = "Values that can be written to the field `ADCMPINTEN9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADCMPINTEN9W::DISABLED => 0,
            ADCMPINTEN9W::OUTSIDE_THRESHOLD => 1,
            ADCMPINTEN9W::CROSSING_THRESHOLD => 2,
        }
    }
}
#[doc = r"Proxy"]
pub struct _ADCMPINTEN9W<'a> {
    w: &'a mut W,
}
impl<'a> _ADCMPINTEN9W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCMPINTEN9W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADCMPINTEN9W::DISABLED)
    }
    #[doc = "Outside threshold."]
    #[inline(always)]
    pub fn outside_threshold(self) -> &'a mut W {
        self.variant(ADCMPINTEN9W::OUTSIDE_THRESHOLD)
    }
    #[doc = "Crossing threshold."]
    #[inline(always)]
    pub fn crossing_threshold(self) -> &'a mut W {
        self.variant(ADCMPINTEN9W::CROSSING_THRESHOLD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | (((value as u32) & 0x03) << 21);
        self.w
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
impl crate::ToBits<u8> for ADCMPINTEN10R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            ADCMPINTEN10R::DISABLED => 0,
            ADCMPINTEN10R::OUTSIDE_THRESHOLD => 1,
            ADCMPINTEN10R::CROSSING_THRESHOLD => 2,
        }
    }
}
#[doc = r"Reader of the field"]
pub type ADCMPINTEN10_R = crate::FR<u8, ADCMPINTEN10R>;
impl ADCMPINTEN10_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADCMPINTEN10R::DISABLED
    }
    #[doc = "Checks if the value of the field is `OUTSIDE_THRESHOLD`"]
    #[inline(always)]
    pub fn is_outside_threshold(&self) -> bool {
        *self == ADCMPINTEN10R::OUTSIDE_THRESHOLD
    }
    #[doc = "Checks if the value of the field is `CROSSING_THRESHOLD`"]
    #[inline(always)]
    pub fn is_crossing_threshold(&self) -> bool {
        *self == ADCMPINTEN10R::CROSSING_THRESHOLD
    }
}
#[doc = "Values that can be written to the field `ADCMPINTEN10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADCMPINTEN10W::DISABLED => 0,
            ADCMPINTEN10W::OUTSIDE_THRESHOLD => 1,
            ADCMPINTEN10W::CROSSING_THRESHOLD => 2,
        }
    }
}
#[doc = r"Proxy"]
pub struct _ADCMPINTEN10W<'a> {
    w: &'a mut W,
}
impl<'a> _ADCMPINTEN10W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCMPINTEN10W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADCMPINTEN10W::DISABLED)
    }
    #[doc = "Outside threshold."]
    #[inline(always)]
    pub fn outside_threshold(self) -> &'a mut W {
        self.variant(ADCMPINTEN10W::OUTSIDE_THRESHOLD)
    }
    #[doc = "Crossing threshold."]
    #[inline(always)]
    pub fn crossing_threshold(self) -> &'a mut W {
        self.variant(ADCMPINTEN10W::CROSSING_THRESHOLD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 23)) | (((value as u32) & 0x03) << 23);
        self.w
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
impl crate::ToBits<u8> for ADCMPINTEN11R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            ADCMPINTEN11R::DISABLED => 0,
            ADCMPINTEN11R::OUTSIDE_THRESHOLD => 1,
            ADCMPINTEN11R::CROSSING_THRESHOLD => 2,
        }
    }
}
#[doc = r"Reader of the field"]
pub type ADCMPINTEN11_R = crate::FR<u8, ADCMPINTEN11R>;
impl ADCMPINTEN11_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADCMPINTEN11R::DISABLED
    }
    #[doc = "Checks if the value of the field is `OUTSIDE_THRESHOLD`"]
    #[inline(always)]
    pub fn is_outside_threshold(&self) -> bool {
        *self == ADCMPINTEN11R::OUTSIDE_THRESHOLD
    }
    #[doc = "Checks if the value of the field is `CROSSING_THRESHOLD`"]
    #[inline(always)]
    pub fn is_crossing_threshold(&self) -> bool {
        *self == ADCMPINTEN11R::CROSSING_THRESHOLD
    }
}
#[doc = "Values that can be written to the field `ADCMPINTEN11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADCMPINTEN11W::DISABLED => 0,
            ADCMPINTEN11W::OUTSIDE_THRESHOLD => 1,
            ADCMPINTEN11W::CROSSING_THRESHOLD => 2,
        }
    }
}
#[doc = r"Proxy"]
pub struct _ADCMPINTEN11W<'a> {
    w: &'a mut W,
}
impl<'a> _ADCMPINTEN11W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCMPINTEN11W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADCMPINTEN11W::DISABLED)
    }
    #[doc = "Outside threshold."]
    #[inline(always)]
    pub fn outside_threshold(self) -> &'a mut W {
        self.variant(ADCMPINTEN11W::OUTSIDE_THRESHOLD)
    }
    #[doc = "Crossing threshold."]
    #[inline(always)]
    pub fn crossing_threshold(self) -> &'a mut W {
        self.variant(ADCMPINTEN11W::CROSSING_THRESHOLD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 25)) | (((value as u32) & 0x03) << 25);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Sequence A interrupt enable."]
    #[inline(always)]
    pub fn seqa_inten(&self) -> SEQA_INTEN_R {
        SEQA_INTEN_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Sequence B interrupt enable."]
    #[inline(always)]
    pub fn seqb_inten(&self) -> SEQB_INTEN_R {
        SEQB_INTEN_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Overrun interrupt enable."]
    #[inline(always)]
    pub fn ovr_inten(&self) -> OVR_INTEN_R {
        OVR_INTEN_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:4 - Threshold comparison interrupt enable."]
    #[inline(always)]
    pub fn adcmpinten0(&self) -> ADCMPINTEN0_R {
        ADCMPINTEN0_R::new(((self.bits() >> 3) & 0x03) as u8)
    }
    #[doc = "Bits 5:6 - Threshold comparison interrupt enable."]
    #[inline(always)]
    pub fn adcmpinten1(&self) -> ADCMPINTEN1_R {
        ADCMPINTEN1_R::new(((self.bits() >> 5) & 0x03) as u8)
    }
    #[doc = "Bits 7:8 - Threshold comparison interrupt enable."]
    #[inline(always)]
    pub fn adcmpinten2(&self) -> ADCMPINTEN2_R {
        ADCMPINTEN2_R::new(((self.bits() >> 7) & 0x03) as u8)
    }
    #[doc = "Bits 9:10 - Threshold comparison interrupt enable."]
    #[inline(always)]
    pub fn adcmpinten3(&self) -> ADCMPINTEN3_R {
        ADCMPINTEN3_R::new(((self.bits() >> 9) & 0x03) as u8)
    }
    #[doc = "Bits 11:12 - Threshold comparison interrupt enable."]
    #[inline(always)]
    pub fn adcmpinten4(&self) -> ADCMPINTEN4_R {
        ADCMPINTEN4_R::new(((self.bits() >> 11) & 0x03) as u8)
    }
    #[doc = "Bits 13:14 - Threshold comparison interrupt enable."]
    #[inline(always)]
    pub fn adcmpinten5(&self) -> ADCMPINTEN5_R {
        ADCMPINTEN5_R::new(((self.bits() >> 13) & 0x03) as u8)
    }
    #[doc = "Bits 15:16 - Threshold comparison interrupt enable."]
    #[inline(always)]
    pub fn adcmpinten6(&self) -> ADCMPINTEN6_R {
        ADCMPINTEN6_R::new(((self.bits() >> 15) & 0x03) as u8)
    }
    #[doc = "Bits 17:18 - Threshold comparison interrupt enable."]
    #[inline(always)]
    pub fn adcmpinten7(&self) -> ADCMPINTEN7_R {
        ADCMPINTEN7_R::new(((self.bits() >> 17) & 0x03) as u8)
    }
    #[doc = "Bits 19:20 - Threshold comparison interrupt enable."]
    #[inline(always)]
    pub fn adcmpinten8(&self) -> ADCMPINTEN8_R {
        ADCMPINTEN8_R::new(((self.bits() >> 19) & 0x03) as u8)
    }
    #[doc = "Bits 21:22 - Threshold comparison interrupt enable."]
    #[inline(always)]
    pub fn adcmpinten9(&self) -> ADCMPINTEN9_R {
        ADCMPINTEN9_R::new(((self.bits() >> 21) & 0x03) as u8)
    }
    #[doc = "Bits 23:24 - Threshold comparison interrupt enable."]
    #[inline(always)]
    pub fn adcmpinten10(&self) -> ADCMPINTEN10_R {
        ADCMPINTEN10_R::new(((self.bits() >> 23) & 0x03) as u8)
    }
    #[doc = "Bits 25:26 - Threshold comparison interrupt enable."]
    #[inline(always)]
    pub fn adcmpinten11(&self) -> ADCMPINTEN11_R {
        ADCMPINTEN11_R::new(((self.bits() >> 25) & 0x03) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Sequence A interrupt enable."]
    #[inline(always)]
    pub fn seqa_inten(&mut self) -> _SEQA_INTENW {
        _SEQA_INTENW { w: self }
    }
    #[doc = "Bit 1 - Sequence B interrupt enable."]
    #[inline(always)]
    pub fn seqb_inten(&mut self) -> _SEQB_INTENW {
        _SEQB_INTENW { w: self }
    }
    #[doc = "Bit 2 - Overrun interrupt enable."]
    #[inline(always)]
    pub fn ovr_inten(&mut self) -> _OVR_INTENW {
        _OVR_INTENW { w: self }
    }
    #[doc = "Bits 3:4 - Threshold comparison interrupt enable."]
    #[inline(always)]
    pub fn adcmpinten0(&mut self) -> _ADCMPINTEN0W {
        _ADCMPINTEN0W { w: self }
    }
    #[doc = "Bits 5:6 - Threshold comparison interrupt enable."]
    #[inline(always)]
    pub fn adcmpinten1(&mut self) -> _ADCMPINTEN1W {
        _ADCMPINTEN1W { w: self }
    }
    #[doc = "Bits 7:8 - Threshold comparison interrupt enable."]
    #[inline(always)]
    pub fn adcmpinten2(&mut self) -> _ADCMPINTEN2W {
        _ADCMPINTEN2W { w: self }
    }
    #[doc = "Bits 9:10 - Threshold comparison interrupt enable."]
    #[inline(always)]
    pub fn adcmpinten3(&mut self) -> _ADCMPINTEN3W {
        _ADCMPINTEN3W { w: self }
    }
    #[doc = "Bits 11:12 - Threshold comparison interrupt enable."]
    #[inline(always)]
    pub fn adcmpinten4(&mut self) -> _ADCMPINTEN4W {
        _ADCMPINTEN4W { w: self }
    }
    #[doc = "Bits 13:14 - Threshold comparison interrupt enable."]
    #[inline(always)]
    pub fn adcmpinten5(&mut self) -> _ADCMPINTEN5W {
        _ADCMPINTEN5W { w: self }
    }
    #[doc = "Bits 15:16 - Threshold comparison interrupt enable."]
    #[inline(always)]
    pub fn adcmpinten6(&mut self) -> _ADCMPINTEN6W {
        _ADCMPINTEN6W { w: self }
    }
    #[doc = "Bits 17:18 - Threshold comparison interrupt enable."]
    #[inline(always)]
    pub fn adcmpinten7(&mut self) -> _ADCMPINTEN7W {
        _ADCMPINTEN7W { w: self }
    }
    #[doc = "Bits 19:20 - Threshold comparison interrupt enable."]
    #[inline(always)]
    pub fn adcmpinten8(&mut self) -> _ADCMPINTEN8W {
        _ADCMPINTEN8W { w: self }
    }
    #[doc = "Bits 21:22 - Threshold comparison interrupt enable."]
    #[inline(always)]
    pub fn adcmpinten9(&mut self) -> _ADCMPINTEN9W {
        _ADCMPINTEN9W { w: self }
    }
    #[doc = "Bits 23:24 - Threshold comparison interrupt enable."]
    #[inline(always)]
    pub fn adcmpinten10(&mut self) -> _ADCMPINTEN10W {
        _ADCMPINTEN10W { w: self }
    }
    #[doc = "Bits 25:26 - Threshold comparison interrupt enable."]
    #[inline(always)]
    pub fn adcmpinten11(&mut self) -> _ADCMPINTEN11W {
        _ADCMPINTEN11W { w: self }
    }
}
