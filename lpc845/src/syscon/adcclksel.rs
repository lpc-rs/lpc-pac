#[doc = "Reader of register ADCCLKSEL"]
pub type R = crate::R<u32, super::ADCCLKSEL>;
#[doc = "Writer for register ADCCLKSEL"]
pub type W = crate::W<u32, super::ADCCLKSEL>;
#[doc = "Register ADCCLKSEL `reset()`'s with value 0"]
impl crate::ResetValue for super::ADCCLKSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Clock source for ADC clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SEL_A {
    #[doc = "0: FRO"]
    FRO = 0,
    #[doc = "1: sys pll"]
    SYS_PLL = 1,
    #[doc = "2: none"]
    NONE = 2,
    #[doc = "3: none"]
    NONE1 = 3,
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
            1 => SEL_A::SYS_PLL,
            2 => SEL_A::NONE,
            3 => SEL_A::NONE1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FRO`"]
    #[inline(always)]
    pub fn is_fro(&self) -> bool {
        *self == SEL_A::FRO
    }
    #[doc = "Checks if the value of the field is `SYS_PLL`"]
    #[inline(always)]
    pub fn is_sys_pll(&self) -> bool {
        *self == SEL_A::SYS_PLL
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
    #[doc = "sys pll"]
    #[inline(always)]
    pub fn sys_pll(self) -> &'a mut W {
        self.variant(SEL_A::SYS_PLL)
    }
    #[doc = "none"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SEL_A::NONE)
    }
    #[doc = "none"]
    #[inline(always)]
    pub fn none1(self) -> &'a mut W {
        self.variant(SEL_A::NONE1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Clock source for ADC clock"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Clock source for ADC clock"]
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W {
        SEL_W { w: self }
    }
}
