#[doc = "Register `CRSR_CTRL` reader"]
pub struct R(crate::R<CRSR_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRSR_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRSR_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRSR_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRSR_CTRL` writer"]
pub struct W(crate::W<CRSR_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRSR_CTRL_SPEC>;
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
impl From<crate::W<CRSR_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRSR_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRSRON` reader - Cursor enable."]
pub struct CRSRON_R(crate::FieldReader<bool, bool>);
impl CRSRON_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRSRON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRSRON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRSRON` writer - Cursor enable."]
pub struct CRSRON_W<'a> {
    w: &'a mut W,
}
impl<'a> CRSRON_W<'a> {
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
#[doc = "Field `CRSRNUM1_0` reader - Cursor image number."]
pub struct CRSRNUM1_0_R(crate::FieldReader<u8, u8>);
impl CRSRNUM1_0_R {
    pub(crate) fn new(bits: u8) -> Self {
        CRSRNUM1_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRSRNUM1_0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRSRNUM1_0` writer - Cursor image number."]
pub struct CRSRNUM1_0_W<'a> {
    w: &'a mut W,
}
impl<'a> CRSRNUM1_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Cursor enable."]
    #[inline(always)]
    pub fn crsron(&self) -> CRSRON_R {
        CRSRON_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Cursor image number."]
    #[inline(always)]
    pub fn crsrnum1_0(&self) -> CRSRNUM1_0_R {
        CRSRNUM1_0_R::new(((self.bits >> 4) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Cursor enable."]
    #[inline(always)]
    pub fn crsron(&mut self) -> CRSRON_W {
        CRSRON_W { w: self }
    }
    #[doc = "Bits 4:5 - Cursor image number."]
    #[inline(always)]
    pub fn crsrnum1_0(&mut self) -> CRSRNUM1_0_W {
        CRSRNUM1_0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cursor Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crsr_ctrl](index.html) module"]
pub struct CRSR_CTRL_SPEC;
impl crate::RegisterSpec for CRSR_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crsr_ctrl::R](R) reader structure"]
impl crate::Readable for CRSR_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crsr_ctrl::W](W) writer structure"]
impl crate::Writable for CRSR_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRSR_CTRL to value 0"]
impl crate::Resettable for CRSR_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
