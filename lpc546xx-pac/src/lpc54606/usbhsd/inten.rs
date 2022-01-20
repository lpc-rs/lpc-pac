#[doc = "Register `INTEN` reader"]
pub struct R(crate::R<INTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTEN` writer"]
pub struct W(crate::W<INTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTEN_SPEC>;
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
impl From<crate::W<INTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EP_INT_EN` reader - If this bit is set and the corresponding USB interrupt status bit is set, a HW interrupt is generated on the interrupt line."]
pub struct EP_INT_EN_R(crate::FieldReader<u16, u16>);
impl EP_INT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        EP_INT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP_INT_EN_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP_INT_EN` writer - If this bit is set and the corresponding USB interrupt status bit is set, a HW interrupt is generated on the interrupt line."]
pub struct EP_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_INT_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
#[doc = "Field `FRAME_INT_EN` reader - If this bit is set and the corresponding USB interrupt status bit is set, a HW interrupt is generated on the interrupt line."]
pub struct FRAME_INT_EN_R(crate::FieldReader<bool, bool>);
impl FRAME_INT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FRAME_INT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRAME_INT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRAME_INT_EN` writer - If this bit is set and the corresponding USB interrupt status bit is set, a HW interrupt is generated on the interrupt line."]
pub struct FRAME_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAME_INT_EN_W<'a> {
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
#[doc = "Field `DEV_INT_EN` reader - If this bit is set and the corresponding USB interrupt status bit is set, a HW interrupt is generated on the interrupt line."]
pub struct DEV_INT_EN_R(crate::FieldReader<bool, bool>);
impl DEV_INT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DEV_INT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEV_INT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEV_INT_EN` writer - If this bit is set and the corresponding USB interrupt status bit is set, a HW interrupt is generated on the interrupt line."]
pub struct DEV_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DEV_INT_EN_W<'a> {
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
    #[doc = "Bits 0:11 - If this bit is set and the corresponding USB interrupt status bit is set, a HW interrupt is generated on the interrupt line."]
    #[inline(always)]
    pub fn ep_int_en(&self) -> EP_INT_EN_R {
        EP_INT_EN_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 30 - If this bit is set and the corresponding USB interrupt status bit is set, a HW interrupt is generated on the interrupt line."]
    #[inline(always)]
    pub fn frame_int_en(&self) -> FRAME_INT_EN_R {
        FRAME_INT_EN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - If this bit is set and the corresponding USB interrupt status bit is set, a HW interrupt is generated on the interrupt line."]
    #[inline(always)]
    pub fn dev_int_en(&self) -> DEV_INT_EN_R {
        DEV_INT_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - If this bit is set and the corresponding USB interrupt status bit is set, a HW interrupt is generated on the interrupt line."]
    #[inline(always)]
    pub fn ep_int_en(&mut self) -> EP_INT_EN_W {
        EP_INT_EN_W { w: self }
    }
    #[doc = "Bit 30 - If this bit is set and the corresponding USB interrupt status bit is set, a HW interrupt is generated on the interrupt line."]
    #[inline(always)]
    pub fn frame_int_en(&mut self) -> FRAME_INT_EN_W {
        FRAME_INT_EN_W { w: self }
    }
    #[doc = "Bit 31 - If this bit is set and the corresponding USB interrupt status bit is set, a HW interrupt is generated on the interrupt line."]
    #[inline(always)]
    pub fn dev_int_en(&mut self) -> DEV_INT_EN_W {
        DEV_INT_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](index.html) module"]
pub struct INTEN_SPEC;
impl crate::RegisterSpec for INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inten::R](R) reader structure"]
impl crate::Readable for INTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inten::W](W) writer structure"]
impl crate::Writable for INTEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for INTEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
