#[doc = "Register `LE` reader"]
pub struct R(crate::R<LE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LE` writer"]
pub struct W(crate::W<LE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LE_SPEC>;
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
impl From<crate::W<LE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LED` reader - Line-end delay."]
pub struct LED_R(crate::FieldReader<u8, u8>);
impl LED_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LED_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LED` writer - Line-end delay."]
pub struct LED_W<'a> {
    w: &'a mut W,
}
impl<'a> LED_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
#[doc = "Field `LEE` reader - LCD Line end enable."]
pub struct LEE_R(crate::FieldReader<bool, bool>);
impl LEE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LEE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LEE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LEE` writer - LCD Line end enable."]
pub struct LEE_W<'a> {
    w: &'a mut W,
}
impl<'a> LEE_W<'a> {
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
impl R {
    #[doc = "Bits 0:6 - Line-end delay."]
    #[inline(always)]
    pub fn led(&self) -> LED_R {
        LED_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 16 - LCD Line end enable."]
    #[inline(always)]
    pub fn lee(&self) -> LEE_R {
        LEE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Line-end delay."]
    #[inline(always)]
    pub fn led(&mut self) -> LED_W {
        LED_W { w: self }
    }
    #[doc = "Bit 16 - LCD Line end enable."]
    #[inline(always)]
    pub fn lee(&mut self) -> LEE_W {
        LEE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Line End Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [le](index.html) module"]
pub struct LE_SPEC;
impl crate::RegisterSpec for LE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [le::R](R) reader structure"]
impl crate::Readable for LE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [le::W](W) writer structure"]
impl crate::Writable for LE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LE to value 0"]
impl crate::Resettable for LE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
