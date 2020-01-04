#[doc = "Reader of register FCLKSEL[%s]"]
pub type R = crate::R<u32, super::FCLKSEL>;
#[doc = "Writer for register FCLKSEL[%s]"]
pub type W = crate::W<u32, super::FCLKSEL>;
#[doc = "Register FCLKSEL[%s] `reset()`'s with value 0x07"]
impl crate::ResetValue for super::FCLKSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x07
    }
}
#[doc = "Peripheral clock source\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEL_A {
    #[doc = "0: FRO"]
    FRO,
    #[doc = "1: main clock"]
    MAIN_CLK,
    #[doc = "2: Frg0clk"]
    FRG0CLK,
    #[doc = "3: Frg1clk"]
    FRG1CLK,
    #[doc = "4: FRO_DIV"]
    FRO_DIV,
    #[doc = "7: none"]
    NONE,
}
impl From<SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SEL_A) -> Self {
        match variant {
            SEL_A::FRO => 0,
            SEL_A::MAIN_CLK => 1,
            SEL_A::FRG0CLK => 2,
            SEL_A::FRG1CLK => 3,
            SEL_A::FRO_DIV => 4,
            SEL_A::NONE => 7,
        }
    }
}
#[doc = "Reader of field `SEL`"]
pub type SEL_R = crate::R<u8, SEL_A>;
impl SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SEL_A::FRO),
            1 => Val(SEL_A::MAIN_CLK),
            2 => Val(SEL_A::FRG0CLK),
            3 => Val(SEL_A::FRG1CLK),
            4 => Val(SEL_A::FRO_DIV),
            7 => Val(SEL_A::NONE),
            i => Res(i),
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
    #[doc = "Checks if the value of the field is `FRG0CLK`"]
    #[inline(always)]
    pub fn is_frg0clk(&self) -> bool {
        *self == SEL_A::FRG0CLK
    }
    #[doc = "Checks if the value of the field is `FRG1CLK`"]
    #[inline(always)]
    pub fn is_frg1clk(&self) -> bool {
        *self == SEL_A::FRG1CLK
    }
    #[doc = "Checks if the value of the field is `FRO_DIV`"]
    #[inline(always)]
    pub fn is_fro_div(&self) -> bool {
        *self == SEL_A::FRO_DIV
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
        unsafe { self.bits(variant.into()) }
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
    #[doc = "Frg0clk"]
    #[inline(always)]
    pub fn frg0clk(self) -> &'a mut W {
        self.variant(SEL_A::FRG0CLK)
    }
    #[doc = "Frg1clk"]
    #[inline(always)]
    pub fn frg1clk(self) -> &'a mut W {
        self.variant(SEL_A::FRG1CLK)
    }
    #[doc = "FRO_DIV"]
    #[inline(always)]
    pub fn fro_div(self) -> &'a mut W {
        self.variant(SEL_A::FRO_DIV)
    }
    #[doc = "none"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SEL_A::NONE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Peripheral clock source"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Peripheral clock source"]
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W {
        SEL_W { w: self }
    }
}
