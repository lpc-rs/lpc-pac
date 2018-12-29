#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::USBPLLCTRL {
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
pub struct MSELR {
    bits: u8,
}
impl MSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
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
impl PSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PSELR::P_EQ_1 => 0,
            PSELR::P_EQ_2 => 1,
            PSELR::P_EQ_4 => 2,
            PSELR::P_EQ_8 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PSELR {
        match value {
            0 => PSELR::P_EQ_1,
            1 => PSELR::P_EQ_2,
            2 => PSELR::P_EQ_4,
            3 => PSELR::P_EQ_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `P_EQ_1`"]
    #[inline]
    pub fn is_p_eq_1(&self) -> bool {
        *self == PSELR::P_EQ_1
    }
    #[doc = "Checks if the value of the field is `P_EQ_2`"]
    #[inline]
    pub fn is_p_eq_2(&self) -> bool {
        *self == PSELR::P_EQ_2
    }
    #[doc = "Checks if the value of the field is `P_EQ_4`"]
    #[inline]
    pub fn is_p_eq_4(&self) -> bool {
        *self == PSELR::P_EQ_4
    }
    #[doc = "Checks if the value of the field is `P_EQ_8`"]
    #[inline]
    pub fn is_p_eq_8(&self) -> bool {
        *self == PSELR::P_EQ_8
    }
}
#[doc = r" Proxy"]
pub struct _MSELW<'a> {
    w: &'a mut W,
}
impl<'a> _MSELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PSEL`"]
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
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PSELW::P_EQ_1 => 0,
            PSELW::P_EQ_2 => 1,
            PSELW::P_EQ_4 => 2,
            PSELW::P_EQ_8 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "P = 1"]
    #[inline]
    pub fn p_eq_1(self) -> &'a mut W {
        self.variant(PSELW::P_EQ_1)
    }
    #[doc = "P = 2"]
    #[inline]
    pub fn p_eq_2(self) -> &'a mut W {
        self.variant(PSELW::P_EQ_2)
    }
    #[doc = "P = 4"]
    #[inline]
    pub fn p_eq_4(self) -> &'a mut W {
        self.variant(PSELW::P_EQ_4)
    }
    #[doc = "P = 8"]
    #[inline]
    pub fn p_eq_8(self) -> &'a mut W {
        self.variant(PSELW::P_EQ_8)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 5;
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
    #[doc = "Bits 0:4 - Feedback divider value. The division value M is the programmed MSEL value + 1. 00000: Division ratio M = 1 to 11111: Division ratio M = 32"]
    #[inline]
    pub fn msel(&self) -> MSELR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MSELR { bits }
    }
    #[doc = "Bits 5:6 - Post divider ratio P. The division ratio is 2 x P."]
    #[inline]
    pub fn psel(&self) -> PSELR {
        PSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 5;
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
    #[doc = "Bits 0:4 - Feedback divider value. The division value M is the programmed MSEL value + 1. 00000: Division ratio M = 1 to 11111: Division ratio M = 32"]
    #[inline]
    pub fn msel(&mut self) -> _MSELW {
        _MSELW { w: self }
    }
    #[doc = "Bits 5:6 - Post divider ratio P. The division ratio is 2 x P."]
    #[inline]
    pub fn psel(&mut self) -> _PSELW {
        _PSELW { w: self }
    }
}
