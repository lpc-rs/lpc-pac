#[doc = "Register `MAC_ADDR_HIGH` reader"]
pub struct R(crate::R<MAC_ADDR_HIGH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_ADDR_HIGH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_ADDR_HIGH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_ADDR_HIGH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAC_ADDR_HIGH` writer"]
pub struct W(crate::W<MAC_ADDR_HIGH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_ADDR_HIGH_SPEC>;
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
impl From<crate::W<MAC_ADDR_HIGH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC_ADDR_HIGH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `A47_32` reader - MAC Address0 \\[47:32\\]
This field contains the upper 16 bits (47:32) of the 6-byte first MAC address."]
pub struct A47_32_R(crate::FieldReader<u16, u16>);
impl A47_32_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        A47_32_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for A47_32_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `A47_32` writer - MAC Address0 \\[47:32\\]
This field contains the upper 16 bits (47:32) of the 6-byte first MAC address."]
pub struct A47_32_W<'a> {
    w: &'a mut W,
}
impl<'a> A47_32_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `DCS` reader - DMA Channel Select This field contains the DMA Channel number to which the Rx packet whose DA matches the MAC Address content is routed."]
pub struct DCS_R(crate::FieldReader<bool, bool>);
impl DCS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DCS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCS` writer - DMA Channel Select This field contains the DMA Channel number to which the Rx packet whose DA matches the MAC Address content is routed."]
pub struct DCS_W<'a> {
    w: &'a mut W,
}
impl<'a> DCS_W<'a> {
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
#[doc = "Field `AE` reader - Address Enable."]
pub struct AE_R(crate::FieldReader<bool, bool>);
impl AE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - MAC Address0 \\[47:32\\]
This field contains the upper 16 bits (47:32) of the 6-byte first MAC address."]
    #[inline(always)]
    pub fn a47_32(&self) -> A47_32_R {
        A47_32_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - DMA Channel Select This field contains the DMA Channel number to which the Rx packet whose DA matches the MAC Address content is routed."]
    #[inline(always)]
    pub fn dcs(&self) -> DCS_R {
        DCS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Address Enable."]
    #[inline(always)]
    pub fn ae(&self) -> AE_R {
        AE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - MAC Address0 \\[47:32\\]
This field contains the upper 16 bits (47:32) of the 6-byte first MAC address."]
    #[inline(always)]
    pub fn a47_32(&mut self) -> A47_32_W {
        A47_32_W { w: self }
    }
    #[doc = "Bit 16 - DMA Channel Select This field contains the DMA Channel number to which the Rx packet whose DA matches the MAC Address content is routed."]
    #[inline(always)]
    pub fn dcs(&mut self) -> DCS_W {
        DCS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MAC address0 high register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_addr_high](index.html) module"]
pub struct MAC_ADDR_HIGH_SPEC;
impl crate::RegisterSpec for MAC_ADDR_HIGH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_addr_high::R](R) reader structure"]
impl crate::Readable for MAC_ADDR_HIGH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mac_addr_high::W](W) writer structure"]
impl crate::Writable for MAC_ADDR_HIGH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAC_ADDR_HIGH to value 0x8000_ffff"]
impl crate::Resettable for MAC_ADDR_HIGH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000_ffff
    }
}
