#[doc = "Register `DMAREQ0` reader"]
pub struct R(crate::R<DMAREQ0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAREQ0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<DMAREQ0_SPEC>> for R {
    fn from(reader: crate::R<DMAREQ0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMAREQ0` writer"]
pub struct W(crate::W<DMAREQ0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAREQ0_SPEC>;
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
impl core::convert::From<crate::W<DMAREQ0_SPEC>> for W {
    fn from(writer: crate::W<DMAREQ0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DEV_0` reader - If bit n is one, event n triggers DMA request 0 (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
pub struct DEV_0_R(crate::FieldReader<u16, u16>);
impl DEV_0_R {
    pub(crate) fn new(bits: u16) -> Self {
        DEV_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEV_0_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEV_0` writer - If bit n is one, event n triggers DMA request 0 (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
pub struct DEV_0_W<'a> {
    w: &'a mut W,
}
impl<'a> DEV_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `DRL0` reader - A 1 in this bit triggers DMA request 0 when it loads the MATCH_L/Unified registers from the RELOAD_L/Unified registers."]
pub struct DRL0_R(crate::FieldReader<bool, bool>);
impl DRL0_R {
    pub(crate) fn new(bits: bool) -> Self {
        DRL0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DRL0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DRL0` writer - A 1 in this bit triggers DMA request 0 when it loads the MATCH_L/Unified registers from the RELOAD_L/Unified registers."]
pub struct DRL0_W<'a> {
    w: &'a mut W,
}
impl<'a> DRL0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `DRQ0` reader - This read-only bit indicates the state of DMA Request 0. Note that if the related DMA channel is enabled and properly set up, it is unlikely that software will see this flag, it will be cleared rapidly by the DMA service. The flag remaining set could point to an issue with DMA setup."]
pub struct DRQ0_R(crate::FieldReader<bool, bool>);
impl DRQ0_R {
    pub(crate) fn new(bits: bool) -> Self {
        DRQ0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DRQ0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DRQ0` writer - This read-only bit indicates the state of DMA Request 0. Note that if the related DMA channel is enabled and properly set up, it is unlikely that software will see this flag, it will be cleared rapidly by the DMA service. The flag remaining set could point to an issue with DMA setup."]
pub struct DRQ0_W<'a> {
    w: &'a mut W,
}
impl<'a> DRQ0_W<'a> {
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
    #[doc = "Bits 0:15 - If bit n is one, event n triggers DMA request 0 (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub fn dev_0(&self) -> DEV_0_R {
        DEV_0_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 30 - A 1 in this bit triggers DMA request 0 when it loads the MATCH_L/Unified registers from the RELOAD_L/Unified registers."]
    #[inline(always)]
    pub fn drl0(&self) -> DRL0_R {
        DRL0_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - This read-only bit indicates the state of DMA Request 0. Note that if the related DMA channel is enabled and properly set up, it is unlikely that software will see this flag, it will be cleared rapidly by the DMA service. The flag remaining set could point to an issue with DMA setup."]
    #[inline(always)]
    pub fn drq0(&self) -> DRQ0_R {
        DRQ0_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - If bit n is one, event n triggers DMA request 0 (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub fn dev_0(&mut self) -> DEV_0_W {
        DEV_0_W { w: self }
    }
    #[doc = "Bit 30 - A 1 in this bit triggers DMA request 0 when it loads the MATCH_L/Unified registers from the RELOAD_L/Unified registers."]
    #[inline(always)]
    pub fn drl0(&mut self) -> DRL0_W {
        DRL0_W { w: self }
    }
    #[doc = "Bit 31 - This read-only bit indicates the state of DMA Request 0. Note that if the related DMA channel is enabled and properly set up, it is unlikely that software will see this flag, it will be cleared rapidly by the DMA service. The flag remaining set could point to an issue with DMA setup."]
    #[inline(always)]
    pub fn drq0(&mut self) -> DRQ0_W {
        DRQ0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SCT DMA request 0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmareq0](index.html) module"]
pub struct DMAREQ0_SPEC;
impl crate::RegisterSpec for DMAREQ0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmareq0::R](R) reader structure"]
impl crate::Readable for DMAREQ0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmareq0::W](W) writer structure"]
impl crate::Writable for DMAREQ0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMAREQ0 to value 0"]
impl crate::Resettable for DMAREQ0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
