#[doc = "Reader of register EXTCLKSEL"]
pub type R = crate::R<u32, super::EXTCLKSEL>;
#[doc = "Writer for register EXTCLKSEL"]
pub type W = crate::W<u32, super::EXTCLKSEL>;
#[doc = "Register EXTCLKSEL `reset()`'s with value 0"]
impl crate::ResetValue for super::EXTCLKSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Clock source for external clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEL_A {
    #[doc = "0: System oscillator"]
    SYS_OSC = 0,
    #[doc = "1: Clk_in"]
    CLK_IN = 1,
}
impl From<SEL_A> for bool {
    #[inline(always)]
    fn from(variant: SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SEL`"]
pub type SEL_R = crate::R<bool, SEL_A>;
impl SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEL_A {
        match self.bits {
            false => SEL_A::SYS_OSC,
            true => SEL_A::CLK_IN,
        }
    }
    #[doc = "Checks if the value of the field is `SYS_OSC`"]
    #[inline(always)]
    pub fn is_sys_osc(&self) -> bool {
        *self == SEL_A::SYS_OSC
    }
    #[doc = "Checks if the value of the field is `CLK_IN`"]
    #[inline(always)]
    pub fn is_clk_in(&self) -> bool {
        *self == SEL_A::CLK_IN
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
            self.bit(variant.into())
        }
    }
    #[doc = "System oscillator"]
    #[inline(always)]
    pub fn sys_osc(self) -> &'a mut W {
        self.variant(SEL_A::SYS_OSC)
    }
    #[doc = "Clk_in"]
    #[inline(always)]
    pub fn clk_in(self) -> &'a mut W {
        self.variant(SEL_A::CLK_IN)
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
    #[doc = "Bit 0 - Clock source for external clock"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock source for external clock"]
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W {
        SEL_W { w: self }
    }
}
