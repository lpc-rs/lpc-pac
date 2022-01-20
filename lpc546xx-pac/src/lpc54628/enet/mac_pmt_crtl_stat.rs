#[doc = "Register `MAC_PMT_CRTL_STAT` reader"]
pub struct R(crate::R<MAC_PMT_CRTL_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_PMT_CRTL_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_PMT_CRTL_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_PMT_CRTL_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAC_PMT_CRTL_STAT` writer"]
pub struct W(crate::W<MAC_PMT_CRTL_STAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_PMT_CRTL_STAT_SPEC>;
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
impl From<crate::W<MAC_PMT_CRTL_STAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC_PMT_CRTL_STAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWRDWN` reader - Transmit LPI Entry When this bit is set, it indicates that the MAC Transmitter has entered the LPI state because of the setting of the LPIEN bit."]
pub struct PWRDWN_R(crate::FieldReader<bool, bool>);
impl PWRDWN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PWRDWN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWRDWN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MGKPKTEN` reader - Magic Packet Enable."]
pub struct MGKPKTEN_R(crate::FieldReader<bool, bool>);
impl MGKPKTEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MGKPKTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MGKPKTEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RWKPKTEN` reader - Remote Wake-Up Packet Enable When this bit is set, a power management event is generated when the MAC receives a remote wake-up packet."]
pub struct RWKPKTEN_R(crate::FieldReader<bool, bool>);
impl RWKPKTEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RWKPKTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RWKPKTEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MGKPRCVD` reader - Magic Packet Received."]
pub struct MGKPRCVD_R(crate::FieldReader<bool, bool>);
impl MGKPRCVD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MGKPRCVD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MGKPRCVD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RWKPRCVD` reader - Remote Wake-Up Packet Received."]
pub struct RWKPRCVD_R(crate::FieldReader<bool, bool>);
impl RWKPRCVD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RWKPRCVD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RWKPRCVD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GLBLUCAST` reader - Global Unicast When this bit set, any unicast packet filtered by the MAC (DAF) address recognition is detected as a remote wake-up packet."]
pub struct GLBLUCAST_R(crate::FieldReader<bool, bool>);
impl GLBLUCAST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GLBLUCAST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GLBLUCAST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GLBLUCAST` writer - Global Unicast When this bit set, any unicast packet filtered by the MAC (DAF) address recognition is detected as a remote wake-up packet."]
pub struct GLBLUCAST_W<'a> {
    w: &'a mut W,
}
impl<'a> GLBLUCAST_W<'a> {
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
#[doc = "Field `RWKPFE` reader - Remote Wake-up Packet Forwarding Enable When this bit is set along with RWKPKTEN, the MAC receiver drops all received frames until it receives the expected wake-up frame."]
pub struct RWKPFE_R(crate::FieldReader<bool, bool>);
impl RWKPFE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RWKPFE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RWKPFE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RWKPFE` writer - Remote Wake-up Packet Forwarding Enable When this bit is set along with RWKPKTEN, the MAC receiver drops all received frames until it receives the expected wake-up frame."]
pub struct RWKPFE_W<'a> {
    w: &'a mut W,
}
impl<'a> RWKPFE_W<'a> {
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
#[doc = "Field `RWKPTR` reader - Remote Wake-up FIFO Pointer This field gives the current value (0 to 7) of the Remote Wake-up Packet Filter register pointer."]
pub struct RWKPTR_R(crate::FieldReader<u8, u8>);
impl RWKPTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RWKPTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RWKPTR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RWKPTR` writer - Remote Wake-up FIFO Pointer This field gives the current value (0 to 7) of the Remote Wake-up Packet Filter register pointer."]
pub struct RWKPTR_W<'a> {
    w: &'a mut W,
}
impl<'a> RWKPTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | ((value as u32 & 0x1f) << 24);
        self.w
    }
}
#[doc = "Field `RWKFILTRST` reader - Remote Wake-Up Packet Filter Register Pointer Reset When this bit is set, the remote wake-up packet filter register pointer is reset to 3'b000."]
pub struct RWKFILTRST_R(crate::FieldReader<bool, bool>);
impl RWKFILTRST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RWKFILTRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RWKFILTRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RWKFILTRST` writer - Remote Wake-Up Packet Filter Register Pointer Reset When this bit is set, the remote wake-up packet filter register pointer is reset to 3'b000."]
pub struct RWKFILTRST_W<'a> {
    w: &'a mut W,
}
impl<'a> RWKFILTRST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Transmit LPI Entry When this bit is set, it indicates that the MAC Transmitter has entered the LPI state because of the setting of the LPIEN bit."]
    #[inline(always)]
    pub fn pwrdwn(&self) -> PWRDWN_R {
        PWRDWN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Magic Packet Enable."]
    #[inline(always)]
    pub fn mgkpkten(&self) -> MGKPKTEN_R {
        MGKPKTEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Remote Wake-Up Packet Enable When this bit is set, a power management event is generated when the MAC receives a remote wake-up packet."]
    #[inline(always)]
    pub fn rwkpkten(&self) -> RWKPKTEN_R {
        RWKPKTEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Magic Packet Received."]
    #[inline(always)]
    pub fn mgkprcvd(&self) -> MGKPRCVD_R {
        MGKPRCVD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Remote Wake-Up Packet Received."]
    #[inline(always)]
    pub fn rwkprcvd(&self) -> RWKPRCVD_R {
        RWKPRCVD_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Global Unicast When this bit set, any unicast packet filtered by the MAC (DAF) address recognition is detected as a remote wake-up packet."]
    #[inline(always)]
    pub fn glblucast(&self) -> GLBLUCAST_R {
        GLBLUCAST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Remote Wake-up Packet Forwarding Enable When this bit is set along with RWKPKTEN, the MAC receiver drops all received frames until it receives the expected wake-up frame."]
    #[inline(always)]
    pub fn rwkpfe(&self) -> RWKPFE_R {
        RWKPFE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 24:28 - Remote Wake-up FIFO Pointer This field gives the current value (0 to 7) of the Remote Wake-up Packet Filter register pointer."]
    #[inline(always)]
    pub fn rwkptr(&self) -> RWKPTR_R {
        RWKPTR_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - Remote Wake-Up Packet Filter Register Pointer Reset When this bit is set, the remote wake-up packet filter register pointer is reset to 3'b000."]
    #[inline(always)]
    pub fn rwkfiltrst(&self) -> RWKFILTRST_R {
        RWKFILTRST_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - Global Unicast When this bit set, any unicast packet filtered by the MAC (DAF) address recognition is detected as a remote wake-up packet."]
    #[inline(always)]
    pub fn glblucast(&mut self) -> GLBLUCAST_W {
        GLBLUCAST_W { w: self }
    }
    #[doc = "Bit 10 - Remote Wake-up Packet Forwarding Enable When this bit is set along with RWKPKTEN, the MAC receiver drops all received frames until it receives the expected wake-up frame."]
    #[inline(always)]
    pub fn rwkpfe(&mut self) -> RWKPFE_W {
        RWKPFE_W { w: self }
    }
    #[doc = "Bits 24:28 - Remote Wake-up FIFO Pointer This field gives the current value (0 to 7) of the Remote Wake-up Packet Filter register pointer."]
    #[inline(always)]
    pub fn rwkptr(&mut self) -> RWKPTR_W {
        RWKPTR_W { w: self }
    }
    #[doc = "Bit 31 - Remote Wake-Up Packet Filter Register Pointer Reset When this bit is set, the remote wake-up packet filter register pointer is reset to 3'b000."]
    #[inline(always)]
    pub fn rwkfiltrst(&mut self) -> RWKFILTRST_W {
        RWKFILTRST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_pmt_crtl_stat](index.html) module"]
pub struct MAC_PMT_CRTL_STAT_SPEC;
impl crate::RegisterSpec for MAC_PMT_CRTL_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_pmt_crtl_stat::R](R) reader structure"]
impl crate::Readable for MAC_PMT_CRTL_STAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mac_pmt_crtl_stat::W](W) writer structure"]
impl crate::Writable for MAC_PMT_CRTL_STAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAC_PMT_CRTL_STAT to value 0"]
impl crate::Resettable for MAC_PMT_CRTL_STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
