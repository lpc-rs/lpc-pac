///Register `DIVHFCLK` reader
pub struct R(crate::R<DIVHFCLK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIVHFCLK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIVHFCLK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIVHFCLK_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DIVHFCLK` writer
pub struct W(crate::W<DIVHFCLK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIVHFCLK_SPEC>;
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
impl From<crate::W<DIVHFCLK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIVHFCLK_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PDMDIV` reader - PDM clock divider value. 0 = divide by 1 1 = divide by 2 2 = divide by 3 3 = divide by 4 4 = divide by 6 5 = divide by 8 6 = divide by 12 7 = divide by 16 8 = divide by 24 9 = divide by 32 10 = divide by 48 11 = divide by 64 12 = divide by 96 13 = divide by 128 others = reserved.
pub struct PDMDIV_R(crate::FieldReader<u8, u8>);
impl PDMDIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PDMDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDMDIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PDMDIV` writer - PDM clock divider value. 0 = divide by 1 1 = divide by 2 2 = divide by 3 3 = divide by 4 4 = divide by 6 5 = divide by 8 6 = divide by 12 7 = divide by 16 8 = divide by 24 9 = divide by 32 10 = divide by 48 11 = divide by 64 12 = divide by 96 13 = divide by 128 others = reserved.
pub struct PDMDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> PDMDIV_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    ///Bits 0:3 - PDM clock divider value. 0 = divide by 1 1 = divide by 2 2 = divide by 3 3 = divide by 4 4 = divide by 6 5 = divide by 8 6 = divide by 12 7 = divide by 16 8 = divide by 24 9 = divide by 32 10 = divide by 48 11 = divide by 64 12 = divide by 96 13 = divide by 128 others = reserved.
    #[inline(always)]
    pub fn pdmdiv(&self) -> PDMDIV_R {
        PDMDIV_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    ///Bits 0:3 - PDM clock divider value. 0 = divide by 1 1 = divide by 2 2 = divide by 3 3 = divide by 4 4 = divide by 6 5 = divide by 8 6 = divide by 12 7 = divide by 16 8 = divide by 24 9 = divide by 32 10 = divide by 48 11 = divide by 64 12 = divide by 96 13 = divide by 128 others = reserved.
    #[inline(always)]
    pub fn pdmdiv(&mut self) -> PDMDIV_W {
        PDMDIV_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DMIC Clock Register 0
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [divhfclk](index.html) module
pub struct DIVHFCLK_SPEC;
impl crate::RegisterSpec for DIVHFCLK_SPEC {
    type Ux = u32;
}
///`read()` method returns [divhfclk::R](R) reader structure
impl crate::Readable for DIVHFCLK_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [divhfclk::W](W) writer structure
impl crate::Writable for DIVHFCLK_SPEC {
    type Writer = W;
}
///`reset()` method sets DIVHFCLK to value 0
impl crate::Resettable for DIVHFCLK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
