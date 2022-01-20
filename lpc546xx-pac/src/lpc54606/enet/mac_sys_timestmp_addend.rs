#[doc = "Register `MAC_SYS_TIMESTMP_ADDEND` reader"]
pub struct R(crate::R<MAC_SYS_TIMESTMP_ADDEND_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_SYS_TIMESTMP_ADDEND_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_SYS_TIMESTMP_ADDEND_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_SYS_TIMESTMP_ADDEND_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAC_SYS_TIMESTMP_ADDEND` writer"]
pub struct W(crate::W<MAC_SYS_TIMESTMP_ADDEND_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_SYS_TIMESTMP_ADDEND_SPEC>;
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
impl From<crate::W<MAC_SYS_TIMESTMP_ADDEND_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC_SYS_TIMESTMP_ADDEND_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSAR` reader - Time stamp addend This register indicates the 32-bit time value to be added to the Accumulator register to achieve time synchronization."]
pub struct TSAR_R(crate::FieldReader<u32, u32>);
impl TSAR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        TSAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSAR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSAR` writer - Time stamp addend This register indicates the 32-bit time value to be added to the Accumulator register to achieve time synchronization."]
pub struct TSAR_W<'a> {
    w: &'a mut W,
}
impl<'a> TSAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Time stamp addend This register indicates the 32-bit time value to be added to the Accumulator register to achieve time synchronization."]
    #[inline(always)]
    pub fn tsar(&self) -> TSAR_R {
        TSAR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Time stamp addend This register indicates the 32-bit time value to be added to the Accumulator register to achieve time synchronization."]
    #[inline(always)]
    pub fn tsar(&mut self) -> TSAR_W {
        TSAR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Time stamp addend register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_sys_timestmp_addend](index.html) module"]
pub struct MAC_SYS_TIMESTMP_ADDEND_SPEC;
impl crate::RegisterSpec for MAC_SYS_TIMESTMP_ADDEND_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_sys_timestmp_addend::R](R) reader structure"]
impl crate::Readable for MAC_SYS_TIMESTMP_ADDEND_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mac_sys_timestmp_addend::W](W) writer structure"]
impl crate::Writable for MAC_SYS_TIMESTMP_ADDEND_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAC_SYS_TIMESTMP_ADDEND to value 0"]
impl crate::Resettable for MAC_SYS_TIMESTMP_ADDEND_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
