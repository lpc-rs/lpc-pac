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
    #[doc = "Enabled. The overrun interrupt is enabled. Detection of an overrun condition on any of the 12 channel data registers will cause an overrun interrupt/DMA trigger. In addition, if the MODE bit for a particular sequence is 0, then an overrun in the global data register for that sequence will also cause this interrupt/DMA trigger to be asserted."]
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
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ADCMPINTEN0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ADCMPINTEN0R::DISABLED => 0,
            ADCMPINTEN0R::OUTSIDE_THRESHOLD => 1,
            ADCMPINTEN0R::CROSSING_THRESHOLD => 2,
            ADCMPINTEN0R::_Reserved(bits) => bits,
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
            i => ADCMPINTEN0R::_Reserved(i),
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
#[doc = r" Value of the field"]
pub struct ADCMPINTEN1R {
    bits: u8,
}
impl ADCMPINTEN1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ADCMPINTEN2R {
    bits: u8,
}
impl ADCMPINTEN2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ADCMPINTEN3R {
    bits: u8,
}
impl ADCMPINTEN3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ADCMPINTEN4R {
    bits: u8,
}
impl ADCMPINTEN4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ADCMPINTEN5R {
    bits: u8,
}
impl ADCMPINTEN5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ADCMPINTEN6R {
    bits: u8,
}
impl ADCMPINTEN6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ADCMPINTEN7R {
    bits: u8,
}
impl ADCMPINTEN7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ADCMPINTEN8R {
    bits: u8,
}
impl ADCMPINTEN8R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ADCMPINTEN9R {
    bits: u8,
}
impl ADCMPINTEN9R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ADCMPINTEN10R {
    bits: u8,
}
impl ADCMPINTEN10R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ADCMPINTEN11R {
    bits: u8,
}
impl ADCMPINTEN11R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
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
    #[doc = "Enabled. The overrun interrupt is enabled. Detection of an overrun condition on any of the 12 channel data registers will cause an overrun interrupt/DMA trigger. In addition, if the MODE bit for a particular sequence is 0, then an overrun in the global data register for that sequence will also cause this interrupt/DMA trigger to be asserted."]
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
    #[doc = "Enabled. The overrun interrupt is enabled. Detection of an overrun condition on any of the 12 channel data registers will cause an overrun interrupt/DMA trigger. In addition, if the MODE bit for a particular sequence is 0, then an overrun in the global data register for that sequence will also cause this interrupt/DMA trigger to be asserted."]
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
#[doc = r" Proxy"]
pub struct _ADCMPINTEN1W<'a> {
    w: &'a mut W,
}
impl<'a> _ADCMPINTEN1W<'a> {
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
#[doc = r" Proxy"]
pub struct _ADCMPINTEN2W<'a> {
    w: &'a mut W,
}
impl<'a> _ADCMPINTEN2W<'a> {
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
#[doc = r" Proxy"]
pub struct _ADCMPINTEN3W<'a> {
    w: &'a mut W,
}
impl<'a> _ADCMPINTEN3W<'a> {
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
#[doc = r" Proxy"]
pub struct _ADCMPINTEN4W<'a> {
    w: &'a mut W,
}
impl<'a> _ADCMPINTEN4W<'a> {
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
#[doc = r" Proxy"]
pub struct _ADCMPINTEN5W<'a> {
    w: &'a mut W,
}
impl<'a> _ADCMPINTEN5W<'a> {
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
#[doc = r" Proxy"]
pub struct _ADCMPINTEN6W<'a> {
    w: &'a mut W,
}
impl<'a> _ADCMPINTEN6W<'a> {
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
#[doc = r" Proxy"]
pub struct _ADCMPINTEN7W<'a> {
    w: &'a mut W,
}
impl<'a> _ADCMPINTEN7W<'a> {
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
#[doc = r" Proxy"]
pub struct _ADCMPINTEN8W<'a> {
    w: &'a mut W,
}
impl<'a> _ADCMPINTEN8W<'a> {
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
#[doc = r" Proxy"]
pub struct _ADCMPINTEN9W<'a> {
    w: &'a mut W,
}
impl<'a> _ADCMPINTEN9W<'a> {
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
#[doc = r" Proxy"]
pub struct _ADCMPINTEN10W<'a> {
    w: &'a mut W,
}
impl<'a> _ADCMPINTEN10W<'a> {
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
#[doc = r" Proxy"]
pub struct _ADCMPINTEN11W<'a> {
    w: &'a mut W,
}
impl<'a> _ADCMPINTEN11W<'a> {
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
    #[doc = "Bits 3:4 - Threshold comparison interrupt enable for channel 0."]
    #[inline]
    pub fn adcmpinten0(&self) -> ADCMPINTEN0R {
        ADCMPINTEN0R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 5:6 - Channel 1 threshold comparison interrupt enable. See description for channel 0."]
    #[inline]
    pub fn adcmpinten1(&self) -> ADCMPINTEN1R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ADCMPINTEN1R { bits }
    }
    #[doc = "Bits 7:8 - Channel 2 threshold comparison interrupt enable. See description for channel 0."]
    #[inline]
    pub fn adcmpinten2(&self) -> ADCMPINTEN2R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ADCMPINTEN2R { bits }
    }
    #[doc = "Bits 9:10 - Channel 3 threshold comparison interrupt enable. See description for channel 0."]
    #[inline]
    pub fn adcmpinten3(&self) -> ADCMPINTEN3R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ADCMPINTEN3R { bits }
    }
    #[doc = "Bits 11:12 - Channel 4 threshold comparison interrupt enable. See description for channel 0."]
    #[inline]
    pub fn adcmpinten4(&self) -> ADCMPINTEN4R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ADCMPINTEN4R { bits }
    }
    #[doc = "Bits 13:14 - Channel 5 threshold comparison interrupt enable. See description for channel 0."]
    #[inline]
    pub fn adcmpinten5(&self) -> ADCMPINTEN5R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ADCMPINTEN5R { bits }
    }
    #[doc = "Bits 15:16 - Channel 6 threshold comparison interrupt enable. See description for channel 0."]
    #[inline]
    pub fn adcmpinten6(&self) -> ADCMPINTEN6R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ADCMPINTEN6R { bits }
    }
    #[doc = "Bits 17:18 - Channel 7 threshold comparison interrupt enable. See description for channel 0."]
    #[inline]
    pub fn adcmpinten7(&self) -> ADCMPINTEN7R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ADCMPINTEN7R { bits }
    }
    #[doc = "Bits 19:20 - Channel 8 threshold comparison interrupt enable. See description for channel 0."]
    #[inline]
    pub fn adcmpinten8(&self) -> ADCMPINTEN8R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ADCMPINTEN8R { bits }
    }
    #[doc = "Bits 21:22 - Channel 9 threshold comparison interrupt enable. See description for channel 0."]
    #[inline]
    pub fn adcmpinten9(&self) -> ADCMPINTEN9R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ADCMPINTEN9R { bits }
    }
    #[doc = "Bits 23:24 - Channel 10 threshold comparison interrupt enable. See description for channel 0."]
    #[inline]
    pub fn adcmpinten10(&self) -> ADCMPINTEN10R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ADCMPINTEN10R { bits }
    }
    #[doc = "Bits 25:26 - Channel 21 threshold comparison interrupt enable. See description for channel 0."]
    #[inline]
    pub fn adcmpinten11(&self) -> ADCMPINTEN11R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ADCMPINTEN11R { bits }
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
    #[doc = "Bits 3:4 - Threshold comparison interrupt enable for channel 0."]
    #[inline]
    pub fn adcmpinten0(&mut self) -> _ADCMPINTEN0W {
        _ADCMPINTEN0W { w: self }
    }
    #[doc = "Bits 5:6 - Channel 1 threshold comparison interrupt enable. See description for channel 0."]
    #[inline]
    pub fn adcmpinten1(&mut self) -> _ADCMPINTEN1W {
        _ADCMPINTEN1W { w: self }
    }
    #[doc = "Bits 7:8 - Channel 2 threshold comparison interrupt enable. See description for channel 0."]
    #[inline]
    pub fn adcmpinten2(&mut self) -> _ADCMPINTEN2W {
        _ADCMPINTEN2W { w: self }
    }
    #[doc = "Bits 9:10 - Channel 3 threshold comparison interrupt enable. See description for channel 0."]
    #[inline]
    pub fn adcmpinten3(&mut self) -> _ADCMPINTEN3W {
        _ADCMPINTEN3W { w: self }
    }
    #[doc = "Bits 11:12 - Channel 4 threshold comparison interrupt enable. See description for channel 0."]
    #[inline]
    pub fn adcmpinten4(&mut self) -> _ADCMPINTEN4W {
        _ADCMPINTEN4W { w: self }
    }
    #[doc = "Bits 13:14 - Channel 5 threshold comparison interrupt enable. See description for channel 0."]
    #[inline]
    pub fn adcmpinten5(&mut self) -> _ADCMPINTEN5W {
        _ADCMPINTEN5W { w: self }
    }
    #[doc = "Bits 15:16 - Channel 6 threshold comparison interrupt enable. See description for channel 0."]
    #[inline]
    pub fn adcmpinten6(&mut self) -> _ADCMPINTEN6W {
        _ADCMPINTEN6W { w: self }
    }
    #[doc = "Bits 17:18 - Channel 7 threshold comparison interrupt enable. See description for channel 0."]
    #[inline]
    pub fn adcmpinten7(&mut self) -> _ADCMPINTEN7W {
        _ADCMPINTEN7W { w: self }
    }
    #[doc = "Bits 19:20 - Channel 8 threshold comparison interrupt enable. See description for channel 0."]
    #[inline]
    pub fn adcmpinten8(&mut self) -> _ADCMPINTEN8W {
        _ADCMPINTEN8W { w: self }
    }
    #[doc = "Bits 21:22 - Channel 9 threshold comparison interrupt enable. See description for channel 0."]
    #[inline]
    pub fn adcmpinten9(&mut self) -> _ADCMPINTEN9W {
        _ADCMPINTEN9W { w: self }
    }
    #[doc = "Bits 23:24 - Channel 10 threshold comparison interrupt enable. See description for channel 0."]
    #[inline]
    pub fn adcmpinten10(&mut self) -> _ADCMPINTEN10W {
        _ADCMPINTEN10W { w: self }
    }
    #[doc = "Bits 25:26 - Channel 21 threshold comparison interrupt enable. See description for channel 0."]
    #[inline]
    pub fn adcmpinten11(&mut self) -> _ADCMPINTEN11W {
        _ADCMPINTEN11W { w: self }
    }
}
