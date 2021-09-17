#[doc = "Register `MAC_LPI_CTRL_STAT` reader"]
pub struct R(crate::R<MAC_LPI_CTRL_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_LPI_CTRL_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_LPI_CTRL_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_LPI_CTRL_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAC_LPI_CTRL_STAT` writer"]
pub struct W(crate::W<MAC_LPI_CTRL_STAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_LPI_CTRL_STAT_SPEC>;
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
impl From<crate::W<MAC_LPI_CTRL_STAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC_LPI_CTRL_STAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TLPIEN` reader - Transmit LPI Entry When this bit is set, it indicates that the MAC Transmitter has entered the LPI state because of the setting of the LPIEN bit."]
pub struct TLPIEN_R(crate::FieldReader<bool, bool>);
impl TLPIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TLPIEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TLPIEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TLPIEX` reader - Transmit LPI Exit When this bit is set, it indicates that the MAC transmitter exited the LPI state after the application cleared the LPIEN bit and the LPI TW Timer has expired."]
pub struct TLPIEX_R(crate::FieldReader<bool, bool>);
impl TLPIEX_R {
    pub(crate) fn new(bits: bool) -> Self {
        TLPIEX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TLPIEX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RLPIEN` reader - Receive LPI Entry When this bit is set, it indicates that the MAC Receiver has received an LPI pattern and entered the LPI state."]
pub struct RLPIEN_R(crate::FieldReader<bool, bool>);
impl RLPIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RLPIEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RLPIEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RLPIEX` reader - Receive LPI Exit When this bit is set, it indicates that the MAC Receiver has stopped receiving the LPI pattern on the MII interface, exited the LPI state, and resumed the normal reception."]
pub struct RLPIEX_R(crate::FieldReader<bool, bool>);
impl RLPIEX_R {
    pub(crate) fn new(bits: bool) -> Self {
        RLPIEX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RLPIEX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TLPIST` reader - Transmit LPI State When this bit is set, it indicates that the MAC is transmitting the LPI pattern on the MII interface."]
pub struct TLPIST_R(crate::FieldReader<bool, bool>);
impl TLPIST_R {
    pub(crate) fn new(bits: bool) -> Self {
        TLPIST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TLPIST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RLPIST` reader - Receive LPI State When this bit is set, it indicates that the MAC is receiving the LPI pattern on the MII interface."]
pub struct RLPIST_R(crate::FieldReader<bool, bool>);
impl RLPIST_R {
    pub(crate) fn new(bits: bool) -> Self {
        RLPIST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RLPIST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPIEN` reader - LPI Enable When this bit is set, it instructs the MAC Transmitter to enter the LPI state."]
pub struct LPIEN_R(crate::FieldReader<bool, bool>);
impl LPIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPIEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPIEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPIEN` writer - LPI Enable When this bit is set, it instructs the MAC Transmitter to enter the LPI state."]
pub struct LPIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPIEN_W<'a> {
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
#[doc = "Field `PLS` reader - PHY Link Status This bit indicates the link status of the PHY."]
pub struct PLS_R(crate::FieldReader<bool, bool>);
impl PLS_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLS` writer - PHY Link Status This bit indicates the link status of the PHY."]
pub struct PLS_W<'a> {
    w: &'a mut W,
}
impl<'a> PLS_W<'a> {
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
#[doc = "Field `LPITXA` reader - LPI Tx Automate This bit controls the behavior of the MAC when it is entering or coming out of the LPI mode on the Transmit side."]
pub struct LPITXA_R(crate::FieldReader<bool, bool>);
impl LPITXA_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPITXA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPITXA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPITXA` writer - LPI Tx Automate This bit controls the behavior of the MAC when it is entering or coming out of the LPI mode on the Transmit side."]
pub struct LPITXA_W<'a> {
    w: &'a mut W,
}
impl<'a> LPITXA_W<'a> {
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
#[doc = "Field `LPIATE` reader - LPI Timer Enable This bit controls the automatic entry of the MAC Transmitter into and exit out of the LPI state."]
pub struct LPIATE_R(crate::FieldReader<bool, bool>);
impl LPIATE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPIATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPIATE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPIATE` writer - LPI Timer Enable This bit controls the automatic entry of the MAC Transmitter into and exit out of the LPI state."]
pub struct LPIATE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPIATE_W<'a> {
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
#[doc = "Field `LPITCSE` reader - LPI Tx Clock Stop Enable When this bit is set, the MAC asserts LPI Tx Clock Gating Control signal high after it enters Tx LPI mode to indicate that the Tx clock to MAC can be stopped."]
pub struct LPITCSE_R(crate::FieldReader<bool, bool>);
impl LPITCSE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPITCSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPITCSE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPITCSE` writer - LPI Tx Clock Stop Enable When this bit is set, the MAC asserts LPI Tx Clock Gating Control signal high after it enters Tx LPI mode to indicate that the Tx clock to MAC can be stopped."]
pub struct LPITCSE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPITCSE_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Transmit LPI Entry When this bit is set, it indicates that the MAC Transmitter has entered the LPI state because of the setting of the LPIEN bit."]
    #[inline(always)]
    pub fn tlpien(&self) -> TLPIEN_R {
        TLPIEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit LPI Exit When this bit is set, it indicates that the MAC transmitter exited the LPI state after the application cleared the LPIEN bit and the LPI TW Timer has expired."]
    #[inline(always)]
    pub fn tlpiex(&self) -> TLPIEX_R {
        TLPIEX_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Receive LPI Entry When this bit is set, it indicates that the MAC Receiver has received an LPI pattern and entered the LPI state."]
    #[inline(always)]
    pub fn rlpien(&self) -> RLPIEN_R {
        RLPIEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Receive LPI Exit When this bit is set, it indicates that the MAC Receiver has stopped receiving the LPI pattern on the MII interface, exited the LPI state, and resumed the normal reception."]
    #[inline(always)]
    pub fn rlpiex(&self) -> RLPIEX_R {
        RLPIEX_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Transmit LPI State When this bit is set, it indicates that the MAC is transmitting the LPI pattern on the MII interface."]
    #[inline(always)]
    pub fn tlpist(&self) -> TLPIST_R {
        TLPIST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Receive LPI State When this bit is set, it indicates that the MAC is receiving the LPI pattern on the MII interface."]
    #[inline(always)]
    pub fn rlpist(&self) -> RLPIST_R {
        RLPIST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 16 - LPI Enable When this bit is set, it instructs the MAC Transmitter to enter the LPI state."]
    #[inline(always)]
    pub fn lpien(&self) -> LPIEN_R {
        LPIEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - PHY Link Status This bit indicates the link status of the PHY."]
    #[inline(always)]
    pub fn pls(&self) -> PLS_R {
        PLS_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 19 - LPI Tx Automate This bit controls the behavior of the MAC when it is entering or coming out of the LPI mode on the Transmit side."]
    #[inline(always)]
    pub fn lpitxa(&self) -> LPITXA_R {
        LPITXA_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - LPI Timer Enable This bit controls the automatic entry of the MAC Transmitter into and exit out of the LPI state."]
    #[inline(always)]
    pub fn lpiate(&self) -> LPIATE_R {
        LPIATE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - LPI Tx Clock Stop Enable When this bit is set, the MAC asserts LPI Tx Clock Gating Control signal high after it enters Tx LPI mode to indicate that the Tx clock to MAC can be stopped."]
    #[inline(always)]
    pub fn lpitcse(&self) -> LPITCSE_R {
        LPITCSE_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - LPI Enable When this bit is set, it instructs the MAC Transmitter to enter the LPI state."]
    #[inline(always)]
    pub fn lpien(&mut self) -> LPIEN_W {
        LPIEN_W { w: self }
    }
    #[doc = "Bit 17 - PHY Link Status This bit indicates the link status of the PHY."]
    #[inline(always)]
    pub fn pls(&mut self) -> PLS_W {
        PLS_W { w: self }
    }
    #[doc = "Bit 19 - LPI Tx Automate This bit controls the behavior of the MAC when it is entering or coming out of the LPI mode on the Transmit side."]
    #[inline(always)]
    pub fn lpitxa(&mut self) -> LPITXA_W {
        LPITXA_W { w: self }
    }
    #[doc = "Bit 20 - LPI Timer Enable This bit controls the automatic entry of the MAC Transmitter into and exit out of the LPI state."]
    #[inline(always)]
    pub fn lpiate(&mut self) -> LPIATE_W {
        LPIATE_W { w: self }
    }
    #[doc = "Bit 21 - LPI Tx Clock Stop Enable When this bit is set, the MAC asserts LPI Tx Clock Gating Control signal high after it enters Tx LPI mode to indicate that the Tx clock to MAC can be stopped."]
    #[inline(always)]
    pub fn lpitcse(&mut self) -> LPITCSE_W {
        LPITCSE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LPI Control and Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_lpi_ctrl_stat](index.html) module"]
pub struct MAC_LPI_CTRL_STAT_SPEC;
impl crate::RegisterSpec for MAC_LPI_CTRL_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_lpi_ctrl_stat::R](R) reader structure"]
impl crate::Readable for MAC_LPI_CTRL_STAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mac_lpi_ctrl_stat::W](W) writer structure"]
impl crate::Writable for MAC_LPI_CTRL_STAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAC_LPI_CTRL_STAT to value 0"]
impl crate::Resettable for MAC_LPI_CTRL_STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
