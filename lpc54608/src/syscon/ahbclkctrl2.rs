#[doc = "Register `AHBCLKCTRL2` reader"]
pub struct R(crate::R<AHBCLKCTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHBCLKCTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<AHBCLKCTRL2_SPEC>> for R {
    fn from(reader: crate::R<AHBCLKCTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHBCLKCTRL2` writer"]
pub struct W(crate::W<AHBCLKCTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHBCLKCTRL2_SPEC>;
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
impl core::convert::From<crate::W<AHBCLKCTRL2_SPEC>> for W {
    fn from(writer: crate::W<AHBCLKCTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LCD` reader - Enables the clock for the LCD interface."]
pub struct LCD_R(crate::FieldReader<bool, bool>);
impl LCD_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCD` writer - Enables the clock for the LCD interface."]
pub struct LCD_W<'a> {
    w: &'a mut W,
}
impl<'a> LCD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `SDIO` reader - Enables the clock for the SDIO interface."]
pub struct SDIO_R(crate::FieldReader<bool, bool>);
impl SDIO_R {
    pub(crate) fn new(bits: bool) -> Self {
        SDIO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDIO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDIO` writer - Enables the clock for the SDIO interface."]
pub struct SDIO_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `USB1H` reader - Enables the clock for the USB1 host interface."]
pub struct USB1H_R(crate::FieldReader<bool, bool>);
impl USB1H_R {
    pub(crate) fn new(bits: bool) -> Self {
        USB1H_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB1H_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB1H` writer - Enables the clock for the USB1 host interface."]
pub struct USB1H_W<'a> {
    w: &'a mut W,
}
impl<'a> USB1H_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `USB1D` reader - Enables the clock for the USB1 device interface."]
pub struct USB1D_R(crate::FieldReader<bool, bool>);
impl USB1D_R {
    pub(crate) fn new(bits: bool) -> Self {
        USB1D_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB1D_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB1D` writer - Enables the clock for the USB1 device interface."]
pub struct USB1D_W<'a> {
    w: &'a mut W,
}
impl<'a> USB1D_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `USB1RAM` reader - Enables the clock for the USB1 RAM interface."]
pub struct USB1RAM_R(crate::FieldReader<bool, bool>);
impl USB1RAM_R {
    pub(crate) fn new(bits: bool) -> Self {
        USB1RAM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB1RAM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB1RAM` writer - Enables the clock for the USB1 RAM interface."]
pub struct USB1RAM_W<'a> {
    w: &'a mut W,
}
impl<'a> USB1RAM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `EMC` reader - Enables the clock for the EMC interface."]
pub struct EMC_R(crate::FieldReader<bool, bool>);
impl EMC_R {
    pub(crate) fn new(bits: bool) -> Self {
        EMC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EMC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EMC` writer - Enables the clock for the EMC interface."]
pub struct EMC_W<'a> {
    w: &'a mut W,
}
impl<'a> EMC_W<'a> {
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
#[doc = "Field `ETH` reader - Enables the clock for the ethernet interface."]
pub struct ETH_R(crate::FieldReader<bool, bool>);
impl ETH_R {
    pub(crate) fn new(bits: bool) -> Self {
        ETH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ETH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ETH` writer - Enables the clock for the ethernet interface."]
pub struct ETH_W<'a> {
    w: &'a mut W,
}
impl<'a> ETH_W<'a> {
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
#[doc = "Field `GPIO4` reader - Enables the clock for the GPIO4 interface."]
pub struct GPIO4_R(crate::FieldReader<bool, bool>);
impl GPIO4_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO4` writer - Enables the clock for the GPIO4 interface."]
pub struct GPIO4_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO4_W<'a> {
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
#[doc = "Field `GPIO5` reader - Enables the clock for the GPIO5 interface."]
pub struct GPIO5_R(crate::FieldReader<bool, bool>);
impl GPIO5_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO5` writer - Enables the clock for the GPIO5 interface."]
pub struct GPIO5_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO5_W<'a> {
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
#[doc = "Field `AES` reader - Enables the clock for the AES interface."]
pub struct AES_R(crate::FieldReader<bool, bool>);
impl AES_R {
    pub(crate) fn new(bits: bool) -> Self {
        AES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AES_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AES` writer - Enables the clock for the AES interface."]
pub struct AES_W<'a> {
    w: &'a mut W,
}
impl<'a> AES_W<'a> {
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
#[doc = "Field `OTP` reader - Enables the clock for the OTP interface."]
pub struct OTP_R(crate::FieldReader<bool, bool>);
impl OTP_R {
    pub(crate) fn new(bits: bool) -> Self {
        OTP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OTP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OTP` writer - Enables the clock for the OTP interface."]
pub struct OTP_W<'a> {
    w: &'a mut W,
}
impl<'a> OTP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `RNG` reader - Enables the clock for the RNG interface."]
pub struct RNG_R(crate::FieldReader<bool, bool>);
impl RNG_R {
    pub(crate) fn new(bits: bool) -> Self {
        RNG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RNG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RNG` writer - Enables the clock for the RNG interface."]
pub struct RNG_W<'a> {
    w: &'a mut W,
}
impl<'a> RNG_W<'a> {
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
#[doc = "Field `FLEXCOMM8` reader - Enables the clock for the Flexcomm8 interface."]
pub struct FLEXCOMM8_R(crate::FieldReader<bool, bool>);
impl FLEXCOMM8_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLEXCOMM8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLEXCOMM8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLEXCOMM8` writer - Enables the clock for the Flexcomm8 interface."]
pub struct FLEXCOMM8_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXCOMM8_W<'a> {
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
#[doc = "Field `FLEXCOMM9` reader - Enables the clock for the Flexcomm9 interface."]
pub struct FLEXCOMM9_R(crate::FieldReader<bool, bool>);
impl FLEXCOMM9_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLEXCOMM9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLEXCOMM9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLEXCOMM9` writer - Enables the clock for the Flexcomm9 interface."]
pub struct FLEXCOMM9_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXCOMM9_W<'a> {
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
#[doc = "Field `USB0HMR` reader - Enables the clock for the USB host master interface."]
pub struct USB0HMR_R(crate::FieldReader<bool, bool>);
impl USB0HMR_R {
    pub(crate) fn new(bits: bool) -> Self {
        USB0HMR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB0HMR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB0HMR` writer - Enables the clock for the USB host master interface."]
pub struct USB0HMR_W<'a> {
    w: &'a mut W,
}
impl<'a> USB0HMR_W<'a> {
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
#[doc = "Field `USB0HSL` reader - Enables the clock for the USB host slave interface."]
pub struct USB0HSL_R(crate::FieldReader<bool, bool>);
impl USB0HSL_R {
    pub(crate) fn new(bits: bool) -> Self {
        USB0HSL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB0HSL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB0HSL` writer - Enables the clock for the USB host slave interface."]
pub struct USB0HSL_W<'a> {
    w: &'a mut W,
}
impl<'a> USB0HSL_W<'a> {
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
#[doc = "Field `SHA0` reader - Enables the clock for the SHA interface."]
pub struct SHA0_R(crate::FieldReader<bool, bool>);
impl SHA0_R {
    pub(crate) fn new(bits: bool) -> Self {
        SHA0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SHA0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SHA0` writer - Enables the clock for the SHA interface."]
pub struct SHA0_W<'a> {
    w: &'a mut W,
}
impl<'a> SHA0_W<'a> {
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
#[doc = "Field `SC0` reader - Enables the clock for the Smart card0 interface."]
pub struct SC0_R(crate::FieldReader<bool, bool>);
impl SC0_R {
    pub(crate) fn new(bits: bool) -> Self {
        SC0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SC0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SC0` writer - Enables the clock for the Smart card0 interface."]
pub struct SC0_W<'a> {
    w: &'a mut W,
}
impl<'a> SC0_W<'a> {
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
#[doc = "Field `SC1` reader - Enables the clock for the Smart card1 interface."]
pub struct SC1_R(crate::FieldReader<bool, bool>);
impl SC1_R {
    pub(crate) fn new(bits: bool) -> Self {
        SC1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SC1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SC1` writer - Enables the clock for the Smart card1 interface."]
pub struct SC1_W<'a> {
    w: &'a mut W,
}
impl<'a> SC1_W<'a> {
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
impl R {
    #[doc = "Bit 2 - Enables the clock for the LCD interface."]
    #[inline(always)]
    pub fn lcd(&self) -> LCD_R {
        LCD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enables the clock for the SDIO interface."]
    #[inline(always)]
    pub fn sdio(&self) -> SDIO_R {
        SDIO_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enables the clock for the USB1 host interface."]
    #[inline(always)]
    pub fn usb1h(&self) -> USB1H_R {
        USB1H_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enables the clock for the USB1 device interface."]
    #[inline(always)]
    pub fn usb1d(&self) -> USB1D_R {
        USB1D_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enables the clock for the USB1 RAM interface."]
    #[inline(always)]
    pub fn usb1ram(&self) -> USB1RAM_R {
        USB1RAM_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enables the clock for the EMC interface."]
    #[inline(always)]
    pub fn emc(&self) -> EMC_R {
        EMC_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enables the clock for the ethernet interface."]
    #[inline(always)]
    pub fn eth(&self) -> ETH_R {
        ETH_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Enables the clock for the GPIO4 interface."]
    #[inline(always)]
    pub fn gpio4(&self) -> GPIO4_R {
        GPIO4_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Enables the clock for the GPIO5 interface."]
    #[inline(always)]
    pub fn gpio5(&self) -> GPIO5_R {
        GPIO5_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Enables the clock for the AES interface."]
    #[inline(always)]
    pub fn aes(&self) -> AES_R {
        AES_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Enables the clock for the OTP interface."]
    #[inline(always)]
    pub fn otp(&self) -> OTP_R {
        OTP_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enables the clock for the RNG interface."]
    #[inline(always)]
    pub fn rng(&self) -> RNG_R {
        RNG_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Enables the clock for the Flexcomm8 interface."]
    #[inline(always)]
    pub fn flexcomm8(&self) -> FLEXCOMM8_R {
        FLEXCOMM8_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Enables the clock for the Flexcomm9 interface."]
    #[inline(always)]
    pub fn flexcomm9(&self) -> FLEXCOMM9_R {
        FLEXCOMM9_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Enables the clock for the USB host master interface."]
    #[inline(always)]
    pub fn usb0hmr(&self) -> USB0HMR_R {
        USB0HMR_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Enables the clock for the USB host slave interface."]
    #[inline(always)]
    pub fn usb0hsl(&self) -> USB0HSL_R {
        USB0HSL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Enables the clock for the SHA interface."]
    #[inline(always)]
    pub fn sha0(&self) -> SHA0_R {
        SHA0_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Enables the clock for the Smart card0 interface."]
    #[inline(always)]
    pub fn sc0(&self) -> SC0_R {
        SC0_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Enables the clock for the Smart card1 interface."]
    #[inline(always)]
    pub fn sc1(&self) -> SC1_R {
        SC1_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Enables the clock for the LCD interface."]
    #[inline(always)]
    pub fn lcd(&mut self) -> LCD_W {
        LCD_W { w: self }
    }
    #[doc = "Bit 3 - Enables the clock for the SDIO interface."]
    #[inline(always)]
    pub fn sdio(&mut self) -> SDIO_W {
        SDIO_W { w: self }
    }
    #[doc = "Bit 4 - Enables the clock for the USB1 host interface."]
    #[inline(always)]
    pub fn usb1h(&mut self) -> USB1H_W {
        USB1H_W { w: self }
    }
    #[doc = "Bit 5 - Enables the clock for the USB1 device interface."]
    #[inline(always)]
    pub fn usb1d(&mut self) -> USB1D_W {
        USB1D_W { w: self }
    }
    #[doc = "Bit 6 - Enables the clock for the USB1 RAM interface."]
    #[inline(always)]
    pub fn usb1ram(&mut self) -> USB1RAM_W {
        USB1RAM_W { w: self }
    }
    #[doc = "Bit 7 - Enables the clock for the EMC interface."]
    #[inline(always)]
    pub fn emc(&mut self) -> EMC_W {
        EMC_W { w: self }
    }
    #[doc = "Bit 8 - Enables the clock for the ethernet interface."]
    #[inline(always)]
    pub fn eth(&mut self) -> ETH_W {
        ETH_W { w: self }
    }
    #[doc = "Bit 9 - Enables the clock for the GPIO4 interface."]
    #[inline(always)]
    pub fn gpio4(&mut self) -> GPIO4_W {
        GPIO4_W { w: self }
    }
    #[doc = "Bit 10 - Enables the clock for the GPIO5 interface."]
    #[inline(always)]
    pub fn gpio5(&mut self) -> GPIO5_W {
        GPIO5_W { w: self }
    }
    #[doc = "Bit 11 - Enables the clock for the AES interface."]
    #[inline(always)]
    pub fn aes(&mut self) -> AES_W {
        AES_W { w: self }
    }
    #[doc = "Bit 12 - Enables the clock for the OTP interface."]
    #[inline(always)]
    pub fn otp(&mut self) -> OTP_W {
        OTP_W { w: self }
    }
    #[doc = "Bit 13 - Enables the clock for the RNG interface."]
    #[inline(always)]
    pub fn rng(&mut self) -> RNG_W {
        RNG_W { w: self }
    }
    #[doc = "Bit 14 - Enables the clock for the Flexcomm8 interface."]
    #[inline(always)]
    pub fn flexcomm8(&mut self) -> FLEXCOMM8_W {
        FLEXCOMM8_W { w: self }
    }
    #[doc = "Bit 15 - Enables the clock for the Flexcomm9 interface."]
    #[inline(always)]
    pub fn flexcomm9(&mut self) -> FLEXCOMM9_W {
        FLEXCOMM9_W { w: self }
    }
    #[doc = "Bit 16 - Enables the clock for the USB host master interface."]
    #[inline(always)]
    pub fn usb0hmr(&mut self) -> USB0HMR_W {
        USB0HMR_W { w: self }
    }
    #[doc = "Bit 17 - Enables the clock for the USB host slave interface."]
    #[inline(always)]
    pub fn usb0hsl(&mut self) -> USB0HSL_W {
        USB0HSL_W { w: self }
    }
    #[doc = "Bit 18 - Enables the clock for the SHA interface."]
    #[inline(always)]
    pub fn sha0(&mut self) -> SHA0_W {
        SHA0_W { w: self }
    }
    #[doc = "Bit 19 - Enables the clock for the Smart card0 interface."]
    #[inline(always)]
    pub fn sc0(&mut self) -> SC0_W {
        SC0_W { w: self }
    }
    #[doc = "Bit 20 - Enables the clock for the Smart card1 interface."]
    #[inline(always)]
    pub fn sc1(&mut self) -> SC1_W {
        SC1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB Clock control n\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahbclkctrl2](index.html) module"]
pub struct AHBCLKCTRL2_SPEC;
impl crate::RegisterSpec for AHBCLKCTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahbclkctrl2::R](R) reader structure"]
impl crate::Readable for AHBCLKCTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahbclkctrl2::W](W) writer structure"]
impl crate::Writable for AHBCLKCTRL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AHBCLKCTRL2 to value 0"]
impl crate::Resettable for AHBCLKCTRL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
