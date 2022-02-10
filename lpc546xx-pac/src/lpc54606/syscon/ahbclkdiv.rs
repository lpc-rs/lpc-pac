///Register `AHBCLKDIV` reader
pub struct R(crate::R<AHBCLKDIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHBCLKDIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHBCLKDIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHBCLKDIV_SPEC>) -> Self {
        R(reader)
    }
}
///Register `AHBCLKDIV` writer
pub struct W(crate::W<AHBCLKDIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHBCLKDIV_SPEC>;
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
impl From<crate::W<AHBCLKDIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHBCLKDIV_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DIV` reader - Clock divider value. 0: Divide by 1 up to 255: Divide by 256.
pub struct DIV_R(crate::FieldReader<u8, u8>);
impl DIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DIV` writer - Clock divider value. 0: Divide by 1 up to 255: Divide by 256.
pub struct DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
///Field `REQFLAG` reader - Divider status flag.
pub struct REQFLAG_R(crate::FieldReader<bool, bool>);
impl REQFLAG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REQFLAG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REQFLAG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `REQFLAG` writer - Divider status flag.
pub struct REQFLAG_W<'a> {
    w: &'a mut W,
}
impl<'a> REQFLAG_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    ///Bits 0:7 - Clock divider value. 0: Divide by 1 up to 255: Divide by 256.
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new((self.bits & 0xff) as u8)
    }
    ///Bit 31 - Divider status flag.
    #[inline(always)]
    pub fn reqflag(&self) -> REQFLAG_R {
        REQFLAG_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    ///Bits 0:7 - Clock divider value. 0: Divide by 1 up to 255: Divide by 256.
    #[inline(always)]
    pub fn div(&mut self) -> DIV_W {
        DIV_W { w: self }
    }
    ///Bit 31 - Divider status flag.
    #[inline(always)]
    pub fn reqflag(&mut self) -> REQFLAG_W {
        REQFLAG_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///AHB clock divider
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ahbclkdiv](index.html) module
pub struct AHBCLKDIV_SPEC;
impl crate::RegisterSpec for AHBCLKDIV_SPEC {
    type Ux = u32;
}
///`read()` method returns [ahbclkdiv::R](R) reader structure
impl crate::Readable for AHBCLKDIV_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ahbclkdiv::W](W) writer structure
impl crate::Writable for AHBCLKDIV_SPEC {
    type Writer = W;
}
///`reset()` method sets AHBCLKDIV to value 0
impl crate::Resettable for AHBCLKDIV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
