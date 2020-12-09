#[doc = "Reader of register PDRUNCFG0"]
pub type R = crate::R<u32, super::PDRUNCFG0>;
#[doc = "Writer for register PDRUNCFG0"]
pub type W = crate::W<u32, super::PDRUNCFG0>;
#[doc = "Register PDRUNCFG0 `reset()`'s with value 0x14f8_1f40"]
impl crate::ResetValue for super::PDRUNCFG0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x14f8_1f40
    }
}
#[doc = "Reader of field `PDEN_FRO`"]
pub type PDEN_FRO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PDEN_FRO`"]
pub struct PDEN_FRO_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_FRO_W<'a> {
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
#[doc = "Reader of field `PDEN_TS`"]
pub type PDEN_TS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PDEN_TS`"]
pub struct PDEN_TS_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_TS_W<'a> {
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
#[doc = "Reader of field `PDEN_BOD_RST`"]
pub type PDEN_BOD_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PDEN_BOD_RST`"]
pub struct PDEN_BOD_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_BOD_RST_W<'a> {
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
#[doc = "Reader of field `PDEN_BOD_INTR`"]
pub type PDEN_BOD_INTR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PDEN_BOD_INTR`"]
pub struct PDEN_BOD_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_BOD_INTR_W<'a> {
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
#[doc = "Reader of field `PDEN_VD2_ANA`"]
pub type PDEN_VD2_ANA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PDEN_VD2_ANA`"]
pub struct PDEN_VD2_ANA_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_VD2_ANA_W<'a> {
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
#[doc = "Reader of field `PDEN_ADC0`"]
pub type PDEN_ADC0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PDEN_ADC0`"]
pub struct PDEN_ADC0_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_ADC0_W<'a> {
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
#[doc = "Reader of field `PDEN_SRAMX`"]
pub type PDEN_SRAMX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PDEN_SRAMX`"]
pub struct PDEN_SRAMX_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_SRAMX_W<'a> {
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
#[doc = "Reader of field `PDEN_SRAM0`"]
pub type PDEN_SRAM0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PDEN_SRAM0`"]
pub struct PDEN_SRAM0_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_SRAM0_W<'a> {
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
#[doc = "Reader of field `PDEN_SRAM1_2_3`"]
pub type PDEN_SRAM1_2_3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PDEN_SRAM1_2_3`"]
pub struct PDEN_SRAM1_2_3_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_SRAM1_2_3_W<'a> {
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
#[doc = "Reader of field `PDEN_USB_RAM`"]
pub type PDEN_USB_RAM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PDEN_USB_RAM`"]
pub struct PDEN_USB_RAM_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_USB_RAM_W<'a> {
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
#[doc = "Reader of field `PDEN_ROM`"]
pub type PDEN_ROM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PDEN_ROM`"]
pub struct PDEN_ROM_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_ROM_W<'a> {
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
#[doc = "Reader of field `PDEN_VDDA`"]
pub type PDEN_VDDA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PDEN_VDDA`"]
pub struct PDEN_VDDA_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_VDDA_W<'a> {
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
#[doc = "Reader of field `PDEN_WDT_OSC`"]
pub type PDEN_WDT_OSC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PDEN_WDT_OSC`"]
pub struct PDEN_WDT_OSC_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_WDT_OSC_W<'a> {
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
#[doc = "Reader of field `PDEN_USB0_PHY`"]
pub type PDEN_USB0_PHY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PDEN_USB0_PHY`"]
pub struct PDEN_USB0_PHY_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_USB0_PHY_W<'a> {
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
#[doc = "Reader of field `PDEN_SYS_PLL`"]
pub type PDEN_SYS_PLL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PDEN_SYS_PLL`"]
pub struct PDEN_SYS_PLL_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_SYS_PLL_W<'a> {
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
#[doc = "Reader of field `PDEN_VREFP`"]
pub type PDEN_VREFP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PDEN_VREFP`"]
pub struct PDEN_VREFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_VREFP_W<'a> {
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
#[doc = "Reader of field `PDEN_VD3`"]
pub type PDEN_VD3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PDEN_VD3`"]
pub struct PDEN_VD3_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_VD3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `PDEN_VD4`"]
pub type PDEN_VD4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PDEN_VD4`"]
pub struct PDEN_VD4_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_VD4_W<'a> {
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
#[doc = "Reader of field `PDEN_VD5`"]
pub type PDEN_VD5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PDEN_VD5`"]
pub struct PDEN_VD5_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_VD5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `PDEN_VD6`"]
pub type PDEN_VD6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PDEN_VD6`"]
pub struct PDEN_VD6_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_VD6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bit 4 - FRO oscillator."]
    #[inline(always)]
    pub fn pden_fro(&self) -> PDEN_FRO_R {
        PDEN_FRO_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Temp sensor."]
    #[inline(always)]
    pub fn pden_ts(&self) -> PDEN_TS_R {
        PDEN_TS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Brown-out Detect reset."]
    #[inline(always)]
    pub fn pden_bod_rst(&self) -> PDEN_BOD_RST_R {
        PDEN_BOD_RST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Brown-out Detect interrupt."]
    #[inline(always)]
    pub fn pden_bod_intr(&self) -> PDEN_BOD_INTR_R {
        PDEN_BOD_INTR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Analog supply for System Oscillator (also enable/disable bit 3 in PDRUNCFG1 register), Temperature Sensor (also, enable/disable bit 6), ADC (also, enable/disable bits 10, 19, and 23)."]
    #[inline(always)]
    pub fn pden_vd2_ana(&self) -> PDEN_VD2_ANA_R {
        PDEN_VD2_ANA_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - ADC power."]
    #[inline(always)]
    pub fn pden_adc0(&self) -> PDEN_ADC0_R {
        PDEN_ADC0_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 13 - PDEN_SRAMX controls SRAMX (also enable/disable bit 27)."]
    #[inline(always)]
    pub fn pden_sramx(&self) -> PDEN_SRAMX_R {
        PDEN_SRAMX_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - PDEN_SRAM0 controls SRAM0 (also enable/disable bit 27)."]
    #[inline(always)]
    pub fn pden_sram0(&self) -> PDEN_SRAM0_R {
        PDEN_SRAM0_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - PDEN_SRAM1_2_3 controls SRAM1, SRAM2, and SRAM3 (also enable/disable bit 27)."]
    #[inline(always)]
    pub fn pden_sram1_2_3(&self) -> PDEN_SRAM1_2_3_R {
        PDEN_SRAM1_2_3_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - PDEN_USB_SRAM controls USB_RAM (also enable/disable bit 27)."]
    #[inline(always)]
    pub fn pden_usb_ram(&self) -> PDEN_USB_RAM_R {
        PDEN_USB_RAM_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - ROM (also enable/disable bit 27)."]
    #[inline(always)]
    pub fn pden_rom(&self) -> PDEN_ROM_R {
        PDEN_ROM_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Vdda to the ADC, must be enabled for the ADC to work (also enable/disable bit 9, 10, and 23)."]
    #[inline(always)]
    pub fn pden_vdda(&self) -> PDEN_VDDA_R {
        PDEN_VDDA_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Watchdog oscillator."]
    #[inline(always)]
    pub fn pden_wdt_osc(&self) -> PDEN_WDT_OSC_R {
        PDEN_WDT_OSC_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - USB0 PHY power (also enable/disable bit 28)."]
    #[inline(always)]
    pub fn pden_usb0_phy(&self) -> PDEN_USB0_PHY_R {
        PDEN_USB0_PHY_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - System PLL (PLL0) power (also enable/disable bit 26)."]
    #[inline(always)]
    pub fn pden_sys_pll(&self) -> PDEN_SYS_PLL_R {
        PDEN_SYS_PLL_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - VREFP to the ADC must be enabled for the ADC to work (also enable/disable bit 9, 10, and 19)."]
    #[inline(always)]
    pub fn pden_vrefp(&self) -> PDEN_VREFP_R {
        PDEN_VREFP_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Power control for all PLLs."]
    #[inline(always)]
    pub fn pden_vd3(&self) -> PDEN_VD3_R {
        PDEN_VD3_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Power control for all SRAMs and ROM."]
    #[inline(always)]
    pub fn pden_vd4(&self) -> PDEN_VD4_R {
        PDEN_VD4_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Power control both USB0 PHY and USB1 PHY."]
    #[inline(always)]
    pub fn pden_vd5(&self) -> PDEN_VD5_R {
        PDEN_VD5_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Power control for EEPROM."]
    #[inline(always)]
    pub fn pden_vd6(&self) -> PDEN_VD6_R {
        PDEN_VD6_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - FRO oscillator."]
    #[inline(always)]
    pub fn pden_fro(&mut self) -> PDEN_FRO_W {
        PDEN_FRO_W { w: self }
    }
    #[doc = "Bit 6 - Temp sensor."]
    #[inline(always)]
    pub fn pden_ts(&mut self) -> PDEN_TS_W {
        PDEN_TS_W { w: self }
    }
    #[doc = "Bit 7 - Brown-out Detect reset."]
    #[inline(always)]
    pub fn pden_bod_rst(&mut self) -> PDEN_BOD_RST_W {
        PDEN_BOD_RST_W { w: self }
    }
    #[doc = "Bit 8 - Brown-out Detect interrupt."]
    #[inline(always)]
    pub fn pden_bod_intr(&mut self) -> PDEN_BOD_INTR_W {
        PDEN_BOD_INTR_W { w: self }
    }
    #[doc = "Bit 9 - Analog supply for System Oscillator (also enable/disable bit 3 in PDRUNCFG1 register), Temperature Sensor (also, enable/disable bit 6), ADC (also, enable/disable bits 10, 19, and 23)."]
    #[inline(always)]
    pub fn pden_vd2_ana(&mut self) -> PDEN_VD2_ANA_W {
        PDEN_VD2_ANA_W { w: self }
    }
    #[doc = "Bit 10 - ADC power."]
    #[inline(always)]
    pub fn pden_adc0(&mut self) -> PDEN_ADC0_W {
        PDEN_ADC0_W { w: self }
    }
    #[doc = "Bit 13 - PDEN_SRAMX controls SRAMX (also enable/disable bit 27)."]
    #[inline(always)]
    pub fn pden_sramx(&mut self) -> PDEN_SRAMX_W {
        PDEN_SRAMX_W { w: self }
    }
    #[doc = "Bit 14 - PDEN_SRAM0 controls SRAM0 (also enable/disable bit 27)."]
    #[inline(always)]
    pub fn pden_sram0(&mut self) -> PDEN_SRAM0_W {
        PDEN_SRAM0_W { w: self }
    }
    #[doc = "Bit 15 - PDEN_SRAM1_2_3 controls SRAM1, SRAM2, and SRAM3 (also enable/disable bit 27)."]
    #[inline(always)]
    pub fn pden_sram1_2_3(&mut self) -> PDEN_SRAM1_2_3_W {
        PDEN_SRAM1_2_3_W { w: self }
    }
    #[doc = "Bit 16 - PDEN_USB_SRAM controls USB_RAM (also enable/disable bit 27)."]
    #[inline(always)]
    pub fn pden_usb_ram(&mut self) -> PDEN_USB_RAM_W {
        PDEN_USB_RAM_W { w: self }
    }
    #[doc = "Bit 17 - ROM (also enable/disable bit 27)."]
    #[inline(always)]
    pub fn pden_rom(&mut self) -> PDEN_ROM_W {
        PDEN_ROM_W { w: self }
    }
    #[doc = "Bit 19 - Vdda to the ADC, must be enabled for the ADC to work (also enable/disable bit 9, 10, and 23)."]
    #[inline(always)]
    pub fn pden_vdda(&mut self) -> PDEN_VDDA_W {
        PDEN_VDDA_W { w: self }
    }
    #[doc = "Bit 20 - Watchdog oscillator."]
    #[inline(always)]
    pub fn pden_wdt_osc(&mut self) -> PDEN_WDT_OSC_W {
        PDEN_WDT_OSC_W { w: self }
    }
    #[doc = "Bit 21 - USB0 PHY power (also enable/disable bit 28)."]
    #[inline(always)]
    pub fn pden_usb0_phy(&mut self) -> PDEN_USB0_PHY_W {
        PDEN_USB0_PHY_W { w: self }
    }
    #[doc = "Bit 22 - System PLL (PLL0) power (also enable/disable bit 26)."]
    #[inline(always)]
    pub fn pden_sys_pll(&mut self) -> PDEN_SYS_PLL_W {
        PDEN_SYS_PLL_W { w: self }
    }
    #[doc = "Bit 23 - VREFP to the ADC must be enabled for the ADC to work (also enable/disable bit 9, 10, and 19)."]
    #[inline(always)]
    pub fn pden_vrefp(&mut self) -> PDEN_VREFP_W {
        PDEN_VREFP_W { w: self }
    }
    #[doc = "Bit 26 - Power control for all PLLs."]
    #[inline(always)]
    pub fn pden_vd3(&mut self) -> PDEN_VD3_W {
        PDEN_VD3_W { w: self }
    }
    #[doc = "Bit 27 - Power control for all SRAMs and ROM."]
    #[inline(always)]
    pub fn pden_vd4(&mut self) -> PDEN_VD4_W {
        PDEN_VD4_W { w: self }
    }
    #[doc = "Bit 28 - Power control both USB0 PHY and USB1 PHY."]
    #[inline(always)]
    pub fn pden_vd5(&mut self) -> PDEN_VD5_W {
        PDEN_VD5_W { w: self }
    }
    #[doc = "Bit 29 - Power control for EEPROM."]
    #[inline(always)]
    pub fn pden_vd6(&mut self) -> PDEN_VD6_W {
        PDEN_VD6_W { w: self }
    }
}
