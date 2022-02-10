///Register `MASK` reader
pub struct R(crate::R<MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MASK_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MASK` writer
pub struct W(crate::W<MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MASK_SPEC>;
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
impl From<crate::W<MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MASK_SPEC>) -> Self {
        W(writer)
    }
}
///Field `RIMASK` reader - Mask register.
pub struct RIMASK_R(crate::FieldReader<u32, u32>);
impl RIMASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        RIMASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RIMASK_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RIMASK` writer - Mask register.
pub struct RIMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> RIMASK_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    ///Bits 0:31 - Mask register.
    #[inline(always)]
    pub fn rimask(&self) -> RIMASK_R {
        RIMASK_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Mask register.
    #[inline(always)]
    pub fn rimask(&mut self) -> RIMASK_W {
        RIMASK_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Mask LSB register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mask](index.html) module
pub struct MASK_SPEC;
impl crate::RegisterSpec for MASK_SPEC {
    type Ux = u32;
}
///`read()` method returns [mask::R](R) reader structure
impl crate::Readable for MASK_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [mask::W](W) writer structure
impl crate::Writable for MASK_SPEC {
    type Writer = W;
}
///`reset()` method sets MASK to value 0
impl crate::Resettable for MASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
