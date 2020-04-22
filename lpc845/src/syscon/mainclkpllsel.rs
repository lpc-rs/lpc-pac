#[doc = "Reader of register MAINCLKPLLSEL"]
pub type R = crate::R<u32, super::MAINCLKPLLSEL>;
#[doc = "Writer for register MAINCLKPLLSEL"]
pub type W = crate::W<u32, super::MAINCLKPLLSEL>;
#[doc = "Register MAINCLKPLLSEL `reset()`'s with value 0"]
impl crate::ResetValue for super::MAINCLKPLLSEL {
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
    #[doc = "0: main_clk_pre_pll"]
    MAIN_CLK_PRE_PLL = 0,
    #[doc = "1: sys pll"]
    SYS_PLL = 1,
    #[doc = "2: none"]
    SEL_2 = 2,
    #[doc = "3: none"]
    SEL_3 = 3,
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
            0 => SEL_A::MAIN_CLK_PRE_PLL,
            1 => SEL_A::SYS_PLL,
            2 => SEL_A::SEL_2,
            3 => SEL_A::SEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MAIN_CLK_PRE_PLL`"]
    #[inline(always)]
    pub fn is_main_clk_pre_pll(&self) -> bool {
        *self == SEL_A::MAIN_CLK_PRE_PLL
    }
    #[doc = "Checks if the value of the field is `SYS_PLL`"]
    #[inline(always)]
    pub fn is_sys_pll(&self) -> bool {
        *self == SEL_A::SYS_PLL
    }
    #[doc = "Checks if the value of the field is `SEL_2`"]
    #[inline(always)]
    pub fn is_sel_2(&self) -> bool {
        *self == SEL_A::SEL_2
    }
    #[doc = "Checks if the value of the field is `SEL_3`"]
    #[inline(always)]
    pub fn is_sel_3(&self) -> bool {
        *self == SEL_A::SEL_3
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
    #[doc = "main_clk_pre_pll"]
    #[inline(always)]
    pub fn main_clk_pre_pll(self) -> &'a mut W {
        self.variant(SEL_A::MAIN_CLK_PRE_PLL)
    }
    #[doc = "sys pll"]
    #[inline(always)]
    pub fn sys_pll(self) -> &'a mut W {
        self.variant(SEL_A::SYS_PLL)
    }
    #[doc = "none"]
    #[inline(always)]
    pub fn sel_2(self) -> &'a mut W {
        self.variant(SEL_A::SEL_2)
    }
    #[doc = "none"]
    #[inline(always)]
    pub fn sel_3(self) -> &'a mut W {
        self.variant(SEL_A::SEL_3)
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
