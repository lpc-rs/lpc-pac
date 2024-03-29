#[doc = "Register `MTL_TXQx_INTCTRL_STAT` reader"]
pub struct R(crate::R<MTL_TXQX_INTCTRL_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MTL_TXQX_INTCTRL_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MTL_TXQX_INTCTRL_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MTL_TXQX_INTCTRL_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MTL_TXQx_INTCTRL_STAT` writer"]
pub struct W(crate::W<MTL_TXQX_INTCTRL_STAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MTL_TXQX_INTCTRL_STAT_SPEC>;
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
impl From<crate::W<MTL_TXQX_INTCTRL_STAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MTL_TXQX_INTCTRL_STAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXUNFIS` reader - Transmit Queue Underflow Interrupt Status This bit indicates that the Transmit Queue had an underflow while transmitting the packet."]
pub struct TXUNFIS_R(crate::FieldReader<bool, bool>);
impl TXUNFIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXUNFIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXUNFIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXUNFIS` writer - Transmit Queue Underflow Interrupt Status This bit indicates that the Transmit Queue had an underflow while transmitting the packet."]
pub struct TXUNFIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TXUNFIS_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `ABPSIS` reader - Average Bits Per Slot Interrupt Status When set, this bit indicates that the MAC has updated the ABS value."]
pub struct ABPSIS_R(crate::FieldReader<bool, bool>);
impl ABPSIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        ABPSIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ABPSIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ABPSIS` writer - Average Bits Per Slot Interrupt Status When set, this bit indicates that the MAC has updated the ABS value."]
pub struct ABPSIS_W<'a> {
    w: &'a mut W,
}
impl<'a> ABPSIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `TXUIE` reader - Transmit Queue Underflow Interrupt Enable When this bit is set, the Transmit Queue Underflow interrupt is enabled."]
pub struct TXUIE_R(crate::FieldReader<bool, bool>);
impl TXUIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXUIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXUIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXUIE` writer - Transmit Queue Underflow Interrupt Enable When this bit is set, the Transmit Queue Underflow interrupt is enabled."]
pub struct TXUIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXUIE_W<'a> {
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
#[doc = "Field `ABPSIE` reader - Average Bits Per Slot Interrupt Enable When this bit is set, the MAC asserts the interrupt when the average bits per slot status is updated."]
pub struct ABPSIE_R(crate::FieldReader<bool, bool>);
impl ABPSIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ABPSIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ABPSIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ABPSIE` writer - Average Bits Per Slot Interrupt Enable When this bit is set, the MAC asserts the interrupt when the average bits per slot status is updated."]
pub struct ABPSIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ABPSIE_W<'a> {
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
#[doc = "Field `RXOVFIS` reader - Receive Queue Overflow Interrupt Status This bit indicates that the Receive Queue had an overflow while receiving the packet."]
pub struct RXOVFIS_R(crate::FieldReader<bool, bool>);
impl RXOVFIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXOVFIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXOVFIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXOVFIS` writer - Receive Queue Overflow Interrupt Status This bit indicates that the Receive Queue had an overflow while receiving the packet."]
pub struct RXOVFIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOVFIS_W<'a> {
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
#[doc = "Field `RXOIE` reader - Receive Queue Overflow Interrupt Enable When this bit is set, the Receive Queue Overflow interrupt is enabled."]
pub struct RXOIE_R(crate::FieldReader<bool, bool>);
impl RXOIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXOIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXOIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXOIE` writer - Receive Queue Overflow Interrupt Enable When this bit is set, the Receive Queue Overflow interrupt is enabled."]
pub struct RXOIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Transmit Queue Underflow Interrupt Status This bit indicates that the Transmit Queue had an underflow while transmitting the packet."]
    #[inline(always)]
    pub fn txunfis(&self) -> TXUNFIS_R {
        TXUNFIS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Average Bits Per Slot Interrupt Status When set, this bit indicates that the MAC has updated the ABS value."]
    #[inline(always)]
    pub fn abpsis(&self) -> ABPSIS_R {
        ABPSIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Transmit Queue Underflow Interrupt Enable When this bit is set, the Transmit Queue Underflow interrupt is enabled."]
    #[inline(always)]
    pub fn txuie(&self) -> TXUIE_R {
        TXUIE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Average Bits Per Slot Interrupt Enable When this bit is set, the MAC asserts the interrupt when the average bits per slot status is updated."]
    #[inline(always)]
    pub fn abpsie(&self) -> ABPSIE_R {
        ABPSIE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Receive Queue Overflow Interrupt Status This bit indicates that the Receive Queue had an overflow while receiving the packet."]
    #[inline(always)]
    pub fn rxovfis(&self) -> RXOVFIS_R {
        RXOVFIS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Receive Queue Overflow Interrupt Enable When this bit is set, the Receive Queue Overflow interrupt is enabled."]
    #[inline(always)]
    pub fn rxoie(&self) -> RXOIE_R {
        RXOIE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Queue Underflow Interrupt Status This bit indicates that the Transmit Queue had an underflow while transmitting the packet."]
    #[inline(always)]
    pub fn txunfis(&mut self) -> TXUNFIS_W {
        TXUNFIS_W { w: self }
    }
    #[doc = "Bit 1 - Average Bits Per Slot Interrupt Status When set, this bit indicates that the MAC has updated the ABS value."]
    #[inline(always)]
    pub fn abpsis(&mut self) -> ABPSIS_W {
        ABPSIS_W { w: self }
    }
    #[doc = "Bit 8 - Transmit Queue Underflow Interrupt Enable When this bit is set, the Transmit Queue Underflow interrupt is enabled."]
    #[inline(always)]
    pub fn txuie(&mut self) -> TXUIE_W {
        TXUIE_W { w: self }
    }
    #[doc = "Bit 9 - Average Bits Per Slot Interrupt Enable When this bit is set, the MAC asserts the interrupt when the average bits per slot status is updated."]
    #[inline(always)]
    pub fn abpsie(&mut self) -> ABPSIE_W {
        ABPSIE_W { w: self }
    }
    #[doc = "Bit 16 - Receive Queue Overflow Interrupt Status This bit indicates that the Receive Queue had an overflow while receiving the packet."]
    #[inline(always)]
    pub fn rxovfis(&mut self) -> RXOVFIS_W {
        RXOVFIS_W { w: self }
    }
    #[doc = "Bit 24 - Receive Queue Overflow Interrupt Enable When this bit is set, the Receive Queue Overflow interrupt is enabled."]
    #[inline(always)]
    pub fn rxoie(&mut self) -> RXOIE_W {
        RXOIE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtl_txqx_intctrl_stat](index.html) module"]
pub struct MTL_TXQX_INTCTRL_STAT_SPEC;
impl crate::RegisterSpec for MTL_TXQX_INTCTRL_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mtl_txqx_intctrl_stat::R](R) reader structure"]
impl crate::Readable for MTL_TXQX_INTCTRL_STAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mtl_txqx_intctrl_stat::W](W) writer structure"]
impl crate::Writable for MTL_TXQX_INTCTRL_STAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MTL_TXQx_INTCTRL_STAT to value 0"]
impl crate::Resettable for MTL_TXQX_INTCTRL_STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
