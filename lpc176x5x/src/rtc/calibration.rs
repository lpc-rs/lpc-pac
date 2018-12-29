#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CALIBRATION {
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
#[doc = r" Value of the field"]
pub struct CALVALR {
    bits: u32,
}
impl CALVALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = "Possible values of the field `CALDIR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CALDIRR {
    #[doc = "Backward calibration. When CALVAL is equal to the calibration counter, the RTC timers will stop incrementing for 1 second."]
    BACKWARD_CALIBRATION,
    #[doc = "Forward calibration. When CALVAL is equal to the calibration counter, the RTC timers will jump by 2 seconds."]
    FORWARD_CALIBRATION_,
}
impl CALDIRR {
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
            CALDIRR::BACKWARD_CALIBRATION => true,
            CALDIRR::FORWARD_CALIBRATION_ => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CALDIRR {
        match value {
            true => CALDIRR::BACKWARD_CALIBRATION,
            false => CALDIRR::FORWARD_CALIBRATION_,
        }
    }
    #[doc = "Checks if the value of the field is `BACKWARD_CALIBRATION`"]
    #[inline]
    pub fn is_backward_calibration(&self) -> bool {
        *self == CALDIRR::BACKWARD_CALIBRATION
    }
    #[doc = "Checks if the value of the field is `FORWARD_CALIBRATION_`"]
    #[inline]
    pub fn is_forward_calibration_(&self) -> bool {
        *self == CALDIRR::FORWARD_CALIBRATION_
    }
}
#[doc = r" Proxy"]
pub struct _CALVALW<'a> {
    w: &'a mut W,
}
impl<'a> _CALVALW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 131071;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CALDIR`"]
pub enum CALDIRW {
    #[doc = "Backward calibration. When CALVAL is equal to the calibration counter, the RTC timers will stop incrementing for 1 second."]
    BACKWARD_CALIBRATION,
    #[doc = "Forward calibration. When CALVAL is equal to the calibration counter, the RTC timers will jump by 2 seconds."]
    FORWARD_CALIBRATION_,
}
impl CALDIRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CALDIRW::BACKWARD_CALIBRATION => true,
            CALDIRW::FORWARD_CALIBRATION_ => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CALDIRW<'a> {
    w: &'a mut W,
}
impl<'a> _CALDIRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CALDIRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Backward calibration. When CALVAL is equal to the calibration counter, the RTC timers will stop incrementing for 1 second."]
    #[inline]
    pub fn backward_calibration(self) -> &'a mut W {
        self.variant(CALDIRW::BACKWARD_CALIBRATION)
    }
    #[doc = "Forward calibration. When CALVAL is equal to the calibration counter, the RTC timers will jump by 2 seconds."]
    #[inline]
    pub fn forward_calibration_(self) -> &'a mut W {
        self.variant(CALDIRW::FORWARD_CALIBRATION_)
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
        const OFFSET: u8 = 17;
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
    #[doc = "Bits 0:16 - If enabled, the calibration counter counts up to this value. The maximum value is 131, 072 corresponding to about 36.4 hours. Calibration is disabled if CALVAL = 0."]
    #[inline]
    pub fn calval(&self) -> CALVALR {
        let bits = {
            const MASK: u32 = 131071;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        CALVALR { bits }
    }
    #[doc = "Bit 17 - Calibration direction"]
    #[inline]
    pub fn caldir(&self) -> CALDIRR {
        CALDIRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
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
    #[doc = "Bits 0:16 - If enabled, the calibration counter counts up to this value. The maximum value is 131, 072 corresponding to about 36.4 hours. Calibration is disabled if CALVAL = 0."]
    #[inline]
    pub fn calval(&mut self) -> _CALVALW {
        _CALVALW { w: self }
    }
    #[doc = "Bit 17 - Calibration direction"]
    #[inline]
    pub fn caldir(&mut self) -> _CALDIRW {
        _CALDIRW { w: self }
    }
}
