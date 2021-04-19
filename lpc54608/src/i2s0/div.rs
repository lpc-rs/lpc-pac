#[doc = "Register `DIV` reader"]
pub struct R(crate::R<DIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<DIV_SPEC>> for R {
    fn from(reader: crate::R<DIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIV` writer"]
pub struct W(crate::W<DIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIV_SPEC>;
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
impl core::convert::From<crate::W<DIV_SPEC>> for W {
    fn from(writer: crate::W<DIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIV` reader - This field controls how this I2S block uses the Flexcomm function clock. 0x000 = The Flexcomm function clock is used directly. 0x001 = The Flexcomm function clock is divided by 2. 0x002 = The Flexcomm function clock is divided by 3. 0xFFF = The Flexcomm function clock is divided by 4,096."]
pub struct DIV_R(crate::FieldReader<u16, u16>);
impl DIV_R {
    pub(crate) fn new(bits: u16) -> Self {
        DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIV_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIV` writer - This field controls how this I2S block uses the Flexcomm function clock. 0x000 = The Flexcomm function clock is used directly. 0x001 = The Flexcomm function clock is divided by 2. 0x002 = The Flexcomm function clock is divided by 3. 0xFFF = The Flexcomm function clock is divided by 4,096."]
pub struct DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - This field controls how this I2S block uses the Flexcomm function clock. 0x000 = The Flexcomm function clock is used directly. 0x001 = The Flexcomm function clock is divided by 2. 0x002 = The Flexcomm function clock is divided by 3. 0xFFF = The Flexcomm function clock is divided by 4,096."]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - This field controls how this I2S block uses the Flexcomm function clock. 0x000 = The Flexcomm function clock is used directly. 0x001 = The Flexcomm function clock is divided by 2. 0x002 = The Flexcomm function clock is divided by 3. 0xFFF = The Flexcomm function clock is divided by 4,096."]
    #[inline(always)]
    pub fn div(&mut self) -> DIV_W {
        DIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock divider, used by all channel pairs.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [div](index.html) module"]
pub struct DIV_SPEC;
impl crate::RegisterSpec for DIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [div::R](R) reader structure"]
impl crate::Readable for DIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [div::W](W) writer structure"]
impl crate::Writable for DIV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DIV to value 0"]
impl crate::Resettable for DIV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
