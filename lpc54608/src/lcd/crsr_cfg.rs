#[doc = "Register `CRSR_CFG` reader"]
pub struct R(crate::R<CRSR_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRSR_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CRSR_CFG_SPEC>> for R {
    fn from(reader: crate::R<CRSR_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRSR_CFG` writer"]
pub struct W(crate::W<CRSR_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRSR_CFG_SPEC>;
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
impl core::convert::From<crate::W<CRSR_CFG_SPEC>> for W {
    fn from(writer: crate::W<CRSR_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRSRSIZE` reader - Cursor size selection."]
pub struct CRSRSIZE_R(crate::FieldReader<bool, bool>);
impl CRSRSIZE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRSRSIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRSRSIZE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRSRSIZE` writer - Cursor size selection."]
pub struct CRSRSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> CRSRSIZE_W<'a> {
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
#[doc = "Field `FRAMESYNC` reader - Cursor frame synchronization type."]
pub struct FRAMESYNC_R(crate::FieldReader<bool, bool>);
impl FRAMESYNC_R {
    pub(crate) fn new(bits: bool) -> Self {
        FRAMESYNC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRAMESYNC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRAMESYNC` writer - Cursor frame synchronization type."]
pub struct FRAMESYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAMESYNC_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Cursor size selection."]
    #[inline(always)]
    pub fn crsrsize(&self) -> CRSRSIZE_R {
        CRSRSIZE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Cursor frame synchronization type."]
    #[inline(always)]
    pub fn framesync(&self) -> FRAMESYNC_R {
        FRAMESYNC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Cursor size selection."]
    #[inline(always)]
    pub fn crsrsize(&mut self) -> CRSRSIZE_W {
        CRSRSIZE_W { w: self }
    }
    #[doc = "Bit 1 - Cursor frame synchronization type."]
    #[inline(always)]
    pub fn framesync(&mut self) -> FRAMESYNC_W {
        FRAMESYNC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cursor Configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crsr_cfg](index.html) module"]
pub struct CRSR_CFG_SPEC;
impl crate::RegisterSpec for CRSR_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crsr_cfg::R](R) reader structure"]
impl crate::Readable for CRSR_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crsr_cfg::W](W) writer structure"]
impl crate::Writable for CRSR_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRSR_CFG to value 0"]
impl crate::Resettable for CRSR_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
