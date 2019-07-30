#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::USBPLLCTRL {
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
#[doc = r"Reader of the field"]
pub type MSEL_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _MSELW<'a> {
    w: &'a mut W,
}
impl<'a> _MSELW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Possible values of the field `PSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSELR {
    #[doc = "P = 1"]
    P_EQ_1,
    #[doc = "P = 2"]
    P_EQ_2,
    #[doc = "P = 4"]
    P_EQ_4,
    #[doc = "P = 8"]
    P_EQ_8,
}
impl crate::ToBits<u8> for PSELR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            PSELR::P_EQ_1 => 0,
            PSELR::P_EQ_2 => 1,
            PSELR::P_EQ_4 => 2,
            PSELR::P_EQ_8 => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type PSEL_R = crate::FR<u8, PSELR>;
impl PSEL_R {
    #[doc = "Checks if the value of the field is `P_EQ_1`"]
    #[inline(always)]
    pub fn is_p_eq_1(&self) -> bool {
        *self == PSELR::P_EQ_1
    }
    #[doc = "Checks if the value of the field is `P_EQ_2`"]
    #[inline(always)]
    pub fn is_p_eq_2(&self) -> bool {
        *self == PSELR::P_EQ_2
    }
    #[doc = "Checks if the value of the field is `P_EQ_4`"]
    #[inline(always)]
    pub fn is_p_eq_4(&self) -> bool {
        *self == PSELR::P_EQ_4
    }
    #[doc = "Checks if the value of the field is `P_EQ_8`"]
    #[inline(always)]
    pub fn is_p_eq_8(&self) -> bool {
        *self == PSELR::P_EQ_8
    }
}
#[doc = "Values that can be written to the field `PSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSELW {
    #[doc = "P = 1"]
    P_EQ_1,
    #[doc = "P = 2"]
    P_EQ_2,
    #[doc = "P = 4"]
    P_EQ_4,
    #[doc = "P = 8"]
    P_EQ_8,
}
impl PSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            PSELW::P_EQ_1 => 0,
            PSELW::P_EQ_2 => 1,
            PSELW::P_EQ_4 => 2,
            PSELW::P_EQ_8 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PSELW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "P = 1"]
    #[inline(always)]
    pub fn p_eq_1(self) -> &'a mut W {
        self.variant(PSELW::P_EQ_1)
    }
    #[doc = "P = 2"]
    #[inline(always)]
    pub fn p_eq_2(self) -> &'a mut W {
        self.variant(PSELW::P_EQ_2)
    }
    #[doc = "P = 4"]
    #[inline(always)]
    pub fn p_eq_4(self) -> &'a mut W {
        self.variant(PSELW::P_EQ_4)
    }
    #[doc = "P = 8"]
    #[inline(always)]
    pub fn p_eq_8(self) -> &'a mut W {
        self.variant(PSELW::P_EQ_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:4 - Feedback divider value. The division value M is the programmed MSEL value + 1. 00000: Division ratio M = 1 to 11111: Division ratio M = 32"]
    #[inline(always)]
    pub fn msel(&self) -> MSEL_R {
        MSEL_R::new((self.bits() & 0x1f) as u8)
    }
    #[doc = "Bits 5:6 - Post divider ratio P. The division ratio is 2 x P."]
    #[inline(always)]
    pub fn psel(&self) -> PSEL_R {
        PSEL_R::new(((self.bits() >> 5) & 0x03) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:4 - Feedback divider value. The division value M is the programmed MSEL value + 1. 00000: Division ratio M = 1 to 11111: Division ratio M = 32"]
    #[inline(always)]
    pub fn msel(&mut self) -> _MSELW {
        _MSELW { w: self }
    }
    #[doc = "Bits 5:6 - Post divider ratio P. The division ratio is 2 x P."]
    #[inline(always)]
    pub fn psel(&mut self) -> _PSELW {
        _PSELW { w: self }
    }
}
