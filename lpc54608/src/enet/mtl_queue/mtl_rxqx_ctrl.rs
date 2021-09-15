#[doc = "Register `MTL_RXQx_CTRL` reader"]
pub struct R(crate::R<MTL_RXQX_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MTL_RXQX_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MTL_RXQX_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MTL_RXQX_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MTL_RXQx_CTRL` writer"]
pub struct W(crate::W<MTL_RXQX_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MTL_RXQX_CTRL_SPEC>;
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
impl From<crate::W<MTL_RXQX_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MTL_RXQX_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXQ_WEGT` reader - Receive Queue Weight This field indicates the weight assigned to the Rx Queue 0."]
pub struct RXQ_WEGT_R(crate::FieldReader<u8, u8>);
impl RXQ_WEGT_R {
    pub(crate) fn new(bits: u8) -> Self {
        RXQ_WEGT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXQ_WEGT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXQ_WEGT` writer - Receive Queue Weight This field indicates the weight assigned to the Rx Queue 0."]
pub struct RXQ_WEGT_W<'a> {
    w: &'a mut W,
}
impl<'a> RXQ_WEGT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Field `RXQ_FRM_ARBIT` reader - Receive Queue Packet Arbitration When this bit is set, the The ethernet block drives the packet data to the ARI interface such that the entire packet data of currently-selected queue is transmitted before switching to other queue."]
pub struct RXQ_FRM_ARBIT_R(crate::FieldReader<bool, bool>);
impl RXQ_FRM_ARBIT_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXQ_FRM_ARBIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXQ_FRM_ARBIT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXQ_FRM_ARBIT` writer - Receive Queue Packet Arbitration When this bit is set, the The ethernet block drives the packet data to the ARI interface such that the entire packet data of currently-selected queue is transmitted before switching to other queue."]
pub struct RXQ_FRM_ARBIT_W<'a> {
    w: &'a mut W,
}
impl<'a> RXQ_FRM_ARBIT_W<'a> {
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
impl R {
    #[doc = "Bits 0:2 - Receive Queue Weight This field indicates the weight assigned to the Rx Queue 0."]
    #[inline(always)]
    pub fn rxq_wegt(&self) -> RXQ_WEGT_R {
        RXQ_WEGT_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - Receive Queue Packet Arbitration When this bit is set, the The ethernet block drives the packet data to the ARI interface such that the entire packet data of currently-selected queue is transmitted before switching to other queue."]
    #[inline(always)]
    pub fn rxq_frm_arbit(&self) -> RXQ_FRM_ARBIT_R {
        RXQ_FRM_ARBIT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Receive Queue Weight This field indicates the weight assigned to the Rx Queue 0."]
    #[inline(always)]
    pub fn rxq_wegt(&mut self) -> RXQ_WEGT_W {
        RXQ_WEGT_W { w: self }
    }
    #[doc = "Bit 3 - Receive Queue Packet Arbitration When this bit is set, the The ethernet block drives the packet data to the ARI interface such that the entire packet data of currently-selected queue is transmitted before switching to other queue."]
    #[inline(always)]
    pub fn rxq_frm_arbit(&mut self) -> RXQ_FRM_ARBIT_W {
        RXQ_FRM_ARBIT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MTL RxQx Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtl_rxqx_ctrl](index.html) module"]
pub struct MTL_RXQX_CTRL_SPEC;
impl crate::RegisterSpec for MTL_RXQX_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mtl_rxqx_ctrl::R](R) reader structure"]
impl crate::Readable for MTL_RXQX_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mtl_rxqx_ctrl::W](W) writer structure"]
impl crate::Writable for MTL_RXQX_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MTL_RXQx_CTRL to value 0"]
impl crate::Resettable for MTL_RXQX_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
