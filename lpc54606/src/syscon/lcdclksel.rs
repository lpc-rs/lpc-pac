#[doc = "Reader of register LCDCLKSEL"]
pub type R = crate::R<u32, super::LCDCLKSEL>;
#[doc = "Writer for register LCDCLKSEL"]
pub type W = crate::W<u32, super::LCDCLKSEL>;
#[doc = "Register LCDCLKSEL `reset()`'s with value 0x03"]
impl crate::ResetValue for super::LCDCLKSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x03
    }
}
#[doc = "LCD clock source select.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SEL_A {
    #[doc = "0: Main clock (main_clk)"]
    MAIN_CLOCK = 0,
    #[doc = "1: LCDCLKIN (LCDCLK_EXT)"]
    LCDCLKIN = 1,
    #[doc = "2: FRO 96 or 48 MHz (fro_hf)"]
    FRO_HF = 2,
    #[doc = "3: None, this may be selected in order to reduce power when no output is needed."]
    NONE = 3,
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
            0 => SEL_A::MAIN_CLOCK,
            1 => SEL_A::LCDCLKIN,
            2 => SEL_A::FRO_HF,
            3 => SEL_A::NONE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MAIN_CLOCK`"]
    #[inline(always)]
    pub fn is_main_clock(&self) -> bool {
        *self == SEL_A::MAIN_CLOCK
    }
    #[doc = "Checks if the value of the field is `LCDCLKIN`"]
    #[inline(always)]
    pub fn is_lcdclkin(&self) -> bool {
        *self == SEL_A::LCDCLKIN
    }
    #[doc = "Checks if the value of the field is `FRO_HF`"]
    #[inline(always)]
    pub fn is_fro_hf(&self) -> bool {
        *self == SEL_A::FRO_HF
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SEL_A::NONE
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
    #[doc = "Main clock (main_clk)"]
    #[inline(always)]
    pub fn main_clock(self) -> &'a mut W {
        self.variant(SEL_A::MAIN_CLOCK)
    }
    #[doc = "LCDCLKIN (LCDCLK_EXT)"]
    #[inline(always)]
    pub fn lcdclkin(self) -> &'a mut W {
        self.variant(SEL_A::LCDCLKIN)
    }
    #[doc = "FRO 96 or 48 MHz (fro_hf)"]
    #[inline(always)]
    pub fn fro_hf(self) -> &'a mut W {
        self.variant(SEL_A::FRO_HF)
    }
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SEL_A::NONE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - LCD clock source select."]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - LCD clock source select."]
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W {
        SEL_W { w: self }
    }
}
