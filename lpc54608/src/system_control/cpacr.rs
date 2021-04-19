#[doc = "Register `CPACR` reader"]
pub struct R(crate::R<CPACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CPACR_SPEC>> for R {
    fn from(reader: crate::R<CPACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPACR` writer"]
pub struct W(crate::W<CPACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPACR_SPEC>;
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
impl core::convert::From<crate::W<CPACR_SPEC>> for W {
    fn from(writer: crate::W<CPACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Access privileges for coprocessor 10.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CP10_A {
    #[doc = "0: Access denied. Any attempted access generates a NOCP UsageFault"]
    CP10_0 = 0,
    #[doc = "1: Privileged access only. An unprivileged access generates a NOCP fault."]
    CP10_1 = 1,
    #[doc = "3: Full access."]
    CP10_3 = 3,
}
impl From<CP10_A> for u8 {
    #[inline(always)]
    fn from(variant: CP10_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CP10` reader - Access privileges for coprocessor 10."]
pub struct CP10_R(crate::FieldReader<u8, CP10_A>);
impl CP10_R {
    pub(crate) fn new(bits: u8) -> Self {
        CP10_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CP10_A> {
        match self.bits {
            0 => Some(CP10_A::CP10_0),
            1 => Some(CP10_A::CP10_1),
            3 => Some(CP10_A::CP10_3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CP10_0`"]
    #[inline(always)]
    pub fn is_cp10_0(&self) -> bool {
        **self == CP10_A::CP10_0
    }
    #[doc = "Checks if the value of the field is `CP10_1`"]
    #[inline(always)]
    pub fn is_cp10_1(&self) -> bool {
        **self == CP10_A::CP10_1
    }
    #[doc = "Checks if the value of the field is `CP10_3`"]
    #[inline(always)]
    pub fn is_cp10_3(&self) -> bool {
        **self == CP10_A::CP10_3
    }
}
impl core::ops::Deref for CP10_R {
    type Target = crate::FieldReader<u8, CP10_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CP10` writer - Access privileges for coprocessor 10."]
pub struct CP10_W<'a> {
    w: &'a mut W,
}
impl<'a> CP10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CP10_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault"]
    #[inline(always)]
    pub fn cp10_0(self) -> &'a mut W {
        self.variant(CP10_A::CP10_0)
    }
    #[doc = "Privileged access only. An unprivileged access generates a NOCP fault."]
    #[inline(always)]
    pub fn cp10_1(self) -> &'a mut W {
        self.variant(CP10_A::CP10_1)
    }
    #[doc = "Full access."]
    #[inline(always)]
    pub fn cp10_3(self) -> &'a mut W {
        self.variant(CP10_A::CP10_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "Access privileges for coprocessor 11.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CP11_A {
    #[doc = "0: Access denied. Any attempted access generates a NOCP UsageFault"]
    CP11_0 = 0,
    #[doc = "1: Privileged access only. An unprivileged access generates a NOCP fault."]
    CP11_1 = 1,
    #[doc = "3: Full access."]
    CP11_3 = 3,
}
impl From<CP11_A> for u8 {
    #[inline(always)]
    fn from(variant: CP11_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CP11` reader - Access privileges for coprocessor 11."]
pub struct CP11_R(crate::FieldReader<u8, CP11_A>);
impl CP11_R {
    pub(crate) fn new(bits: u8) -> Self {
        CP11_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CP11_A> {
        match self.bits {
            0 => Some(CP11_A::CP11_0),
            1 => Some(CP11_A::CP11_1),
            3 => Some(CP11_A::CP11_3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CP11_0`"]
    #[inline(always)]
    pub fn is_cp11_0(&self) -> bool {
        **self == CP11_A::CP11_0
    }
    #[doc = "Checks if the value of the field is `CP11_1`"]
    #[inline(always)]
    pub fn is_cp11_1(&self) -> bool {
        **self == CP11_A::CP11_1
    }
    #[doc = "Checks if the value of the field is `CP11_3`"]
    #[inline(always)]
    pub fn is_cp11_3(&self) -> bool {
        **self == CP11_A::CP11_3
    }
}
impl core::ops::Deref for CP11_R {
    type Target = crate::FieldReader<u8, CP11_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CP11` writer - Access privileges for coprocessor 11."]
pub struct CP11_W<'a> {
    w: &'a mut W,
}
impl<'a> CP11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CP11_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault"]
    #[inline(always)]
    pub fn cp11_0(self) -> &'a mut W {
        self.variant(CP11_A::CP11_0)
    }
    #[doc = "Privileged access only. An unprivileged access generates a NOCP fault."]
    #[inline(always)]
    pub fn cp11_1(self) -> &'a mut W {
        self.variant(CP11_A::CP11_1)
    }
    #[doc = "Full access."]
    #[inline(always)]
    pub fn cp11_3(self) -> &'a mut W {
        self.variant(CP11_A::CP11_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
        self.w
    }
}
impl R {
    #[doc = "Bits 20:21 - Access privileges for coprocessor 10."]
    #[inline(always)]
    pub fn cp10(&self) -> CP10_R {
        CP10_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Access privileges for coprocessor 11."]
    #[inline(always)]
    pub fn cp11(&self) -> CP11_R {
        CP11_R::new(((self.bits >> 22) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 20:21 - Access privileges for coprocessor 10."]
    #[inline(always)]
    pub fn cp10(&mut self) -> CP10_W {
        CP10_W { w: self }
    }
    #[doc = "Bits 22:23 - Access privileges for coprocessor 11."]
    #[inline(always)]
    pub fn cp11(&mut self) -> CP11_W {
        CP11_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Coprocessor Access Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpacr](index.html) module"]
pub struct CPACR_SPEC;
impl crate::RegisterSpec for CPACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpacr::R](R) reader structure"]
impl crate::Readable for CPACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpacr::W](W) writer structure"]
impl crate::Writable for CPACR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CPACR to value 0"]
impl crate::Resettable for CPACR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
