#[doc = "Reader of register PLL1CON"]
pub type R = crate::R<u32, super::PLL1CON>;
#[doc = "Writer for register PLL1CON"]
pub type W = crate::W<u32, super::PLL1CON>;
#[doc = "Register PLL1CON `reset()`'s with value 0"]
impl crate::ResetValue for super::PLL1CON {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PLLE1`"]
pub type PLLE1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLLE1`"]
pub struct PLLE1_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLE1_W<'a> {
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
#[doc = "Reader of field `PLLC1`"]
pub type PLLC1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLLC1`"]
pub struct PLLC1_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLC1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - PLL1 Enable. When one, and after a valid PLL1 feed, this bit will activate PLL1 and allow it to lock to the requested frequency."]
    #[inline(always)]
    pub fn plle1(&self) -> PLLE1_R {
        PLLE1_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PLL1 Connect. Setting PLLC to one after PLL1 has been enabled and locked, then followed by a valid PLL1 feed sequence causes PLL1 to become the clock source for the USB subsystem via the USB clock divider. See PLL1STAT register."]
    #[inline(always)]
    pub fn pllc1(&self) -> PLLC1_R {
        PLLC1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PLL1 Enable. When one, and after a valid PLL1 feed, this bit will activate PLL1 and allow it to lock to the requested frequency."]
    #[inline(always)]
    pub fn plle1(&mut self) -> PLLE1_W {
        PLLE1_W { w: self }
    }
    #[doc = "Bit 1 - PLL1 Connect. Setting PLLC to one after PLL1 has been enabled and locked, then followed by a valid PLL1 feed sequence causes PLL1 to become the clock source for the USB subsystem via the USB clock divider. See PLL1STAT register."]
    #[inline(always)]
    pub fn pllc1(&mut self) -> PLLC1_W {
        PLLC1_W { w: self }
    }
}
