///Register `PDRUNCFG0` reader
pub struct R(crate::R<PDRUNCFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDRUNCFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDRUNCFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDRUNCFG0_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PDRUNCFG0` writer
pub struct W(crate::W<PDRUNCFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDRUNCFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<PDRUNCFG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDRUNCFG0_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PDEN_FRO` reader - FRO oscillator.
pub struct PDEN_FRO_R(crate::FieldReader<bool, bool>);
impl PDEN_FRO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDEN_FRO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDEN_FRO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PDEN_FRO` writer - FRO oscillator.
pub struct PDEN_FRO_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_FRO_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
///Field `PDEN_TS` reader - Temp sensor.
pub struct PDEN_TS_R(crate::FieldReader<bool, bool>);
impl PDEN_TS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDEN_TS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDEN_TS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PDEN_TS` writer - Temp sensor.
pub struct PDEN_TS_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_TS_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
///Field `PDEN_BOD_RST` reader - Brown-out Detect reset.
pub struct PDEN_BOD_RST_R(crate::FieldReader<bool, bool>);
impl PDEN_BOD_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDEN_BOD_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDEN_BOD_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PDEN_BOD_RST` writer - Brown-out Detect reset.
pub struct PDEN_BOD_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_BOD_RST_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
///Field `PDEN_BOD_INTR` reader - Brown-out Detect interrupt.
pub struct PDEN_BOD_INTR_R(crate::FieldReader<bool, bool>);
impl PDEN_BOD_INTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDEN_BOD_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDEN_BOD_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PDEN_BOD_INTR` writer - Brown-out Detect interrupt.
pub struct PDEN_BOD_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_BOD_INTR_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
///Field `PDEN_VD2_ANA` reader - Analog supply for System Oscillator (also enable/disable bit 3 in PDRUNCFG1 register), Temperature Sensor (also, enable/disable bit 6), ADC (also, enable/disable bits 10, 19, and 23).
pub struct PDEN_VD2_ANA_R(crate::FieldReader<bool, bool>);
impl PDEN_VD2_ANA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDEN_VD2_ANA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDEN_VD2_ANA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PDEN_VD2_ANA` writer - Analog supply for System Oscillator (also enable/disable bit 3 in PDRUNCFG1 register), Temperature Sensor (also, enable/disable bit 6), ADC (also, enable/disable bits 10, 19, and 23).
pub struct PDEN_VD2_ANA_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_VD2_ANA_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
///Field `PDEN_ADC0` reader - ADC power.
pub struct PDEN_ADC0_R(crate::FieldReader<bool, bool>);
impl PDEN_ADC0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDEN_ADC0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDEN_ADC0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PDEN_ADC0` writer - ADC power.
pub struct PDEN_ADC0_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_ADC0_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
///Field `PDEN_SRAMX` reader - PDEN_SRAMX controls SRAMX (also enable/disable bit 27).
pub struct PDEN_SRAMX_R(crate::FieldReader<bool, bool>);
impl PDEN_SRAMX_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDEN_SRAMX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDEN_SRAMX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PDEN_SRAMX` writer - PDEN_SRAMX controls SRAMX (also enable/disable bit 27).
pub struct PDEN_SRAMX_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_SRAMX_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
///Field `PDEN_SRAM0` reader - PDEN_SRAM0 controls SRAM0 (also enable/disable bit 27).
pub struct PDEN_SRAM0_R(crate::FieldReader<bool, bool>);
impl PDEN_SRAM0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDEN_SRAM0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDEN_SRAM0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PDEN_SRAM0` writer - PDEN_SRAM0 controls SRAM0 (also enable/disable bit 27).
pub struct PDEN_SRAM0_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_SRAM0_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
///Field `PDEN_SRAM1_2_3` reader - PDEN_SRAM1_2_3 controls SRAM1, SRAM2, and SRAM3 (also enable/disable bit 27).
pub struct PDEN_SRAM1_2_3_R(crate::FieldReader<bool, bool>);
impl PDEN_SRAM1_2_3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDEN_SRAM1_2_3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDEN_SRAM1_2_3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PDEN_SRAM1_2_3` writer - PDEN_SRAM1_2_3 controls SRAM1, SRAM2, and SRAM3 (also enable/disable bit 27).
pub struct PDEN_SRAM1_2_3_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_SRAM1_2_3_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
///Field `PDEN_USB_RAM` reader - PDEN_USB_SRAM controls USB_RAM (also enable/disable bit 27).
pub struct PDEN_USB_RAM_R(crate::FieldReader<bool, bool>);
impl PDEN_USB_RAM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDEN_USB_RAM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDEN_USB_RAM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PDEN_USB_RAM` writer - PDEN_USB_SRAM controls USB_RAM (also enable/disable bit 27).
pub struct PDEN_USB_RAM_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_USB_RAM_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
///Field `PDEN_ROM` reader - ROM (also enable/disable bit 27).
pub struct PDEN_ROM_R(crate::FieldReader<bool, bool>);
impl PDEN_ROM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDEN_ROM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDEN_ROM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PDEN_ROM` writer - ROM (also enable/disable bit 27).
pub struct PDEN_ROM_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_ROM_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
///Field `PDEN_VDDA` reader - Vdda to the ADC, must be enabled for the ADC to work (also enable/disable bit 9, 10, and 23).
pub struct PDEN_VDDA_R(crate::FieldReader<bool, bool>);
impl PDEN_VDDA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDEN_VDDA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDEN_VDDA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PDEN_VDDA` writer - Vdda to the ADC, must be enabled for the ADC to work (also enable/disable bit 9, 10, and 23).
pub struct PDEN_VDDA_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_VDDA_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
///Field `PDEN_WDT_OSC` reader - Watchdog oscillator.
pub struct PDEN_WDT_OSC_R(crate::FieldReader<bool, bool>);
impl PDEN_WDT_OSC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDEN_WDT_OSC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDEN_WDT_OSC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PDEN_WDT_OSC` writer - Watchdog oscillator.
pub struct PDEN_WDT_OSC_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_WDT_OSC_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
///Field `PDEN_USB0_PHY` reader - USB0 PHY power (also enable/disable bit 28).
pub struct PDEN_USB0_PHY_R(crate::FieldReader<bool, bool>);
impl PDEN_USB0_PHY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDEN_USB0_PHY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDEN_USB0_PHY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PDEN_USB0_PHY` writer - USB0 PHY power (also enable/disable bit 28).
pub struct PDEN_USB0_PHY_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_USB0_PHY_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
///Field `PDEN_SYS_PLL` reader - System PLL (PLL0) power (also enable/disable bit 26).
pub struct PDEN_SYS_PLL_R(crate::FieldReader<bool, bool>);
impl PDEN_SYS_PLL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDEN_SYS_PLL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDEN_SYS_PLL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PDEN_SYS_PLL` writer - System PLL (PLL0) power (also enable/disable bit 26).
pub struct PDEN_SYS_PLL_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_SYS_PLL_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
///Field `PDEN_VREFP` reader - VREFP to the ADC must be enabled for the ADC to work (also enable/disable bit 9, 10, and 19).
pub struct PDEN_VREFP_R(crate::FieldReader<bool, bool>);
impl PDEN_VREFP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDEN_VREFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDEN_VREFP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PDEN_VREFP` writer - VREFP to the ADC must be enabled for the ADC to work (also enable/disable bit 9, 10, and 19).
pub struct PDEN_VREFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_VREFP_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
///Field `PDEN_VD3` reader - Power control for all PLLs.
pub struct PDEN_VD3_R(crate::FieldReader<bool, bool>);
impl PDEN_VD3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDEN_VD3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDEN_VD3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PDEN_VD3` writer - Power control for all PLLs.
pub struct PDEN_VD3_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_VD3_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
///Field `PDEN_VD4` reader - Power control for all SRAMs and ROM.
pub struct PDEN_VD4_R(crate::FieldReader<bool, bool>);
impl PDEN_VD4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDEN_VD4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDEN_VD4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PDEN_VD4` writer - Power control for all SRAMs and ROM.
pub struct PDEN_VD4_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_VD4_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
///Field `PDEN_VD5` reader - Power control both USB0 PHY and USB1 PHY.
pub struct PDEN_VD5_R(crate::FieldReader<bool, bool>);
impl PDEN_VD5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDEN_VD5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDEN_VD5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PDEN_VD5` writer - Power control both USB0 PHY and USB1 PHY.
pub struct PDEN_VD5_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_VD5_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
///Field `PDEN_VD6` reader - Power control for EEPROM.
pub struct PDEN_VD6_R(crate::FieldReader<bool, bool>);
impl PDEN_VD6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDEN_VD6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDEN_VD6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PDEN_VD6` writer - Power control for EEPROM.
pub struct PDEN_VD6_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_VD6_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
impl R {
    ///Bit 4 - FRO oscillator.
    #[inline(always)]
    pub fn pden_fro(&self) -> PDEN_FRO_R {
        PDEN_FRO_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 6 - Temp sensor.
    #[inline(always)]
    pub fn pden_ts(&self) -> PDEN_TS_R {
        PDEN_TS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 7 - Brown-out Detect reset.
    #[inline(always)]
    pub fn pden_bod_rst(&self) -> PDEN_BOD_RST_R {
        PDEN_BOD_RST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 8 - Brown-out Detect interrupt.
    #[inline(always)]
    pub fn pden_bod_intr(&self) -> PDEN_BOD_INTR_R {
        PDEN_BOD_INTR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 9 - Analog supply for System Oscillator (also enable/disable bit 3 in PDRUNCFG1 register), Temperature Sensor (also, enable/disable bit 6), ADC (also, enable/disable bits 10, 19, and 23).
    #[inline(always)]
    pub fn pden_vd2_ana(&self) -> PDEN_VD2_ANA_R {
        PDEN_VD2_ANA_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 10 - ADC power.
    #[inline(always)]
    pub fn pden_adc0(&self) -> PDEN_ADC0_R {
        PDEN_ADC0_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 13 - PDEN_SRAMX controls SRAMX (also enable/disable bit 27).
    #[inline(always)]
    pub fn pden_sramx(&self) -> PDEN_SRAMX_R {
        PDEN_SRAMX_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 14 - PDEN_SRAM0 controls SRAM0 (also enable/disable bit 27).
    #[inline(always)]
    pub fn pden_sram0(&self) -> PDEN_SRAM0_R {
        PDEN_SRAM0_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 15 - PDEN_SRAM1_2_3 controls SRAM1, SRAM2, and SRAM3 (also enable/disable bit 27).
    #[inline(always)]
    pub fn pden_sram1_2_3(&self) -> PDEN_SRAM1_2_3_R {
        PDEN_SRAM1_2_3_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 16 - PDEN_USB_SRAM controls USB_RAM (also enable/disable bit 27).
    #[inline(always)]
    pub fn pden_usb_ram(&self) -> PDEN_USB_RAM_R {
        PDEN_USB_RAM_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 17 - ROM (also enable/disable bit 27).
    #[inline(always)]
    pub fn pden_rom(&self) -> PDEN_ROM_R {
        PDEN_ROM_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    ///Bit 19 - Vdda to the ADC, must be enabled for the ADC to work (also enable/disable bit 9, 10, and 23).
    #[inline(always)]
    pub fn pden_vdda(&self) -> PDEN_VDDA_R {
        PDEN_VDDA_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    ///Bit 20 - Watchdog oscillator.
    #[inline(always)]
    pub fn pden_wdt_osc(&self) -> PDEN_WDT_OSC_R {
        PDEN_WDT_OSC_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    ///Bit 21 - USB0 PHY power (also enable/disable bit 28).
    #[inline(always)]
    pub fn pden_usb0_phy(&self) -> PDEN_USB0_PHY_R {
        PDEN_USB0_PHY_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    ///Bit 22 - System PLL (PLL0) power (also enable/disable bit 26).
    #[inline(always)]
    pub fn pden_sys_pll(&self) -> PDEN_SYS_PLL_R {
        PDEN_SYS_PLL_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    ///Bit 23 - VREFP to the ADC must be enabled for the ADC to work (also enable/disable bit 9, 10, and 19).
    #[inline(always)]
    pub fn pden_vrefp(&self) -> PDEN_VREFP_R {
        PDEN_VREFP_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    ///Bit 26 - Power control for all PLLs.
    #[inline(always)]
    pub fn pden_vd3(&self) -> PDEN_VD3_R {
        PDEN_VD3_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    ///Bit 27 - Power control for all SRAMs and ROM.
    #[inline(always)]
    pub fn pden_vd4(&self) -> PDEN_VD4_R {
        PDEN_VD4_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    ///Bit 28 - Power control both USB0 PHY and USB1 PHY.
    #[inline(always)]
    pub fn pden_vd5(&self) -> PDEN_VD5_R {
        PDEN_VD5_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    ///Bit 29 - Power control for EEPROM.
    #[inline(always)]
    pub fn pden_vd6(&self) -> PDEN_VD6_R {
        PDEN_VD6_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    ///Bit 4 - FRO oscillator.
    #[inline(always)]
    pub fn pden_fro(&mut self) -> PDEN_FRO_W {
        PDEN_FRO_W { w: self }
    }
    ///Bit 6 - Temp sensor.
    #[inline(always)]
    pub fn pden_ts(&mut self) -> PDEN_TS_W {
        PDEN_TS_W { w: self }
    }
    ///Bit 7 - Brown-out Detect reset.
    #[inline(always)]
    pub fn pden_bod_rst(&mut self) -> PDEN_BOD_RST_W {
        PDEN_BOD_RST_W { w: self }
    }
    ///Bit 8 - Brown-out Detect interrupt.
    #[inline(always)]
    pub fn pden_bod_intr(&mut self) -> PDEN_BOD_INTR_W {
        PDEN_BOD_INTR_W { w: self }
    }
    ///Bit 9 - Analog supply for System Oscillator (also enable/disable bit 3 in PDRUNCFG1 register), Temperature Sensor (also, enable/disable bit 6), ADC (also, enable/disable bits 10, 19, and 23).
    #[inline(always)]
    pub fn pden_vd2_ana(&mut self) -> PDEN_VD2_ANA_W {
        PDEN_VD2_ANA_W { w: self }
    }
    ///Bit 10 - ADC power.
    #[inline(always)]
    pub fn pden_adc0(&mut self) -> PDEN_ADC0_W {
        PDEN_ADC0_W { w: self }
    }
    ///Bit 13 - PDEN_SRAMX controls SRAMX (also enable/disable bit 27).
    #[inline(always)]
    pub fn pden_sramx(&mut self) -> PDEN_SRAMX_W {
        PDEN_SRAMX_W { w: self }
    }
    ///Bit 14 - PDEN_SRAM0 controls SRAM0 (also enable/disable bit 27).
    #[inline(always)]
    pub fn pden_sram0(&mut self) -> PDEN_SRAM0_W {
        PDEN_SRAM0_W { w: self }
    }
    ///Bit 15 - PDEN_SRAM1_2_3 controls SRAM1, SRAM2, and SRAM3 (also enable/disable bit 27).
    #[inline(always)]
    pub fn pden_sram1_2_3(&mut self) -> PDEN_SRAM1_2_3_W {
        PDEN_SRAM1_2_3_W { w: self }
    }
    ///Bit 16 - PDEN_USB_SRAM controls USB_RAM (also enable/disable bit 27).
    #[inline(always)]
    pub fn pden_usb_ram(&mut self) -> PDEN_USB_RAM_W {
        PDEN_USB_RAM_W { w: self }
    }
    ///Bit 17 - ROM (also enable/disable bit 27).
    #[inline(always)]
    pub fn pden_rom(&mut self) -> PDEN_ROM_W {
        PDEN_ROM_W { w: self }
    }
    ///Bit 19 - Vdda to the ADC, must be enabled for the ADC to work (also enable/disable bit 9, 10, and 23).
    #[inline(always)]
    pub fn pden_vdda(&mut self) -> PDEN_VDDA_W {
        PDEN_VDDA_W { w: self }
    }
    ///Bit 20 - Watchdog oscillator.
    #[inline(always)]
    pub fn pden_wdt_osc(&mut self) -> PDEN_WDT_OSC_W {
        PDEN_WDT_OSC_W { w: self }
    }
    ///Bit 21 - USB0 PHY power (also enable/disable bit 28).
    #[inline(always)]
    pub fn pden_usb0_phy(&mut self) -> PDEN_USB0_PHY_W {
        PDEN_USB0_PHY_W { w: self }
    }
    ///Bit 22 - System PLL (PLL0) power (also enable/disable bit 26).
    #[inline(always)]
    pub fn pden_sys_pll(&mut self) -> PDEN_SYS_PLL_W {
        PDEN_SYS_PLL_W { w: self }
    }
    ///Bit 23 - VREFP to the ADC must be enabled for the ADC to work (also enable/disable bit 9, 10, and 19).
    #[inline(always)]
    pub fn pden_vrefp(&mut self) -> PDEN_VREFP_W {
        PDEN_VREFP_W { w: self }
    }
    ///Bit 26 - Power control for all PLLs.
    #[inline(always)]
    pub fn pden_vd3(&mut self) -> PDEN_VD3_W {
        PDEN_VD3_W { w: self }
    }
    ///Bit 27 - Power control for all SRAMs and ROM.
    #[inline(always)]
    pub fn pden_vd4(&mut self) -> PDEN_VD4_W {
        PDEN_VD4_W { w: self }
    }
    ///Bit 28 - Power control both USB0 PHY and USB1 PHY.
    #[inline(always)]
    pub fn pden_vd5(&mut self) -> PDEN_VD5_W {
        PDEN_VD5_W { w: self }
    }
    ///Bit 29 - Power control for EEPROM.
    #[inline(always)]
    pub fn pden_vd6(&mut self) -> PDEN_VD6_W {
        PDEN_VD6_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Power configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pdruncfg0](index.html) module
pub struct PDRUNCFG0_SPEC;
impl crate::RegisterSpec for PDRUNCFG0_SPEC {
    type Ux = u32;
}
///`read()` method returns [pdruncfg0::R](R) reader structure
impl crate::Readable for PDRUNCFG0_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [pdruncfg0::W](W) writer structure
impl crate::Writable for PDRUNCFG0_SPEC {
    type Writer = W;
}
///`reset()` method sets PDRUNCFG0 to value 0x14f8_1f40
impl crate::Resettable for PDRUNCFG0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x14f8_1f40
    }
}
