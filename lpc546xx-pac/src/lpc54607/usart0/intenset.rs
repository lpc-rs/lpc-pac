///Register `INTENSET` reader
pub struct R(crate::R<INTENSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTENSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTENSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTENSET_SPEC>) -> Self {
        R(reader)
    }
}
///Register `INTENSET` writer
pub struct W(crate::W<INTENSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTENSET_SPEC>;
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
impl From<crate::W<INTENSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTENSET_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TXIDLEEN` reader - When 1, enables an interrupt when the transmitter becomes idle (TXIDLE = 1).
pub struct TXIDLEEN_R(crate::FieldReader<bool, bool>);
impl TXIDLEEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXIDLEEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXIDLEEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TXIDLEEN` writer - When 1, enables an interrupt when the transmitter becomes idle (TXIDLE = 1).
pub struct TXIDLEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXIDLEEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
///Field `DELTACTSEN` reader - When 1, enables an interrupt when there is a change in the state of the CTS input.
pub struct DELTACTSEN_R(crate::FieldReader<bool, bool>);
impl DELTACTSEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DELTACTSEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DELTACTSEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DELTACTSEN` writer - When 1, enables an interrupt when there is a change in the state of the CTS input.
pub struct DELTACTSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DELTACTSEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
///Field `TXDISEN` reader - When 1, enables an interrupt when the transmitter is fully disabled as indicated by the TXDISINT flag in STAT. See description of the TXDISINT bit for details.
pub struct TXDISEN_R(crate::FieldReader<bool, bool>);
impl TXDISEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXDISEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXDISEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TXDISEN` writer - When 1, enables an interrupt when the transmitter is fully disabled as indicated by the TXDISINT flag in STAT. See description of the TXDISINT bit for details.
pub struct TXDISEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDISEN_W<'a> {
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
///Field `DELTARXBRKEN` reader - When 1, enables an interrupt when a change of state has occurred in the detection of a received break condition (break condition asserted or deasserted).
pub struct DELTARXBRKEN_R(crate::FieldReader<bool, bool>);
impl DELTARXBRKEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DELTARXBRKEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DELTARXBRKEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DELTARXBRKEN` writer - When 1, enables an interrupt when a change of state has occurred in the detection of a received break condition (break condition asserted or deasserted).
pub struct DELTARXBRKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DELTARXBRKEN_W<'a> {
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
///Field `STARTEN` reader - When 1, enables an interrupt when a received start bit has been detected.
pub struct STARTEN_R(crate::FieldReader<bool, bool>);
impl STARTEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STARTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STARTEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `STARTEN` writer - When 1, enables an interrupt when a received start bit has been detected.
pub struct STARTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> STARTEN_W<'a> {
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
///Field `FRAMERREN` reader - When 1, enables an interrupt when a framing error has been detected.
pub struct FRAMERREN_R(crate::FieldReader<bool, bool>);
impl FRAMERREN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FRAMERREN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRAMERREN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `FRAMERREN` writer - When 1, enables an interrupt when a framing error has been detected.
pub struct FRAMERREN_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAMERREN_W<'a> {
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
///Field `PARITYERREN` reader - When 1, enables an interrupt when a parity error has been detected.
pub struct PARITYERREN_R(crate::FieldReader<bool, bool>);
impl PARITYERREN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PARITYERREN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PARITYERREN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PARITYERREN` writer - When 1, enables an interrupt when a parity error has been detected.
pub struct PARITYERREN_W<'a> {
    w: &'a mut W,
}
impl<'a> PARITYERREN_W<'a> {
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
///Field `RXNOISEEN` reader - When 1, enables an interrupt when noise is detected. See description of the RXNOISEINT bit in Table 354.
pub struct RXNOISEEN_R(crate::FieldReader<bool, bool>);
impl RXNOISEEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXNOISEEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXNOISEEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RXNOISEEN` writer - When 1, enables an interrupt when noise is detected. See description of the RXNOISEINT bit in Table 354.
pub struct RXNOISEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXNOISEEN_W<'a> {
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
///Field `ABERREN` reader - When 1, enables an interrupt when an auto baud error occurs.
pub struct ABERREN_R(crate::FieldReader<bool, bool>);
impl ABERREN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ABERREN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ABERREN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ABERREN` writer - When 1, enables an interrupt when an auto baud error occurs.
pub struct ABERREN_W<'a> {
    w: &'a mut W,
}
impl<'a> ABERREN_W<'a> {
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
impl R {
    ///Bit 3 - When 1, enables an interrupt when the transmitter becomes idle (TXIDLE = 1).
    #[inline(always)]
    pub fn txidleen(&self) -> TXIDLEEN_R {
        TXIDLEEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 5 - When 1, enables an interrupt when there is a change in the state of the CTS input.
    #[inline(always)]
    pub fn deltactsen(&self) -> DELTACTSEN_R {
        DELTACTSEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 6 - When 1, enables an interrupt when the transmitter is fully disabled as indicated by the TXDISINT flag in STAT. See description of the TXDISINT bit for details.
    #[inline(always)]
    pub fn txdisen(&self) -> TXDISEN_R {
        TXDISEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 11 - When 1, enables an interrupt when a change of state has occurred in the detection of a received break condition (break condition asserted or deasserted).
    #[inline(always)]
    pub fn deltarxbrken(&self) -> DELTARXBRKEN_R {
        DELTARXBRKEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 12 - When 1, enables an interrupt when a received start bit has been detected.
    #[inline(always)]
    pub fn starten(&self) -> STARTEN_R {
        STARTEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 13 - When 1, enables an interrupt when a framing error has been detected.
    #[inline(always)]
    pub fn framerren(&self) -> FRAMERREN_R {
        FRAMERREN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 14 - When 1, enables an interrupt when a parity error has been detected.
    #[inline(always)]
    pub fn parityerren(&self) -> PARITYERREN_R {
        PARITYERREN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 15 - When 1, enables an interrupt when noise is detected. See description of the RXNOISEINT bit in Table 354.
    #[inline(always)]
    pub fn rxnoiseen(&self) -> RXNOISEEN_R {
        RXNOISEEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 16 - When 1, enables an interrupt when an auto baud error occurs.
    #[inline(always)]
    pub fn aberren(&self) -> ABERREN_R {
        ABERREN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    ///Bit 3 - When 1, enables an interrupt when the transmitter becomes idle (TXIDLE = 1).
    #[inline(always)]
    pub fn txidleen(&mut self) -> TXIDLEEN_W {
        TXIDLEEN_W { w: self }
    }
    ///Bit 5 - When 1, enables an interrupt when there is a change in the state of the CTS input.
    #[inline(always)]
    pub fn deltactsen(&mut self) -> DELTACTSEN_W {
        DELTACTSEN_W { w: self }
    }
    ///Bit 6 - When 1, enables an interrupt when the transmitter is fully disabled as indicated by the TXDISINT flag in STAT. See description of the TXDISINT bit for details.
    #[inline(always)]
    pub fn txdisen(&mut self) -> TXDISEN_W {
        TXDISEN_W { w: self }
    }
    ///Bit 11 - When 1, enables an interrupt when a change of state has occurred in the detection of a received break condition (break condition asserted or deasserted).
    #[inline(always)]
    pub fn deltarxbrken(&mut self) -> DELTARXBRKEN_W {
        DELTARXBRKEN_W { w: self }
    }
    ///Bit 12 - When 1, enables an interrupt when a received start bit has been detected.
    #[inline(always)]
    pub fn starten(&mut self) -> STARTEN_W {
        STARTEN_W { w: self }
    }
    ///Bit 13 - When 1, enables an interrupt when a framing error has been detected.
    #[inline(always)]
    pub fn framerren(&mut self) -> FRAMERREN_W {
        FRAMERREN_W { w: self }
    }
    ///Bit 14 - When 1, enables an interrupt when a parity error has been detected.
    #[inline(always)]
    pub fn parityerren(&mut self) -> PARITYERREN_W {
        PARITYERREN_W { w: self }
    }
    ///Bit 15 - When 1, enables an interrupt when noise is detected. See description of the RXNOISEINT bit in Table 354.
    #[inline(always)]
    pub fn rxnoiseen(&mut self) -> RXNOISEEN_W {
        RXNOISEEN_W { w: self }
    }
    ///Bit 16 - When 1, enables an interrupt when an auto baud error occurs.
    #[inline(always)]
    pub fn aberren(&mut self) -> ABERREN_W {
        ABERREN_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Interrupt Enable read and Set register for USART (not FIFO) status. Contains individual interrupt enable bits for each potential USART interrupt. A complete value may be read from this register. Writing a 1 to any implemented bit position causes that bit to be set.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [intenset](index.html) module
pub struct INTENSET_SPEC;
impl crate::RegisterSpec for INTENSET_SPEC {
    type Ux = u32;
}
///`read()` method returns [intenset::R](R) reader structure
impl crate::Readable for INTENSET_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [intenset::W](W) writer structure
impl crate::Writable for INTENSET_SPEC {
    type Writer = W;
}
///`reset()` method sets INTENSET to value 0
impl crate::Resettable for INTENSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
