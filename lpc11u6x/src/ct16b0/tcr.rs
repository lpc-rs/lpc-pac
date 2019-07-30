#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TCR {
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
#[doc = "Possible values of the field `CEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CENR {
    #[doc = "The counters are disabled."]
    DISABLED,
    #[doc = "The Timer Counter and Prescale Counter are enabled for counting."]
    ENABLED,
}
impl crate::ToBits<bool> for CENR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            CENR::DISABLED => false,
            CENR::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CEN_R = crate::FR<bool, CENR>;
impl CEN_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CENR::ENABLED
    }
}
#[doc = "Values that can be written to the field `CEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CENW {
    #[doc = "The counters are disabled."]
    DISABLED,
    #[doc = "The Timer Counter and Prescale Counter are enabled for counting."]
    ENABLED,
}
impl CENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            CENW::DISABLED => false,
            CENW::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CENW<'a> {
    w: &'a mut W,
}
impl<'a> _CENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The counters are disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CENW::DISABLED)
    }
    #[doc = "The Timer Counter and Prescale Counter are enabled for counting."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CENW::ENABLED)
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
#[doc = "Possible values of the field `CRST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRSTR {
    #[doc = "Do nothing."]
    NOP,
    #[doc = "The Timer Counter and the Prescale Counter are synchronously reset on the next positive edge of PCLK. The counters remain reset until TCR\\[1\\] is returned to zero."]
    RESET,
}
impl crate::ToBits<bool> for CRSTR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            CRSTR::NOP => false,
            CRSTR::RESET => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CRST_R = crate::FR<bool, CRSTR>;
impl CRST_R {
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == CRSTR::NOP
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == CRSTR::RESET
    }
}
#[doc = "Values that can be written to the field `CRST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRSTW {
    #[doc = "Do nothing."]
    NOP,
    #[doc = "The Timer Counter and the Prescale Counter are synchronously reset on the next positive edge of PCLK. The counters remain reset until TCR\\[1\\] is returned to zero."]
    RESET,
}
impl CRSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            CRSTW::NOP => false,
            CRSTW::RESET => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _CRSTW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Do nothing."]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(CRSTW::NOP)
    }
    #[doc = "The Timer Counter and the Prescale Counter are synchronously reset on the next positive edge of PCLK. The counters remain reset until TCR\\[1\\] is returned to zero."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(CRSTW::RESET)
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Counter enable."]
    #[inline(always)]
    pub fn cen(&self) -> CEN_R {
        CEN_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Counter reset."]
    #[inline(always)]
    pub fn crst(&self) -> CRST_R {
        CRST_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Counter enable."]
    #[inline(always)]
    pub fn cen(&mut self) -> _CENW {
        _CENW { w: self }
    }
    #[doc = "Bit 1 - Counter reset."]
    #[inline(always)]
    pub fn crst(&mut self) -> _CRSTW {
        _CRSTW { w: self }
    }
}
