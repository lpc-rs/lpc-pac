#[doc = "Register `AUDPLLFRAC` reader"]
pub struct R(crate::R<AUDPLLFRAC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AUDPLLFRAC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<AUDPLLFRAC_SPEC>> for R {
    fn from(reader: crate::R<AUDPLLFRAC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AUDPLLFRAC` writer"]
pub struct W(crate::W<AUDPLLFRAC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AUDPLLFRAC_SPEC>;
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
impl core::convert::From<crate::W<AUDPLLFRAC_SPEC>> for W {
    fn from(writer: crate::W<AUDPLLFRAC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTRL` reader - PLL fractional divider control word"]
pub struct CTRL_R(crate::FieldReader<u32, u32>);
impl CTRL_R {
    pub(crate) fn new(bits: u32) -> Self {
        CTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTRL_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTRL` writer - PLL fractional divider control word"]
pub struct CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x003f_ffff) | (value as u32 & 0x003f_ffff);
        self.w
    }
}
#[doc = "Field `REQ` reader - Writing 1 to REQ signal loads CTRL value into fractional wrapper modulator."]
pub struct REQ_R(crate::FieldReader<bool, bool>);
impl REQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        REQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REQ` writer - Writing 1 to REQ signal loads CTRL value into fractional wrapper modulator."]
pub struct REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> REQ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `SEL_EXT` reader - Select fractional divider."]
pub struct SEL_EXT_R(crate::FieldReader<bool, bool>);
impl SEL_EXT_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEL_EXT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEL_EXT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEL_EXT` writer - Select fractional divider."]
pub struct SEL_EXT_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_EXT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:21 - PLL fractional divider control word"]
    #[inline(always)]
    pub fn ctrl(&self) -> CTRL_R {
        CTRL_R::new((self.bits & 0x003f_ffff) as u32)
    }
    #[doc = "Bit 22 - Writing 1 to REQ signal loads CTRL value into fractional wrapper modulator."]
    #[inline(always)]
    pub fn req(&self) -> REQ_R {
        REQ_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Select fractional divider."]
    #[inline(always)]
    pub fn sel_ext(&self) -> SEL_EXT_R {
        SEL_EXT_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:21 - PLL fractional divider control word"]
    #[inline(always)]
    pub fn ctrl(&mut self) -> CTRL_W {
        CTRL_W { w: self }
    }
    #[doc = "Bit 22 - Writing 1 to REQ signal loads CTRL value into fractional wrapper modulator."]
    #[inline(always)]
    pub fn req(&mut self) -> REQ_W {
        REQ_W { w: self }
    }
    #[doc = "Bit 23 - Select fractional divider."]
    #[inline(always)]
    pub fn sel_ext(&mut self) -> SEL_EXT_W {
        SEL_EXT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Audio PLL fractional divider control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [audpllfrac](index.html) module"]
pub struct AUDPLLFRAC_SPEC;
impl crate::RegisterSpec for AUDPLLFRAC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [audpllfrac::R](R) reader structure"]
impl crate::Readable for AUDPLLFRAC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [audpllfrac::W](W) writer structure"]
impl crate::Writable for AUDPLLFRAC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AUDPLLFRAC to value 0"]
impl crate::Resettable for AUDPLLFRAC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
