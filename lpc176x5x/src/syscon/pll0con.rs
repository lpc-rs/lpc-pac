#[doc = "Reader of register PLL0CON"]
pub type R = crate::R<u32, super::PLL0CON>;
#[doc = "Writer for register PLL0CON"]
pub type W = crate::W<u32, super::PLL0CON>;
#[doc = "Register PLL0CON `reset()`'s with value 0"]
impl crate::ResetValue for super::PLL0CON {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PLLE0`"]
pub type PLLE0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLLE0`"]
pub struct PLLE0_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLE0_W<'a> {
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
#[doc = "Reader of field `PLLC0`"]
pub type PLLC0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLLC0`"]
pub struct PLLC0_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLC0_W<'a> {
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
    #[doc = "Bit 0 - PLL0 Enable. When one, and after a valid PLL0 feed, this bit will activate PLL0 and allow it to lock to the requested frequency. See PLL0STAT register."]
    #[inline(always)]
    pub fn plle0(&self) -> PLLE0_R {
        PLLE0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PLL0 Connect. Setting PLLC0 to one after PLL0 has been enabled and locked, then followed by a valid PLL0 feed sequence causes PLL0 to become the clock source for the CPU, AHB peripherals, and used to derive the clocks for APB peripherals. The PLL0 output may potentially be used to clock the USB subsystem if the frequency is 48 MHz. See PLL0STAT register."]
    #[inline(always)]
    pub fn pllc0(&self) -> PLLC0_R {
        PLLC0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PLL0 Enable. When one, and after a valid PLL0 feed, this bit will activate PLL0 and allow it to lock to the requested frequency. See PLL0STAT register."]
    #[inline(always)]
    pub fn plle0(&mut self) -> PLLE0_W {
        PLLE0_W { w: self }
    }
    #[doc = "Bit 1 - PLL0 Connect. Setting PLLC0 to one after PLL0 has been enabled and locked, then followed by a valid PLL0 feed sequence causes PLL0 to become the clock source for the CPU, AHB peripherals, and used to derive the clocks for APB peripherals. The PLL0 output may potentially be used to clock the USB subsystem if the frequency is 48 MHz. See PLL0STAT register."]
    #[inline(always)]
    pub fn pllc0(&mut self) -> PLLC0_W {
        PLLC0_W { w: self }
    }
}
