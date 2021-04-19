#[doc = "Register `PRESETCTRL2` reader"]
pub struct R(crate::R<PRESETCTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRESETCTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PRESETCTRL2_SPEC>> for R {
    fn from(reader: crate::R<PRESETCTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRESETCTRL2` writer"]
pub struct W(crate::W<PRESETCTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRESETCTRL2_SPEC>;
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
impl core::convert::From<crate::W<PRESETCTRL2_SPEC>> for W {
    fn from(writer: crate::W<PRESETCTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LCD_RST` reader - LCD reset control."]
pub struct LCD_RST_R(crate::FieldReader<bool, bool>);
impl LCD_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCD_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCD_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCD_RST` writer - LCD reset control."]
pub struct LCD_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> LCD_RST_W<'a> {
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
#[doc = "Field `SDIO_RST` reader - SDIO reset control."]
pub struct SDIO_RST_R(crate::FieldReader<bool, bool>);
impl SDIO_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        SDIO_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDIO_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDIO_RST` writer - SDIO reset control."]
pub struct SDIO_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_RST_W<'a> {
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
#[doc = "Field `USB1H_RST` reader - USB1 Host reset control."]
pub struct USB1H_RST_R(crate::FieldReader<bool, bool>);
impl USB1H_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        USB1H_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB1H_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB1H_RST` writer - USB1 Host reset control."]
pub struct USB1H_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> USB1H_RST_W<'a> {
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
#[doc = "Field `USB1D_RST` reader - USB1 Device reset control."]
pub struct USB1D_RST_R(crate::FieldReader<bool, bool>);
impl USB1D_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        USB1D_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB1D_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB1D_RST` writer - USB1 Device reset control."]
pub struct USB1D_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> USB1D_RST_W<'a> {
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
#[doc = "Field `USB1RAM_RST` reader - USB1 RAM reset control."]
pub struct USB1RAM_RST_R(crate::FieldReader<bool, bool>);
impl USB1RAM_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        USB1RAM_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB1RAM_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB1RAM_RST` writer - USB1 RAM reset control."]
pub struct USB1RAM_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> USB1RAM_RST_W<'a> {
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
#[doc = "Field `EMC_RESET` reader - EMC reset control."]
pub struct EMC_RESET_R(crate::FieldReader<bool, bool>);
impl EMC_RESET_R {
    pub(crate) fn new(bits: bool) -> Self {
        EMC_RESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EMC_RESET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EMC_RESET` writer - EMC reset control."]
pub struct EMC_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> EMC_RESET_W<'a> {
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
#[doc = "Field `ETH_RST` reader - Ethernet reset control."]
pub struct ETH_RST_R(crate::FieldReader<bool, bool>);
impl ETH_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        ETH_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ETH_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ETH_RST` writer - Ethernet reset control."]
pub struct ETH_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> ETH_RST_W<'a> {
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
#[doc = "Field `GPIO4_RST` reader - GPIO4 reset control."]
pub struct GPIO4_RST_R(crate::FieldReader<bool, bool>);
impl GPIO4_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO4_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO4_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO4_RST` writer - GPIO4 reset control."]
pub struct GPIO4_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO4_RST_W<'a> {
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
#[doc = "Field `GPIO5_RST` reader - GPIO5 reset control."]
pub struct GPIO5_RST_R(crate::FieldReader<bool, bool>);
impl GPIO5_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO5_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO5_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO5_RST` writer - GPIO5 reset control."]
pub struct GPIO5_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO5_RST_W<'a> {
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
#[doc = "Field `AES_RST` reader - AES reset control."]
pub struct AES_RST_R(crate::FieldReader<bool, bool>);
impl AES_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        AES_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AES_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AES_RST` writer - AES reset control."]
pub struct AES_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> AES_RST_W<'a> {
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
#[doc = "Field `OTP_RST` reader - OTP reset control."]
pub struct OTP_RST_R(crate::FieldReader<bool, bool>);
impl OTP_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        OTP_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OTP_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OTP_RST` writer - OTP reset control."]
pub struct OTP_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> OTP_RST_W<'a> {
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
#[doc = "Field `RNG_RST` reader - RNG reset control."]
pub struct RNG_RST_R(crate::FieldReader<bool, bool>);
impl RNG_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        RNG_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RNG_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RNG_RST` writer - RNG reset control."]
pub struct RNG_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> RNG_RST_W<'a> {
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
#[doc = "Field `FC8_RST` reader - Flexcomm 8 reset control."]
pub struct FC8_RST_R(crate::FieldReader<bool, bool>);
impl FC8_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        FC8_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FC8_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FC8_RST` writer - Flexcomm 8 reset control."]
pub struct FC8_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> FC8_RST_W<'a> {
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
#[doc = "Field `FC9_RST` reader - Flexcomm 9 reset control."]
pub struct FC9_RST_R(crate::FieldReader<bool, bool>);
impl FC9_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        FC9_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FC9_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FC9_RST` writer - Flexcomm 9 reset control."]
pub struct FC9_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> FC9_RST_W<'a> {
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
#[doc = "Field `USB0HMR_RST` reader - USB0 HOST master reset control."]
pub struct USB0HMR_RST_R(crate::FieldReader<bool, bool>);
impl USB0HMR_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        USB0HMR_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB0HMR_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB0HMR_RST` writer - USB0 HOST master reset control."]
pub struct USB0HMR_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> USB0HMR_RST_W<'a> {
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
#[doc = "Field `USB0HSL_RST` reader - USB0 HOST slave reset control."]
pub struct USB0HSL_RST_R(crate::FieldReader<bool, bool>);
impl USB0HSL_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        USB0HSL_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB0HSL_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB0HSL_RST` writer - USB0 HOST slave reset control."]
pub struct USB0HSL_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> USB0HSL_RST_W<'a> {
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
#[doc = "Field `SHA_RST` reader - SHA reset control."]
pub struct SHA_RST_R(crate::FieldReader<bool, bool>);
impl SHA_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        SHA_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SHA_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SHA_RST` writer - SHA reset control."]
pub struct SHA_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SHA_RST_W<'a> {
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
#[doc = "Field `SC0_RST` reader - Smart card 0 reset control."]
pub struct SC0_RST_R(crate::FieldReader<bool, bool>);
impl SC0_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        SC0_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SC0_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SC0_RST` writer - Smart card 0 reset control."]
pub struct SC0_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SC0_RST_W<'a> {
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
#[doc = "Field `SC1_RST` reader - Smart card 1 reset control."]
pub struct SC1_RST_R(crate::FieldReader<bool, bool>);
impl SC1_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        SC1_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SC1_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SC1_RST` writer - Smart card 1 reset control."]
pub struct SC1_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SC1_RST_W<'a> {
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
    #[doc = "Bit 2 - LCD reset control."]
    #[inline(always)]
    pub fn lcd_rst(&self) -> LCD_RST_R {
        LCD_RST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SDIO reset control."]
    #[inline(always)]
    pub fn sdio_rst(&self) -> SDIO_RST_R {
        SDIO_RST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - USB1 Host reset control."]
    #[inline(always)]
    pub fn usb1h_rst(&self) -> USB1H_RST_R {
        USB1H_RST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - USB1 Device reset control."]
    #[inline(always)]
    pub fn usb1d_rst(&self) -> USB1D_RST_R {
        USB1D_RST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - USB1 RAM reset control."]
    #[inline(always)]
    pub fn usb1ram_rst(&self) -> USB1RAM_RST_R {
        USB1RAM_RST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - EMC reset control."]
    #[inline(always)]
    pub fn emc_reset(&self) -> EMC_RESET_R {
        EMC_RESET_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Ethernet reset control."]
    #[inline(always)]
    pub fn eth_rst(&self) -> ETH_RST_R {
        ETH_RST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - GPIO4 reset control."]
    #[inline(always)]
    pub fn gpio4_rst(&self) -> GPIO4_RST_R {
        GPIO4_RST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - GPIO5 reset control."]
    #[inline(always)]
    pub fn gpio5_rst(&self) -> GPIO5_RST_R {
        GPIO5_RST_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - AES reset control."]
    #[inline(always)]
    pub fn aes_rst(&self) -> AES_RST_R {
        AES_RST_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - OTP reset control."]
    #[inline(always)]
    pub fn otp_rst(&self) -> OTP_RST_R {
        OTP_RST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - RNG reset control."]
    #[inline(always)]
    pub fn rng_rst(&self) -> RNG_RST_R {
        RNG_RST_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Flexcomm 8 reset control."]
    #[inline(always)]
    pub fn fc8_rst(&self) -> FC8_RST_R {
        FC8_RST_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Flexcomm 9 reset control."]
    #[inline(always)]
    pub fn fc9_rst(&self) -> FC9_RST_R {
        FC9_RST_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - USB0 HOST master reset control."]
    #[inline(always)]
    pub fn usb0hmr_rst(&self) -> USB0HMR_RST_R {
        USB0HMR_RST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - USB0 HOST slave reset control."]
    #[inline(always)]
    pub fn usb0hsl_rst(&self) -> USB0HSL_RST_R {
        USB0HSL_RST_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - SHA reset control."]
    #[inline(always)]
    pub fn sha_rst(&self) -> SHA_RST_R {
        SHA_RST_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Smart card 0 reset control."]
    #[inline(always)]
    pub fn sc0_rst(&self) -> SC0_RST_R {
        SC0_RST_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Smart card 1 reset control."]
    #[inline(always)]
    pub fn sc1_rst(&self) -> SC1_RST_R {
        SC1_RST_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - LCD reset control."]
    #[inline(always)]
    pub fn lcd_rst(&mut self) -> LCD_RST_W {
        LCD_RST_W { w: self }
    }
    #[doc = "Bit 3 - SDIO reset control."]
    #[inline(always)]
    pub fn sdio_rst(&mut self) -> SDIO_RST_W {
        SDIO_RST_W { w: self }
    }
    #[doc = "Bit 4 - USB1 Host reset control."]
    #[inline(always)]
    pub fn usb1h_rst(&mut self) -> USB1H_RST_W {
        USB1H_RST_W { w: self }
    }
    #[doc = "Bit 5 - USB1 Device reset control."]
    #[inline(always)]
    pub fn usb1d_rst(&mut self) -> USB1D_RST_W {
        USB1D_RST_W { w: self }
    }
    #[doc = "Bit 6 - USB1 RAM reset control."]
    #[inline(always)]
    pub fn usb1ram_rst(&mut self) -> USB1RAM_RST_W {
        USB1RAM_RST_W { w: self }
    }
    #[doc = "Bit 7 - EMC reset control."]
    #[inline(always)]
    pub fn emc_reset(&mut self) -> EMC_RESET_W {
        EMC_RESET_W { w: self }
    }
    #[doc = "Bit 8 - Ethernet reset control."]
    #[inline(always)]
    pub fn eth_rst(&mut self) -> ETH_RST_W {
        ETH_RST_W { w: self }
    }
    #[doc = "Bit 9 - GPIO4 reset control."]
    #[inline(always)]
    pub fn gpio4_rst(&mut self) -> GPIO4_RST_W {
        GPIO4_RST_W { w: self }
    }
    #[doc = "Bit 10 - GPIO5 reset control."]
    #[inline(always)]
    pub fn gpio5_rst(&mut self) -> GPIO5_RST_W {
        GPIO5_RST_W { w: self }
    }
    #[doc = "Bit 11 - AES reset control."]
    #[inline(always)]
    pub fn aes_rst(&mut self) -> AES_RST_W {
        AES_RST_W { w: self }
    }
    #[doc = "Bit 12 - OTP reset control."]
    #[inline(always)]
    pub fn otp_rst(&mut self) -> OTP_RST_W {
        OTP_RST_W { w: self }
    }
    #[doc = "Bit 13 - RNG reset control."]
    #[inline(always)]
    pub fn rng_rst(&mut self) -> RNG_RST_W {
        RNG_RST_W { w: self }
    }
    #[doc = "Bit 14 - Flexcomm 8 reset control."]
    #[inline(always)]
    pub fn fc8_rst(&mut self) -> FC8_RST_W {
        FC8_RST_W { w: self }
    }
    #[doc = "Bit 15 - Flexcomm 9 reset control."]
    #[inline(always)]
    pub fn fc9_rst(&mut self) -> FC9_RST_W {
        FC9_RST_W { w: self }
    }
    #[doc = "Bit 16 - USB0 HOST master reset control."]
    #[inline(always)]
    pub fn usb0hmr_rst(&mut self) -> USB0HMR_RST_W {
        USB0HMR_RST_W { w: self }
    }
    #[doc = "Bit 17 - USB0 HOST slave reset control."]
    #[inline(always)]
    pub fn usb0hsl_rst(&mut self) -> USB0HSL_RST_W {
        USB0HSL_RST_W { w: self }
    }
    #[doc = "Bit 18 - SHA reset control."]
    #[inline(always)]
    pub fn sha_rst(&mut self) -> SHA_RST_W {
        SHA_RST_W { w: self }
    }
    #[doc = "Bit 19 - Smart card 0 reset control."]
    #[inline(always)]
    pub fn sc0_rst(&mut self) -> SC0_RST_W {
        SC0_RST_W { w: self }
    }
    #[doc = "Bit 20 - Smart card 1 reset control."]
    #[inline(always)]
    pub fn sc1_rst(&mut self) -> SC1_RST_W {
        SC1_RST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral reset control n\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [presetctrl2](index.html) module"]
pub struct PRESETCTRL2_SPEC;
impl crate::RegisterSpec for PRESETCTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [presetctrl2::R](R) reader structure"]
impl crate::Readable for PRESETCTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [presetctrl2::W](W) writer structure"]
impl crate::Writable for PRESETCTRL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRESETCTRL2 to value 0"]
impl crate::Resettable for PRESETCTRL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
