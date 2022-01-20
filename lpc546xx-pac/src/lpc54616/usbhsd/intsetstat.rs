#[doc = "Register `INTSETSTAT` reader"]
pub struct R(crate::R<INTSETSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTSETSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTSETSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTSETSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTSETSTAT` writer"]
pub struct W(crate::W<INTSETSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTSETSTAT_SPEC>;
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
impl From<crate::W<INTSETSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTSETSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EP_SET_INT` reader - If software writes a one to one of these bits, the corresponding USB interrupt status bit is set."]
pub struct EP_SET_INT_R(crate::FieldReader<u16, u16>);
impl EP_SET_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        EP_SET_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP_SET_INT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP_SET_INT` writer - If software writes a one to one of these bits, the corresponding USB interrupt status bit is set."]
pub struct EP_SET_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_SET_INT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
#[doc = "Field `FRAME_SET_INT` reader - If software writes a one to one of these bits, the corresponding USB interrupt status bit is set."]
pub struct FRAME_SET_INT_R(crate::FieldReader<bool, bool>);
impl FRAME_SET_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FRAME_SET_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRAME_SET_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRAME_SET_INT` writer - If software writes a one to one of these bits, the corresponding USB interrupt status bit is set."]
pub struct FRAME_SET_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAME_SET_INT_W<'a> {
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
#[doc = "Field `DEV_SET_INT` reader - If software writes a one to one of these bits, the corresponding USB interrupt status bit is set."]
pub struct DEV_SET_INT_R(crate::FieldReader<bool, bool>);
impl DEV_SET_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DEV_SET_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEV_SET_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEV_SET_INT` writer - If software writes a one to one of these bits, the corresponding USB interrupt status bit is set."]
pub struct DEV_SET_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> DEV_SET_INT_W<'a> {
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
    #[doc = "Bits 0:11 - If software writes a one to one of these bits, the corresponding USB interrupt status bit is set."]
    #[inline(always)]
    pub fn ep_set_int(&self) -> EP_SET_INT_R {
        EP_SET_INT_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 30 - If software writes a one to one of these bits, the corresponding USB interrupt status bit is set."]
    #[inline(always)]
    pub fn frame_set_int(&self) -> FRAME_SET_INT_R {
        FRAME_SET_INT_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - If software writes a one to one of these bits, the corresponding USB interrupt status bit is set."]
    #[inline(always)]
    pub fn dev_set_int(&self) -> DEV_SET_INT_R {
        DEV_SET_INT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - If software writes a one to one of these bits, the corresponding USB interrupt status bit is set."]
    #[inline(always)]
    pub fn ep_set_int(&mut self) -> EP_SET_INT_W {
        EP_SET_INT_W { w: self }
    }
    #[doc = "Bit 30 - If software writes a one to one of these bits, the corresponding USB interrupt status bit is set."]
    #[inline(always)]
    pub fn frame_set_int(&mut self) -> FRAME_SET_INT_W {
        FRAME_SET_INT_W { w: self }
    }
    #[doc = "Bit 31 - If software writes a one to one of these bits, the corresponding USB interrupt status bit is set."]
    #[inline(always)]
    pub fn dev_set_int(&mut self) -> DEV_SET_INT_W {
        DEV_SET_INT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB set interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intsetstat](index.html) module"]
pub struct INTSETSTAT_SPEC;
impl crate::RegisterSpec for INTSETSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intsetstat::R](R) reader structure"]
impl crate::Readable for INTSETSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intsetstat::W](W) writer structure"]
impl crate::Writable for INTSETSTAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTSETSTAT to value 0"]
impl crate::Resettable for INTSETSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
