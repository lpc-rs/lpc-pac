#[doc = "Reader of register SYSPLLCLKSEL"]
pub type R = crate::R<u32, super::SYSPLLCLKSEL>;
#[doc = "Writer for register SYSPLLCLKSEL"]
pub type W = crate::W<u32, super::SYSPLLCLKSEL>;
#[doc = "Register SYSPLLCLKSEL `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSPLLCLKSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "System PLL clock source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SEL_A {
    #[doc = "0: FRO"]
    FRO = 0,
    #[doc = "1: External clock"]
    EXT_CLK = 1,
    #[doc = "2: Watchdog oscillator"]
    WDTOSC = 2,
    #[doc = "3: FRO DIV"]
    FRODIV = 3,
}
impl From<SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SEL`"]
pub type SEL_R = crate::R<u8, SEL_A>;
impl SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEL_A {
        match self.bits {
            0 => SEL_A::FRO,
            1 => SEL_A::EXT_CLK,
            2 => SEL_A::WDTOSC,
            3 => SEL_A::FRODIV,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FRO`"]
    #[inline(always)]
    pub fn is_fro(&self) -> bool {
        *self == SEL_A::FRO
    }
    #[doc = "Checks if the value of the field is `EXT_CLK`"]
    #[inline(always)]
    pub fn is_ext_clk(&self) -> bool {
        *self == SEL_A::EXT_CLK
    }
    #[doc = "Checks if the value of the field is `WDTOSC`"]
    #[inline(always)]
    pub fn is_wdtosc(&self) -> bool {
        *self == SEL_A::WDTOSC
    }
    #[doc = "Checks if the value of the field is `FRODIV`"]
    #[inline(always)]
    pub fn is_frodiv(&self) -> bool {
        *self == SEL_A::FRODIV
    }
}
#[doc = "Write proxy for field `SEL`"]
pub struct SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "FRO"]
    #[inline(always)]
    pub fn fro(self) -> &'a mut W {
        self.variant(SEL_A::FRO)
    }
    #[doc = "External clock"]
    #[inline(always)]
    pub fn ext_clk(self) -> &'a mut W {
        self.variant(SEL_A::EXT_CLK)
    }
    #[doc = "Watchdog oscillator"]
    #[inline(always)]
    pub fn wdtosc(self) -> &'a mut W {
        self.variant(SEL_A::WDTOSC)
    }
    #[doc = "FRO DIV"]
    #[inline(always)]
    pub fn frodiv(self) -> &'a mut W {
        self.variant(SEL_A::FRODIV)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - System PLL clock source"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - System PLL clock source"]
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W {
        SEL_W { w: self }
    }
}
