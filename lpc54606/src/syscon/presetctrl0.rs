#[doc = "Reader of register PRESETCTRL0"]
pub type R = crate::R<u32, super::PRESETCTRL0>;
#[doc = "Writer for register PRESETCTRL0"]
pub type W = crate::W<u32, super::PRESETCTRL0>;
#[doc = "Register PRESETCTRL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::PRESETCTRL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FLASH_RST`"]
pub type FLASH_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLASH_RST`"]
pub struct FLASH_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_RST_W<'a> {
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
#[doc = "Reader of field `FMC_RST`"]
pub type FMC_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FMC_RST`"]
pub struct FMC_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> FMC_RST_W<'a> {
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
#[doc = "Reader of field `EEPROM_RST`"]
pub type EEPROM_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EEPROM_RST`"]
pub struct EEPROM_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> EEPROM_RST_W<'a> {
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
#[doc = "Reader of field `SPIFI_RST`"]
pub type SPIFI_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPIFI_RST`"]
pub struct SPIFI_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SPIFI_RST_W<'a> {
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
#[doc = "Reader of field `MUX_RST`"]
pub type MUX_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MUX_RST`"]
pub struct MUX_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_RST_W<'a> {
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
#[doc = "Reader of field `IOCON_RST`"]
pub type IOCON_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IOCON_RST`"]
pub struct IOCON_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> IOCON_RST_W<'a> {
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
#[doc = "Reader of field `GPIO0_RST`"]
pub type GPIO0_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO0_RST`"]
pub struct GPIO0_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO0_RST_W<'a> {
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
#[doc = "Reader of field `GPIO1_RST`"]
pub type GPIO1_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO1_RST`"]
pub struct GPIO1_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO1_RST_W<'a> {
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
#[doc = "Reader of field `GPIO2_RST`"]
pub type GPIO2_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO2_RST`"]
pub struct GPIO2_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO2_RST_W<'a> {
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
#[doc = "Reader of field `GPIO3_RST`"]
pub type GPIO3_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO3_RST`"]
pub struct GPIO3_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO3_RST_W<'a> {
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
#[doc = "Reader of field `PINT_RST`"]
pub type PINT_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINT_RST`"]
pub struct PINT_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> PINT_RST_W<'a> {
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
#[doc = "Reader of field `GINT_RST`"]
pub type GINT_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GINT_RST`"]
pub struct GINT_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> GINT_RST_W<'a> {
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
#[doc = "Reader of field `DMA0_RST`"]
pub type DMA0_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA0_RST`"]
pub struct DMA0_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA0_RST_W<'a> {
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
#[doc = "Reader of field `CRC_RST`"]
pub type CRC_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRC_RST`"]
pub struct CRC_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC_RST_W<'a> {
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
#[doc = "Reader of field `WWDT_RST`"]
pub type WWDT_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WWDT_RST`"]
pub struct WWDT_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> WWDT_RST_W<'a> {
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
#[doc = "Reader of field `ADC0_RST`"]
pub type ADC0_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC0_RST`"]
pub struct ADC0_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC0_RST_W<'a> {
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
    #[doc = "Bit 7 - Flash controller reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn flash_rst(&self) -> FLASH_RST_R {
        FLASH_RST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Flash accelerator reset control. Note that the FMC must not be reset while executing from flash, and must be reconfigured after reset. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn fmc_rst(&self) -> FMC_RST_R {
        FMC_RST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - EEPROM reset control."]
    #[inline(always)]
    pub fn eeprom_rst(&self) -> EEPROM_RST_R {
        EEPROM_RST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - SPIFI reset control."]
    #[inline(always)]
    pub fn spifi_rst(&self) -> SPIFI_RST_R {
        SPIFI_RST_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Input mux reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn mux_rst(&self) -> MUX_RST_R {
        MUX_RST_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 13 - IOCON reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn iocon_rst(&self) -> IOCON_RST_R {
        IOCON_RST_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - GPIO0 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn gpio0_rst(&self) -> GPIO0_RST_R {
        GPIO0_RST_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - GPIO1 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn gpio1_rst(&self) -> GPIO1_RST_R {
        GPIO1_RST_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - GPIO2 reset control."]
    #[inline(always)]
    pub fn gpio2_rst(&self) -> GPIO2_RST_R {
        GPIO2_RST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - GPIO3 reset control."]
    #[inline(always)]
    pub fn gpio3_rst(&self) -> GPIO3_RST_R {
        GPIO3_RST_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Pin interrupt (PINT) reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn pint_rst(&self) -> PINT_RST_R {
        PINT_RST_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Grouped interrupt (GINT) reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn gint_rst(&self) -> GINT_RST_R {
        GINT_RST_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - DMA0 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn dma0_rst(&self) -> DMA0_RST_R {
        DMA0_RST_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - CRC generator reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn crc_rst(&self) -> CRC_RST_R {
        CRC_RST_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Watchdog timer reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn wwdt_rst(&self) -> WWDT_RST_R {
        WWDT_RST_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 27 - ADC0 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn adc0_rst(&self) -> ADC0_RST_R {
        ADC0_RST_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Flash controller reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn flash_rst(&mut self) -> FLASH_RST_W {
        FLASH_RST_W { w: self }
    }
    #[doc = "Bit 8 - Flash accelerator reset control. Note that the FMC must not be reset while executing from flash, and must be reconfigured after reset. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn fmc_rst(&mut self) -> FMC_RST_W {
        FMC_RST_W { w: self }
    }
    #[doc = "Bit 9 - EEPROM reset control."]
    #[inline(always)]
    pub fn eeprom_rst(&mut self) -> EEPROM_RST_W {
        EEPROM_RST_W { w: self }
    }
    #[doc = "Bit 10 - SPIFI reset control."]
    #[inline(always)]
    pub fn spifi_rst(&mut self) -> SPIFI_RST_W {
        SPIFI_RST_W { w: self }
    }
    #[doc = "Bit 11 - Input mux reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn mux_rst(&mut self) -> MUX_RST_W {
        MUX_RST_W { w: self }
    }
    #[doc = "Bit 13 - IOCON reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn iocon_rst(&mut self) -> IOCON_RST_W {
        IOCON_RST_W { w: self }
    }
    #[doc = "Bit 14 - GPIO0 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn gpio0_rst(&mut self) -> GPIO0_RST_W {
        GPIO0_RST_W { w: self }
    }
    #[doc = "Bit 15 - GPIO1 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn gpio1_rst(&mut self) -> GPIO1_RST_W {
        GPIO1_RST_W { w: self }
    }
    #[doc = "Bit 16 - GPIO2 reset control."]
    #[inline(always)]
    pub fn gpio2_rst(&mut self) -> GPIO2_RST_W {
        GPIO2_RST_W { w: self }
    }
    #[doc = "Bit 17 - GPIO3 reset control."]
    #[inline(always)]
    pub fn gpio3_rst(&mut self) -> GPIO3_RST_W {
        GPIO3_RST_W { w: self }
    }
    #[doc = "Bit 18 - Pin interrupt (PINT) reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn pint_rst(&mut self) -> PINT_RST_W {
        PINT_RST_W { w: self }
    }
    #[doc = "Bit 19 - Grouped interrupt (GINT) reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn gint_rst(&mut self) -> GINT_RST_W {
        GINT_RST_W { w: self }
    }
    #[doc = "Bit 20 - DMA0 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn dma0_rst(&mut self) -> DMA0_RST_W {
        DMA0_RST_W { w: self }
    }
    #[doc = "Bit 21 - CRC generator reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn crc_rst(&mut self) -> CRC_RST_W {
        CRC_RST_W { w: self }
    }
    #[doc = "Bit 22 - Watchdog timer reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn wwdt_rst(&mut self) -> WWDT_RST_W {
        WWDT_RST_W { w: self }
    }
    #[doc = "Bit 27 - ADC0 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn adc0_rst(&mut self) -> ADC0_RST_W {
        ADC0_RST_W { w: self }
    }
}
