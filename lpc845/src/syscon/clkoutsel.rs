#[doc = "Reader of register CLKOUTSEL"]
pub type R = crate::R<u32, super::CLKOUTSEL>;
#[doc = "Writer for register CLKOUTSEL"]
pub type W = crate::W<u32, super::CLKOUTSEL>;
#[doc = "Register CLKOUTSEL `reset()`'s with value 0"]
impl crate::ResetValue for super::CLKOUTSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "CLKOUT clock source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SEL_A {
    #[doc = "0: FRO"]
    FRO = 0,
    #[doc = "1: main clock"]
    MAIN_CLK = 1,
    #[doc = "2: sys pll"]
    SYS_PLL = 2,
    #[doc = "3: external clock"]
    EXT_CLK = 3,
    #[doc = "4: Watchdog oscillator"]
    WDTOSC = 4,
    #[doc = "5: None"]
    NONE = 5,
    #[doc = "6: None"]
    NONE1 = 6,
    #[doc = "7: None"]
    NONE2 = 7,
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
            1 => SEL_A::MAIN_CLK,
            2 => SEL_A::SYS_PLL,
            3 => SEL_A::EXT_CLK,
            4 => SEL_A::WDTOSC,
            5 => SEL_A::NONE,
            6 => SEL_A::NONE1,
            7 => SEL_A::NONE2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FRO`"]
    #[inline(always)]
    pub fn is_fro(&self) -> bool {
        *self == SEL_A::FRO
    }
    #[doc = "Checks if the value of the field is `MAIN_CLK`"]
    #[inline(always)]
    pub fn is_main_clk(&self) -> bool {
        *self == SEL_A::MAIN_CLK
    }
    #[doc = "Checks if the value of the field is `SYS_PLL`"]
    #[inline(always)]
    pub fn is_sys_pll(&self) -> bool {
        *self == SEL_A::SYS_PLL
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
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SEL_A::NONE
    }
    #[doc = "Checks if the value of the field is `NONE1`"]
    #[inline(always)]
    pub fn is_none1(&self) -> bool {
        *self == SEL_A::NONE1
    }
    #[doc = "Checks if the value of the field is `NONE2`"]
    #[inline(always)]
    pub fn is_none2(&self) -> bool {
        *self == SEL_A::NONE2
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
    #[doc = "main clock"]
    #[inline(always)]
    pub fn main_clk(self) -> &'a mut W {
        self.variant(SEL_A::MAIN_CLK)
    }
    #[doc = "sys pll"]
    #[inline(always)]
    pub fn sys_pll(self) -> &'a mut W {
        self.variant(SEL_A::SYS_PLL)
    }
    #[doc = "external clock"]
    #[inline(always)]
    pub fn ext_clk(self) -> &'a mut W {
        self.variant(SEL_A::EXT_CLK)
    }
    #[doc = "Watchdog oscillator"]
    #[inline(always)]
    pub fn wdtosc(self) -> &'a mut W {
        self.variant(SEL_A::WDTOSC)
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SEL_A::NONE)
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn none1(self) -> &'a mut W {
        self.variant(SEL_A::NONE1)
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn none2(self) -> &'a mut W {
        self.variant(SEL_A::NONE2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - CLKOUT clock source"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - CLKOUT clock source"]
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W {
        SEL_W { w: self }
    }
}
