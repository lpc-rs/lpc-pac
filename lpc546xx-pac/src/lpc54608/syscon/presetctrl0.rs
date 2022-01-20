#[doc = "Register `PRESETCTRL0` reader"]
pub struct R(crate::R<PRESETCTRL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRESETCTRL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRESETCTRL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRESETCTRL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRESETCTRL0` writer"]
pub struct W(crate::W<PRESETCTRL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRESETCTRL0_SPEC>;
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
impl From<crate::W<PRESETCTRL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRESETCTRL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLASH_RST` reader - Flash controller reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub struct FLASH_RST_R(crate::FieldReader<bool, bool>);
impl FLASH_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FLASH_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASH_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASH_RST` writer - Flash controller reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `FMC_RST` reader - Flash accelerator reset control. Note that the FMC must not be reset while executing from flash, and must be reconfigured after reset. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub struct FMC_RST_R(crate::FieldReader<bool, bool>);
impl FMC_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FMC_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FMC_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FMC_RST` writer - Flash accelerator reset control. Note that the FMC must not be reset while executing from flash, and must be reconfigured after reset. 0 = Clear reset to this function. 1 = Assert reset to this function."]
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `EEPROM_RST` reader - EEPROM reset control."]
pub struct EEPROM_RST_R(crate::FieldReader<bool, bool>);
impl EEPROM_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EEPROM_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EEPROM_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EEPROM_RST` writer - EEPROM reset control."]
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `SPIFI_RST` reader - SPIFI reset control."]
pub struct SPIFI_RST_R(crate::FieldReader<bool, bool>);
impl SPIFI_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPIFI_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPIFI_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPIFI_RST` writer - SPIFI reset control."]
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `MUX_RST` reader - Input mux reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub struct MUX_RST_R(crate::FieldReader<bool, bool>);
impl MUX_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MUX_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MUX_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUX_RST` writer - Input mux reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `IOCON_RST` reader - IOCON reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub struct IOCON_RST_R(crate::FieldReader<bool, bool>);
impl IOCON_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IOCON_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IOCON_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IOCON_RST` writer - IOCON reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `GPIO0_RST` reader - GPIO0 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub struct GPIO0_RST_R(crate::FieldReader<bool, bool>);
impl GPIO0_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO0_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO0_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO0_RST` writer - GPIO0 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `GPIO1_RST` reader - GPIO1 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub struct GPIO1_RST_R(crate::FieldReader<bool, bool>);
impl GPIO1_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO1_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO1_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO1_RST` writer - GPIO1 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `GPIO2_RST` reader - GPIO2 reset control."]
pub struct GPIO2_RST_R(crate::FieldReader<bool, bool>);
impl GPIO2_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO2_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO2_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO2_RST` writer - GPIO2 reset control."]
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `GPIO3_RST` reader - GPIO3 reset control."]
pub struct GPIO3_RST_R(crate::FieldReader<bool, bool>);
impl GPIO3_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO3_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO3_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO3_RST` writer - GPIO3 reset control."]
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `PINT_RST` reader - Pin interrupt (PINT) reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub struct PINT_RST_R(crate::FieldReader<bool, bool>);
impl PINT_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PINT_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PINT_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PINT_RST` writer - Pin interrupt (PINT) reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `GINT_RST` reader - Grouped interrupt (GINT) reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub struct GINT_RST_R(crate::FieldReader<bool, bool>);
impl GINT_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GINT_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GINT_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GINT_RST` writer - Grouped interrupt (GINT) reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `DMA0_RST` reader - DMA0 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub struct DMA0_RST_R(crate::FieldReader<bool, bool>);
impl DMA0_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA0_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA0_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA0_RST` writer - DMA0 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `CRC_RST` reader - CRC generator reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub struct CRC_RST_R(crate::FieldReader<bool, bool>);
impl CRC_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CRC_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRC_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRC_RST` writer - CRC generator reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `WWDT_RST` reader - Watchdog timer reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub struct WWDT_RST_R(crate::FieldReader<bool, bool>);
impl WWDT_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WWDT_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WWDT_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WWDT_RST` writer - Watchdog timer reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `ADC0_RST` reader - ADC0 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub struct ADC0_RST_R(crate::FieldReader<bool, bool>);
impl ADC0_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC0_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC0_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC0_RST` writer - ADC0 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral reset control n\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [presetctrl0](index.html) module"]
pub struct PRESETCTRL0_SPEC;
impl crate::RegisterSpec for PRESETCTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [presetctrl0::R](R) reader structure"]
impl crate::Readable for PRESETCTRL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [presetctrl0::W](W) writer structure"]
impl crate::Writable for PRESETCTRL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRESETCTRL0 to value 0"]
impl crate::Resettable for PRESETCTRL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
