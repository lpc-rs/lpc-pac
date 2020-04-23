#[doc = "Reader of register AHBCLKCTRL0"]
pub type R = crate::R<u32, super::AHBCLKCTRL0>;
#[doc = "Writer for register AHBCLKCTRL0"]
pub type W = crate::W<u32, super::AHBCLKCTRL0>;
#[doc = "Register AHBCLKCTRL0 `reset()`'s with value 0x0183"]
impl crate::ResetValue for super::AHBCLKCTRL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0183
    }
}
#[doc = "Reader of field `ROM`"]
pub type ROM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ROM`"]
pub struct ROM_W<'a> {
    w: &'a mut W,
}
impl<'a> ROM_W<'a> {
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
#[doc = "Reader of field `SRAM1`"]
pub type SRAM1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SRAM1`"]
pub struct SRAM1_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `SRAM2`"]
pub type SRAM2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SRAM2`"]
pub struct SRAM2_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `SRAM3`"]
pub type SRAM3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SRAM3`"]
pub struct SRAM3_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `FLASH`"]
pub type FLASH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLASH`"]
pub struct FLASH_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `FMC`"]
pub type FMC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FMC`"]
pub struct FMC_W<'a> {
    w: &'a mut W,
}
impl<'a> FMC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `EEPROM`"]
pub type EEPROM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EEPROM`"]
pub struct EEPROM_W<'a> {
    w: &'a mut W,
}
impl<'a> EEPROM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `SPIFI`"]
pub type SPIFI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPIFI`"]
pub struct SPIFI_W<'a> {
    w: &'a mut W,
}
impl<'a> SPIFI_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `INPUTMUX`"]
pub type INPUTMUX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INPUTMUX`"]
pub struct INPUTMUX_W<'a> {
    w: &'a mut W,
}
impl<'a> INPUTMUX_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `IOCON`"]
pub type IOCON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IOCON`"]
pub struct IOCON_W<'a> {
    w: &'a mut W,
}
impl<'a> IOCON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `GPIO0`"]
pub type GPIO0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO0`"]
pub struct GPIO0_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `GPIO1`"]
pub type GPIO1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO1`"]
pub struct GPIO1_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `GPIO2`"]
pub type GPIO2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO2`"]
pub struct GPIO2_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `GPIO3`"]
pub type GPIO3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO3`"]
pub struct GPIO3_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `PINT`"]
pub type PINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINT`"]
pub struct PINT_W<'a> {
    w: &'a mut W,
}
impl<'a> PINT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `GINT`"]
pub type GINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GINT`"]
pub struct GINT_W<'a> {
    w: &'a mut W,
}
impl<'a> GINT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `DMA`"]
pub type DMA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA`"]
pub struct DMA_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `CRC`"]
pub type CRC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRC`"]
pub struct CRC_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `WWDT`"]
pub type WWDT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WWDT`"]
pub struct WWDT_W<'a> {
    w: &'a mut W,
}
impl<'a> WWDT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `RTC`"]
pub type RTC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC`"]
pub struct RTC_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `ADC0`"]
pub type ADC0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC0`"]
pub struct ADC0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Enables the clock for the Boot ROM. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn rom(&self) -> ROM_R {
        ROM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enables the clock for SRAM1. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn sram1(&self) -> SRAM1_R {
        SRAM1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enables the clock for SRAM2. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn sram2(&self) -> SRAM2_R {
        SRAM2_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enables the clock for SRAM3."]
    #[inline(always)]
    pub fn sram3(&self) -> SRAM3_R {
        SRAM3_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enables the clock for the flash controller. 0 = Disable; 1 = Enable. This clock is needed for flash programming, not for flash read."]
    #[inline(always)]
    pub fn flash(&self) -> FLASH_R {
        FLASH_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enables the clock for the Flash accelerator. 0 = Disable; 1 = Enable. This clock is needed if the flash is being read."]
    #[inline(always)]
    pub fn fmc(&self) -> FMC_R {
        FMC_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Enables the clock for EEPROM."]
    #[inline(always)]
    pub fn eeprom(&self) -> EEPROM_R {
        EEPROM_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Enables the clock for the SPIFI. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn spifi(&self) -> SPIFI_R {
        SPIFI_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Enables the clock for the input muxes. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn inputmux(&self) -> INPUTMUX_R {
        INPUTMUX_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enables the clock for the IOCON block. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn iocon(&self) -> IOCON_R {
        IOCON_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Enables the clock for the GPIO0 port registers. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn gpio0(&self) -> GPIO0_R {
        GPIO0_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Enables the clock for the GPIO1 port registers. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn gpio1(&self) -> GPIO1_R {
        GPIO1_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Enables the clock for the GPIO2 port registers."]
    #[inline(always)]
    pub fn gpio2(&self) -> GPIO2_R {
        GPIO2_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Enables the clock for the GPIO3 port registers."]
    #[inline(always)]
    pub fn gpio3(&self) -> GPIO3_R {
        GPIO3_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Enables the clock for the pin interrupt block.0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn pint(&self) -> PINT_R {
        PINT_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Enables the clock for the grouped pin interrupt block. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn gint(&self) -> GINT_R {
        GINT_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Enables the clock for the DMA controller. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Enables the clock for the CRC engine. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn crc(&self) -> CRC_R {
        CRC_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Enables the clock for the Watchdog Timer. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn wwdt(&self) -> WWDT_R {
        WWDT_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Enables the bus clock for the RTC. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Enables the clock for the ADC0 register interface."]
    #[inline(always)]
    pub fn adc0(&self) -> ADC0_R {
        ADC0_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Enables the clock for the Boot ROM. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn rom(&mut self) -> ROM_W {
        ROM_W { w: self }
    }
    #[doc = "Bit 3 - Enables the clock for SRAM1. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn sram1(&mut self) -> SRAM1_W {
        SRAM1_W { w: self }
    }
    #[doc = "Bit 4 - Enables the clock for SRAM2. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn sram2(&mut self) -> SRAM2_W {
        SRAM2_W { w: self }
    }
    #[doc = "Bit 5 - Enables the clock for SRAM3."]
    #[inline(always)]
    pub fn sram3(&mut self) -> SRAM3_W {
        SRAM3_W { w: self }
    }
    #[doc = "Bit 7 - Enables the clock for the flash controller. 0 = Disable; 1 = Enable. This clock is needed for flash programming, not for flash read."]
    #[inline(always)]
    pub fn flash(&mut self) -> FLASH_W {
        FLASH_W { w: self }
    }
    #[doc = "Bit 8 - Enables the clock for the Flash accelerator. 0 = Disable; 1 = Enable. This clock is needed if the flash is being read."]
    #[inline(always)]
    pub fn fmc(&mut self) -> FMC_W {
        FMC_W { w: self }
    }
    #[doc = "Bit 9 - Enables the clock for EEPROM."]
    #[inline(always)]
    pub fn eeprom(&mut self) -> EEPROM_W {
        EEPROM_W { w: self }
    }
    #[doc = "Bit 10 - Enables the clock for the SPIFI. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn spifi(&mut self) -> SPIFI_W {
        SPIFI_W { w: self }
    }
    #[doc = "Bit 11 - Enables the clock for the input muxes. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn inputmux(&mut self) -> INPUTMUX_W {
        INPUTMUX_W { w: self }
    }
    #[doc = "Bit 13 - Enables the clock for the IOCON block. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn iocon(&mut self) -> IOCON_W {
        IOCON_W { w: self }
    }
    #[doc = "Bit 14 - Enables the clock for the GPIO0 port registers. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn gpio0(&mut self) -> GPIO0_W {
        GPIO0_W { w: self }
    }
    #[doc = "Bit 15 - Enables the clock for the GPIO1 port registers. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn gpio1(&mut self) -> GPIO1_W {
        GPIO1_W { w: self }
    }
    #[doc = "Bit 16 - Enables the clock for the GPIO2 port registers."]
    #[inline(always)]
    pub fn gpio2(&mut self) -> GPIO2_W {
        GPIO2_W { w: self }
    }
    #[doc = "Bit 17 - Enables the clock for the GPIO3 port registers."]
    #[inline(always)]
    pub fn gpio3(&mut self) -> GPIO3_W {
        GPIO3_W { w: self }
    }
    #[doc = "Bit 18 - Enables the clock for the pin interrupt block.0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn pint(&mut self) -> PINT_W {
        PINT_W { w: self }
    }
    #[doc = "Bit 19 - Enables the clock for the grouped pin interrupt block. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn gint(&mut self) -> GINT_W {
        GINT_W { w: self }
    }
    #[doc = "Bit 20 - Enables the clock for the DMA controller. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn dma(&mut self) -> DMA_W {
        DMA_W { w: self }
    }
    #[doc = "Bit 21 - Enables the clock for the CRC engine. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn crc(&mut self) -> CRC_W {
        CRC_W { w: self }
    }
    #[doc = "Bit 22 - Enables the clock for the Watchdog Timer. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn wwdt(&mut self) -> WWDT_W {
        WWDT_W { w: self }
    }
    #[doc = "Bit 23 - Enables the bus clock for the RTC. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn rtc(&mut self) -> RTC_W {
        RTC_W { w: self }
    }
    #[doc = "Bit 27 - Enables the clock for the ADC0 register interface."]
    #[inline(always)]
    pub fn adc0(&mut self) -> ADC0_W {
        ADC0_W { w: self }
    }
}
