#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HDEN {
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
#[doc = "Possible values of the field `HDEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HDENR {
    #[doc = "Disable half-duplex mode."]
    DISABLE_HALF_DUPLEX_,
    #[doc = "Enable half-duplex mode."]
    ENABLE_HALF_DUPLEX_M,
}
impl crate::ToBits<bool> for HDENR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            HDENR::DISABLE_HALF_DUPLEX_ => false,
            HDENR::ENABLE_HALF_DUPLEX_M => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type HDEN_R = crate::FR<bool, HDENR>;
impl HDEN_R {
    #[doc = "Checks if the value of the field is `DISABLE_HALF_DUPLEX_`"]
    #[inline(always)]
    pub fn is_disable_half_duplex_(&self) -> bool {
        *self == HDENR::DISABLE_HALF_DUPLEX_
    }
    #[doc = "Checks if the value of the field is `ENABLE_HALF_DUPLEX_M`"]
    #[inline(always)]
    pub fn is_enable_half_duplex_m(&self) -> bool {
        *self == HDENR::ENABLE_HALF_DUPLEX_M
    }
}
#[doc = "Values that can be written to the field `HDEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HDENW {
    #[doc = "Disable half-duplex mode."]
    DISABLE_HALF_DUPLEX_,
    #[doc = "Enable half-duplex mode."]
    ENABLE_HALF_DUPLEX_M,
}
impl HDENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            HDENW::DISABLE_HALF_DUPLEX_ => false,
            HDENW::ENABLE_HALF_DUPLEX_M => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _HDENW<'a> {
    w: &'a mut W,
}
impl<'a> _HDENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HDENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable half-duplex mode."]
    #[inline(always)]
    pub fn disable_half_duplex_(self) -> &'a mut W {
        self.variant(HDENW::DISABLE_HALF_DUPLEX_)
    }
    #[doc = "Enable half-duplex mode."]
    #[inline(always)]
    pub fn enable_half_duplex_m(self) -> &'a mut W {
        self.variant(HDENW::ENABLE_HALF_DUPLEX_M)
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Half-duplex mode enable"]
    #[inline(always)]
    pub fn hden(&self) -> HDEN_R {
        HDEN_R::new((self.bits() & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Half-duplex mode enable"]
    #[inline(always)]
    pub fn hden(&mut self) -> _HDENW {
        _HDENW { w: self }
    }
}
