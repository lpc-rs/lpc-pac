///Register `DEVCMDSTAT` reader
pub struct R(crate::R<DEVCMDSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEVCMDSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEVCMDSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEVCMDSTAT_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DEVCMDSTAT` writer
pub struct W(crate::W<DEVCMDSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEVCMDSTAT_SPEC>;
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
impl From<crate::W<DEVCMDSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEVCMDSTAT_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DEV_ADDR` reader - USB device address.
pub struct DEV_ADDR_R(crate::FieldReader<u8, u8>);
impl DEV_ADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DEV_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEV_ADDR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DEV_ADDR` writer - USB device address.
pub struct DEV_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> DEV_ADDR_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
///Field `DEV_EN` reader - USB device enable.
pub struct DEV_EN_R(crate::FieldReader<bool, bool>);
impl DEV_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DEV_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEV_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DEV_EN` writer - USB device enable.
pub struct DEV_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DEV_EN_W<'a> {
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
///Field `SETUP` reader - SETUP token received.
pub struct SETUP_R(crate::FieldReader<bool, bool>);
impl SETUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SETUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SETUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SETUP` writer - SETUP token received.
pub struct SETUP_W<'a> {
    w: &'a mut W,
}
impl<'a> SETUP_W<'a> {
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
///Field `FORCE_NEEDCLK` reader - Forces the NEEDCLK output to always be on:.
pub struct FORCE_NEEDCLK_R(crate::FieldReader<bool, bool>);
impl FORCE_NEEDCLK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FORCE_NEEDCLK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FORCE_NEEDCLK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `FORCE_NEEDCLK` writer - Forces the NEEDCLK output to always be on:.
pub struct FORCE_NEEDCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_NEEDCLK_W<'a> {
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
///Field `FORCE_VBUS` reader - If this bit is set to 1, the VBUS voltage indicators from the PHY are overruled.
pub struct FORCE_VBUS_R(crate::FieldReader<bool, bool>);
impl FORCE_VBUS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FORCE_VBUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FORCE_VBUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `FORCE_VBUS` writer - If this bit is set to 1, the VBUS voltage indicators from the PHY are overruled.
pub struct FORCE_VBUS_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_VBUS_W<'a> {
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
///Field `LPM_SUP` reader - LPM Supported:.
pub struct LPM_SUP_R(crate::FieldReader<bool, bool>);
impl LPM_SUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LPM_SUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPM_SUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `LPM_SUP` writer - LPM Supported:.
pub struct LPM_SUP_W<'a> {
    w: &'a mut W,
}
impl<'a> LPM_SUP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
///Field `INTONNAK_AO` reader - Interrupt on NAK for interrupt and bulk OUT EP:.
pub struct INTONNAK_AO_R(crate::FieldReader<bool, bool>);
impl INTONNAK_AO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INTONNAK_AO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTONNAK_AO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `INTONNAK_AO` writer - Interrupt on NAK for interrupt and bulk OUT EP:.
pub struct INTONNAK_AO_W<'a> {
    w: &'a mut W,
}
impl<'a> INTONNAK_AO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
///Field `INTONNAK_AI` reader - Interrupt on NAK for interrupt and bulk IN EP:.
pub struct INTONNAK_AI_R(crate::FieldReader<bool, bool>);
impl INTONNAK_AI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INTONNAK_AI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTONNAK_AI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `INTONNAK_AI` writer - Interrupt on NAK for interrupt and bulk IN EP:.
pub struct INTONNAK_AI_W<'a> {
    w: &'a mut W,
}
impl<'a> INTONNAK_AI_W<'a> {
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
///Field `INTONNAK_CO` reader - Interrupt on NAK for control OUT EP:.
pub struct INTONNAK_CO_R(crate::FieldReader<bool, bool>);
impl INTONNAK_CO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INTONNAK_CO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTONNAK_CO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `INTONNAK_CO` writer - Interrupt on NAK for control OUT EP:.
pub struct INTONNAK_CO_W<'a> {
    w: &'a mut W,
}
impl<'a> INTONNAK_CO_W<'a> {
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
///Field `INTONNAK_CI` reader - Interrupt on NAK for control IN EP:.
pub struct INTONNAK_CI_R(crate::FieldReader<bool, bool>);
impl INTONNAK_CI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INTONNAK_CI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTONNAK_CI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `INTONNAK_CI` writer - Interrupt on NAK for control IN EP:.
pub struct INTONNAK_CI_W<'a> {
    w: &'a mut W,
}
impl<'a> INTONNAK_CI_W<'a> {
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
///Field `DCON` reader - Device status - connect.
pub struct DCON_R(crate::FieldReader<bool, bool>);
impl DCON_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DCON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DCON` writer - Device status - connect.
pub struct DCON_W<'a> {
    w: &'a mut W,
}
impl<'a> DCON_W<'a> {
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
///Field `DSUS` reader - Device status - suspend.
pub struct DSUS_R(crate::FieldReader<bool, bool>);
impl DSUS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DSUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DSUS` writer - Device status - suspend.
pub struct DSUS_W<'a> {
    w: &'a mut W,
}
impl<'a> DSUS_W<'a> {
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
///Field `LPM_SUS` reader - Device status - LPM Suspend.
pub struct LPM_SUS_R(crate::FieldReader<bool, bool>);
impl LPM_SUS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LPM_SUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPM_SUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `LPM_SUS` writer - Device status - LPM Suspend.
pub struct LPM_SUS_W<'a> {
    w: &'a mut W,
}
impl<'a> LPM_SUS_W<'a> {
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
///Field `LPM_REWP` reader - LPM Remote Wake-up Enabled by USB host.
pub struct LPM_REWP_R(crate::FieldReader<bool, bool>);
impl LPM_REWP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LPM_REWP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPM_REWP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `Speed` reader - This field indicates the speed at which the device operates: 00b: reserved 01b: full-speed 10b: high-speed 11b: super-speed (reserved for future use).
pub struct SPEED_R(crate::FieldReader<u8, u8>);
impl SPEED_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SPEED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPEED_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DCON_C` reader - Device status - connect change.
pub struct DCON_C_R(crate::FieldReader<bool, bool>);
impl DCON_C_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DCON_C_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCON_C_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DCON_C` writer - Device status - connect change.
pub struct DCON_C_W<'a> {
    w: &'a mut W,
}
impl<'a> DCON_C_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
///Field `DSUS_C` reader - Device status - suspend change.
pub struct DSUS_C_R(crate::FieldReader<bool, bool>);
impl DSUS_C_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DSUS_C_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSUS_C_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DSUS_C` writer - Device status - suspend change.
pub struct DSUS_C_W<'a> {
    w: &'a mut W,
}
impl<'a> DSUS_C_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
///Field `DRES_C` reader - Device status - reset change.
pub struct DRES_C_R(crate::FieldReader<bool, bool>);
impl DRES_C_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DRES_C_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DRES_C_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DRES_C` writer - Device status - reset change.
pub struct DRES_C_W<'a> {
    w: &'a mut W,
}
impl<'a> DRES_C_W<'a> {
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
///Field `VBUS_DEBOUNCED` reader - This bit indicates if VBUS is detected or not.
pub struct VBUS_DEBOUNCED_R(crate::FieldReader<bool, bool>);
impl VBUS_DEBOUNCED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VBUS_DEBOUNCED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VBUS_DEBOUNCED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PHY_TEST_MODE` reader - This field is written by firmware to put the PHY into a test mode as defined by the USB2.
pub struct PHY_TEST_MODE_R(crate::FieldReader<u8, u8>);
impl PHY_TEST_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PHY_TEST_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PHY_TEST_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PHY_TEST_MODE` writer - This field is written by firmware to put the PHY into a test mode as defined by the USB2.
pub struct PHY_TEST_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> PHY_TEST_MODE_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 29)) | ((value as u32 & 0x07) << 29);
        self.w
    }
}
impl R {
    ///Bits 0:6 - USB device address.
    #[inline(always)]
    pub fn dev_addr(&self) -> DEV_ADDR_R {
        DEV_ADDR_R::new((self.bits & 0x7f) as u8)
    }
    ///Bit 7 - USB device enable.
    #[inline(always)]
    pub fn dev_en(&self) -> DEV_EN_R {
        DEV_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 8 - SETUP token received.
    #[inline(always)]
    pub fn setup(&self) -> SETUP_R {
        SETUP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 9 - Forces the NEEDCLK output to always be on:.
    #[inline(always)]
    pub fn force_needclk(&self) -> FORCE_NEEDCLK_R {
        FORCE_NEEDCLK_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 10 - If this bit is set to 1, the VBUS voltage indicators from the PHY are overruled.
    #[inline(always)]
    pub fn force_vbus(&self) -> FORCE_VBUS_R {
        FORCE_VBUS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 11 - LPM Supported:.
    #[inline(always)]
    pub fn lpm_sup(&self) -> LPM_SUP_R {
        LPM_SUP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 12 - Interrupt on NAK for interrupt and bulk OUT EP:.
    #[inline(always)]
    pub fn intonnak_ao(&self) -> INTONNAK_AO_R {
        INTONNAK_AO_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 13 - Interrupt on NAK for interrupt and bulk IN EP:.
    #[inline(always)]
    pub fn intonnak_ai(&self) -> INTONNAK_AI_R {
        INTONNAK_AI_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 14 - Interrupt on NAK for control OUT EP:.
    #[inline(always)]
    pub fn intonnak_co(&self) -> INTONNAK_CO_R {
        INTONNAK_CO_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 15 - Interrupt on NAK for control IN EP:.
    #[inline(always)]
    pub fn intonnak_ci(&self) -> INTONNAK_CI_R {
        INTONNAK_CI_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 16 - Device status - connect.
    #[inline(always)]
    pub fn dcon(&self) -> DCON_R {
        DCON_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 17 - Device status - suspend.
    #[inline(always)]
    pub fn dsus(&self) -> DSUS_R {
        DSUS_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    ///Bit 19 - Device status - LPM Suspend.
    #[inline(always)]
    pub fn lpm_sus(&self) -> LPM_SUS_R {
        LPM_SUS_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    ///Bit 20 - LPM Remote Wake-up Enabled by USB host.
    #[inline(always)]
    pub fn lpm_rewp(&self) -> LPM_REWP_R {
        LPM_REWP_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    ///Bits 22:23 - This field indicates the speed at which the device operates: 00b: reserved 01b: full-speed 10b: high-speed 11b: super-speed (reserved for future use).
    #[inline(always)]
    pub fn speed(&self) -> SPEED_R {
        SPEED_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    ///Bit 24 - Device status - connect change.
    #[inline(always)]
    pub fn dcon_c(&self) -> DCON_C_R {
        DCON_C_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    ///Bit 25 - Device status - suspend change.
    #[inline(always)]
    pub fn dsus_c(&self) -> DSUS_C_R {
        DSUS_C_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    ///Bit 26 - Device status - reset change.
    #[inline(always)]
    pub fn dres_c(&self) -> DRES_C_R {
        DRES_C_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    ///Bit 28 - This bit indicates if VBUS is detected or not.
    #[inline(always)]
    pub fn vbus_debounced(&self) -> VBUS_DEBOUNCED_R {
        VBUS_DEBOUNCED_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    ///Bits 29:31 - This field is written by firmware to put the PHY into a test mode as defined by the USB2.
    #[inline(always)]
    pub fn phy_test_mode(&self) -> PHY_TEST_MODE_R {
        PHY_TEST_MODE_R::new(((self.bits >> 29) & 0x07) as u8)
    }
}
impl W {
    ///Bits 0:6 - USB device address.
    #[inline(always)]
    pub fn dev_addr(&mut self) -> DEV_ADDR_W {
        DEV_ADDR_W { w: self }
    }
    ///Bit 7 - USB device enable.
    #[inline(always)]
    pub fn dev_en(&mut self) -> DEV_EN_W {
        DEV_EN_W { w: self }
    }
    ///Bit 8 - SETUP token received.
    #[inline(always)]
    pub fn setup(&mut self) -> SETUP_W {
        SETUP_W { w: self }
    }
    ///Bit 9 - Forces the NEEDCLK output to always be on:.
    #[inline(always)]
    pub fn force_needclk(&mut self) -> FORCE_NEEDCLK_W {
        FORCE_NEEDCLK_W { w: self }
    }
    ///Bit 10 - If this bit is set to 1, the VBUS voltage indicators from the PHY are overruled.
    #[inline(always)]
    pub fn force_vbus(&mut self) -> FORCE_VBUS_W {
        FORCE_VBUS_W { w: self }
    }
    ///Bit 11 - LPM Supported:.
    #[inline(always)]
    pub fn lpm_sup(&mut self) -> LPM_SUP_W {
        LPM_SUP_W { w: self }
    }
    ///Bit 12 - Interrupt on NAK for interrupt and bulk OUT EP:.
    #[inline(always)]
    pub fn intonnak_ao(&mut self) -> INTONNAK_AO_W {
        INTONNAK_AO_W { w: self }
    }
    ///Bit 13 - Interrupt on NAK for interrupt and bulk IN EP:.
    #[inline(always)]
    pub fn intonnak_ai(&mut self) -> INTONNAK_AI_W {
        INTONNAK_AI_W { w: self }
    }
    ///Bit 14 - Interrupt on NAK for control OUT EP:.
    #[inline(always)]
    pub fn intonnak_co(&mut self) -> INTONNAK_CO_W {
        INTONNAK_CO_W { w: self }
    }
    ///Bit 15 - Interrupt on NAK for control IN EP:.
    #[inline(always)]
    pub fn intonnak_ci(&mut self) -> INTONNAK_CI_W {
        INTONNAK_CI_W { w: self }
    }
    ///Bit 16 - Device status - connect.
    #[inline(always)]
    pub fn dcon(&mut self) -> DCON_W {
        DCON_W { w: self }
    }
    ///Bit 17 - Device status - suspend.
    #[inline(always)]
    pub fn dsus(&mut self) -> DSUS_W {
        DSUS_W { w: self }
    }
    ///Bit 19 - Device status - LPM Suspend.
    #[inline(always)]
    pub fn lpm_sus(&mut self) -> LPM_SUS_W {
        LPM_SUS_W { w: self }
    }
    ///Bit 24 - Device status - connect change.
    #[inline(always)]
    pub fn dcon_c(&mut self) -> DCON_C_W {
        DCON_C_W { w: self }
    }
    ///Bit 25 - Device status - suspend change.
    #[inline(always)]
    pub fn dsus_c(&mut self) -> DSUS_C_W {
        DSUS_C_W { w: self }
    }
    ///Bit 26 - Device status - reset change.
    #[inline(always)]
    pub fn dres_c(&mut self) -> DRES_C_W {
        DRES_C_W { w: self }
    }
    ///Bits 29:31 - This field is written by firmware to put the PHY into a test mode as defined by the USB2.
    #[inline(always)]
    pub fn phy_test_mode(&mut self) -> PHY_TEST_MODE_W {
        PHY_TEST_MODE_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///USB Device Command/Status register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [devcmdstat](index.html) module
pub struct DEVCMDSTAT_SPEC;
impl crate::RegisterSpec for DEVCMDSTAT_SPEC {
    type Ux = u32;
}
///`read()` method returns [devcmdstat::R](R) reader structure
impl crate::Readable for DEVCMDSTAT_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [devcmdstat::W](W) writer structure
impl crate::Writable for DEVCMDSTAT_SPEC {
    type Writer = W;
}
///`reset()` method sets DEVCMDSTAT to value 0x0800
impl crate::Resettable for DEVCMDSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0800
    }
}
