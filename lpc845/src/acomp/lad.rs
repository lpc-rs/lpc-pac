#[doc = "Reader of register LAD"]
pub type R = crate::R<u32, super::LAD>;
#[doc = "Writer for register LAD"]
pub type W = crate::W<u32, super::LAD>;
#[doc = "Register LAD `reset()`'s with value 0"]
impl crate::ResetValue for super::LAD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LADEN`"]
pub type LADEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LADEN`"]
pub struct LADEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LADEN_W<'a> {
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
#[doc = "Reader of field `LADSEL`"]
pub type LADSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LADSEL`"]
pub struct LADSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LADSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 1)) | (((value as u32) & 0x1f) << 1);
        self.w
    }
}
#[doc = "Selects the reference voltage Vref for the voltage ladder.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LADREF_A {
    #[doc = "0: Supply pin VDD"]
    LADREF_0 = 0,
    #[doc = "1: VDDCMP pin"]
    LADREF_1 = 1,
}
impl From<LADREF_A> for bool {
    #[inline(always)]
    fn from(variant: LADREF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LADREF`"]
pub type LADREF_R = crate::R<bool, LADREF_A>;
impl LADREF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LADREF_A {
        match self.bits {
            false => LADREF_A::LADREF_0,
            true => LADREF_A::LADREF_1,
        }
    }
    #[doc = "Checks if the value of the field is `LADREF_0`"]
    #[inline(always)]
    pub fn is_ladref_0(&self) -> bool {
        *self == LADREF_A::LADREF_0
    }
    #[doc = "Checks if the value of the field is `LADREF_1`"]
    #[inline(always)]
    pub fn is_ladref_1(&self) -> bool {
        *self == LADREF_A::LADREF_1
    }
}
#[doc = "Write proxy for field `LADREF`"]
pub struct LADREF_W<'a> {
    w: &'a mut W,
}
impl<'a> LADREF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LADREF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Supply pin VDD"]
    #[inline(always)]
    pub fn ladref_0(self) -> &'a mut W {
        self.variant(LADREF_A::LADREF_0)
    }
    #[doc = "VDDCMP pin"]
    #[inline(always)]
    pub fn ladref_1(self) -> &'a mut W {
        self.variant(LADREF_A::LADREF_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Voltage ladder enable"]
    #[inline(always)]
    pub fn laden(&self) -> LADEN_R {
        LADEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:5 - Voltage ladder value. The reference voltage Vref depends on the LADREF bit below. 00000 = VSS 00001 = 1 x Vref/31 00010 = 2 x Vref/31 ... 11111 = Vref"]
    #[inline(always)]
    pub fn ladsel(&self) -> LADSEL_R {
        LADSEL_R::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bit 6 - Selects the reference voltage Vref for the voltage ladder."]
    #[inline(always)]
    pub fn ladref(&self) -> LADREF_R {
        LADREF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Voltage ladder enable"]
    #[inline(always)]
    pub fn laden(&mut self) -> LADEN_W {
        LADEN_W { w: self }
    }
    #[doc = "Bits 1:5 - Voltage ladder value. The reference voltage Vref depends on the LADREF bit below. 00000 = VSS 00001 = 1 x Vref/31 00010 = 2 x Vref/31 ... 11111 = Vref"]
    #[inline(always)]
    pub fn ladsel(&mut self) -> LADSEL_W {
        LADSEL_W { w: self }
    }
    #[doc = "Bit 6 - Selects the reference voltage Vref for the voltage ladder."]
    #[inline(always)]
    pub fn ladref(&mut self) -> LADREF_W {
        LADREF_W { w: self }
    }
}
