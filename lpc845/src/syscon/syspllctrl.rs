#[doc = "Reader of register SYSPLLCTRL"]
pub type R = crate::R<u32, super::SYSPLLCTRL>;
#[doc = "Writer for register SYSPLLCTRL"]
pub type W = crate::W<u32, super::SYSPLLCTRL>;
#[doc = "Register SYSPLLCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSPLLCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MSEL`"]
pub type MSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MSEL`"]
pub struct MSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Post divider ratio P. The division ratio is 2 x P.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PSEL_A {
    #[doc = "0: P = 1"]
    PSEL_0 = 0,
    #[doc = "1: P = 2"]
    PSEL_1 = 1,
    #[doc = "2: P = 4"]
    PSEL_2 = 2,
    #[doc = "3: P = 8"]
    PSEL_3 = 3,
}
impl From<PSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PSEL`"]
pub type PSEL_R = crate::R<u8, PSEL_A>;
impl PSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSEL_A {
        match self.bits {
            0 => PSEL_A::PSEL_0,
            1 => PSEL_A::PSEL_1,
            2 => PSEL_A::PSEL_2,
            3 => PSEL_A::PSEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PSEL_0`"]
    #[inline(always)]
    pub fn is_psel_0(&self) -> bool {
        *self == PSEL_A::PSEL_0
    }
    #[doc = "Checks if the value of the field is `PSEL_1`"]
    #[inline(always)]
    pub fn is_psel_1(&self) -> bool {
        *self == PSEL_A::PSEL_1
    }
    #[doc = "Checks if the value of the field is `PSEL_2`"]
    #[inline(always)]
    pub fn is_psel_2(&self) -> bool {
        *self == PSEL_A::PSEL_2
    }
    #[doc = "Checks if the value of the field is `PSEL_3`"]
    #[inline(always)]
    pub fn is_psel_3(&self) -> bool {
        *self == PSEL_A::PSEL_3
    }
}
#[doc = "Write proxy for field `PSEL`"]
pub struct PSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "P = 1"]
    #[inline(always)]
    pub fn psel_0(self) -> &'a mut W {
        self.variant(PSEL_A::PSEL_0)
    }
    #[doc = "P = 2"]
    #[inline(always)]
    pub fn psel_1(self) -> &'a mut W {
        self.variant(PSEL_A::PSEL_1)
    }
    #[doc = "P = 4"]
    #[inline(always)]
    pub fn psel_2(self) -> &'a mut W {
        self.variant(PSEL_A::PSEL_2)
    }
    #[doc = "P = 8"]
    #[inline(always)]
    pub fn psel_3(self) -> &'a mut W {
        self.variant(PSEL_A::PSEL_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Feedback divider value. The division value M is the programmed MSEL value + 1. 00000: Division ratio M = 1 to 11111: Division ratio M = 32"]
    #[inline(always)]
    pub fn msel(&self) -> MSEL_R {
        MSEL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:6 - Post divider ratio P. The division ratio is 2 x P."]
    #[inline(always)]
    pub fn psel(&self) -> PSEL_R {
        PSEL_R::new(((self.bits >> 5) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Feedback divider value. The division value M is the programmed MSEL value + 1. 00000: Division ratio M = 1 to 11111: Division ratio M = 32"]
    #[inline(always)]
    pub fn msel(&mut self) -> MSEL_W {
        MSEL_W { w: self }
    }
    #[doc = "Bits 5:6 - Post divider ratio P. The division ratio is 2 x P."]
    #[inline(always)]
    pub fn psel(&mut self) -> PSEL_W {
        PSEL_W { w: self }
    }
}
