#[doc = "Register `FLASHCFG` reader"]
pub struct R(crate::R<FLASHCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASHCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<FLASHCFG_SPEC>> for R {
    fn from(reader: crate::R<FLASHCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLASHCFG` writer"]
pub struct W(crate::W<FLASHCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASHCFG_SPEC>;
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
impl core::convert::From<crate::W<FLASHCFG_SPEC>> for W {
    fn from(writer: crate::W<FLASHCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Flash memory access time. FLASHTIM +1 is equal to the number of system clocks used for flash access.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLASHTIM_A {
    #[doc = "0: 1 system clock flash access time."]
    ONE_SYSTEM_CLOCK_FLASH_ACCESS = 0,
    #[doc = "1: 2 system clock flash access time."]
    TWO_SYSTEM_CLOCK_FLASH_ACCESS = 1,
}
impl From<FLASHTIM_A> for u8 {
    #[inline(always)]
    fn from(variant: FLASHTIM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FLASHTIM` reader - Flash memory access time. FLASHTIM +1 is equal to the number of system clocks used for flash access."]
pub struct FLASHTIM_R(crate::FieldReader<u8, FLASHTIM_A>);
impl FLASHTIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        FLASHTIM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FLASHTIM_A> {
        match self.bits {
            0 => Some(FLASHTIM_A::ONE_SYSTEM_CLOCK_FLASH_ACCESS),
            1 => Some(FLASHTIM_A::TWO_SYSTEM_CLOCK_FLASH_ACCESS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ONE_SYSTEM_CLOCK_FLASH_ACCESS`"]
    #[inline(always)]
    pub fn is_one_system_clock_flash_access(&self) -> bool {
        **self == FLASHTIM_A::ONE_SYSTEM_CLOCK_FLASH_ACCESS
    }
    #[doc = "Checks if the value of the field is `TWO_SYSTEM_CLOCK_FLASH_ACCESS`"]
    #[inline(always)]
    pub fn is_two_system_clock_flash_access(&self) -> bool {
        **self == FLASHTIM_A::TWO_SYSTEM_CLOCK_FLASH_ACCESS
    }
}
impl core::ops::Deref for FLASHTIM_R {
    type Target = crate::FieldReader<u8, FLASHTIM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASHTIM` writer - Flash memory access time. FLASHTIM +1 is equal to the number of system clocks used for flash access."]
pub struct FLASHTIM_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASHTIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLASHTIM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "1 system clock flash access time."]
    #[inline(always)]
    pub fn one_system_clock_flash_access(self) -> &'a mut W {
        self.variant(FLASHTIM_A::ONE_SYSTEM_CLOCK_FLASH_ACCESS)
    }
    #[doc = "2 system clock flash access time."]
    #[inline(always)]
    pub fn two_system_clock_flash_access(self) -> &'a mut W {
        self.variant(FLASHTIM_A::TWO_SYSTEM_CLOCK_FLASH_ACCESS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Flash memory access time. FLASHTIM +1 is equal to the number of system clocks used for flash access."]
    #[inline(always)]
    pub fn flashtim(&self) -> FLASHTIM_R {
        FLASHTIM_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Flash memory access time. FLASHTIM +1 is equal to the number of system clocks used for flash access."]
    #[inline(always)]
    pub fn flashtim(&mut self) -> FLASHTIM_W {
        FLASHTIM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flashcfg](index.html) module"]
pub struct FLASHCFG_SPEC;
impl crate::RegisterSpec for FLASHCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flashcfg::R](R) reader structure"]
impl crate::Readable for FLASHCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flashcfg::W](W) writer structure"]
impl crate::Writable for FLASHCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLASHCFG to value 0x01"]
impl crate::Resettable for FLASHCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
