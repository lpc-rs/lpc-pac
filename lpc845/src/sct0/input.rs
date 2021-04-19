#[doc = "Register `INPUT` reader"]
pub struct R(crate::R<INPUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INPUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<INPUT_SPEC>> for R {
    fn from(reader: crate::R<INPUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INPUT` writer"]
pub struct W(crate::W<INPUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INPUT_SPEC>;
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
impl core::convert::From<crate::W<INPUT_SPEC>> for W {
    fn from(writer: crate::W<INPUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AIN0` reader - Input 0 state. Input 0 state on the last SCT clock edge."]
pub struct AIN0_R(crate::FieldReader<bool, bool>);
impl AIN0_R {
    pub(crate) fn new(bits: bool) -> Self {
        AIN0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AIN0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AIN1` reader - Input 1 state. Input 1 state on the last SCT clock edge."]
pub struct AIN1_R(crate::FieldReader<bool, bool>);
impl AIN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        AIN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AIN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AIN2` reader - Input 2 state. Input 2 state on the last SCT clock edge."]
pub struct AIN2_R(crate::FieldReader<bool, bool>);
impl AIN2_R {
    pub(crate) fn new(bits: bool) -> Self {
        AIN2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AIN2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AIN3` reader - Input 3 state. Input 3 state on the last SCT clock edge."]
pub struct AIN3_R(crate::FieldReader<bool, bool>);
impl AIN3_R {
    pub(crate) fn new(bits: bool) -> Self {
        AIN3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AIN3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AIN4` reader - Input 4 state. Input 4 state on the last SCT clock edge."]
pub struct AIN4_R(crate::FieldReader<bool, bool>);
impl AIN4_R {
    pub(crate) fn new(bits: bool) -> Self {
        AIN4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AIN4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIN0` reader - Input 0 state. Input 0 state following the synchronization specified by INSYNC."]
pub struct SIN0_R(crate::FieldReader<bool, bool>);
impl SIN0_R {
    pub(crate) fn new(bits: bool) -> Self {
        SIN0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SIN0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIN1` reader - Input 1 state. Input 1 state following the synchronization specified by INSYNC."]
pub struct SIN1_R(crate::FieldReader<bool, bool>);
impl SIN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        SIN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SIN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIN2` reader - Input 2 state. Input 2 state following the synchronization specified by INSYNC."]
pub struct SIN2_R(crate::FieldReader<bool, bool>);
impl SIN2_R {
    pub(crate) fn new(bits: bool) -> Self {
        SIN2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SIN2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIN3` reader - Input 3 state. Input 3 state following the synchronization specified by INSYNC."]
pub struct SIN3_R(crate::FieldReader<bool, bool>);
impl SIN3_R {
    pub(crate) fn new(bits: bool) -> Self {
        SIN3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SIN3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIN4` reader - Input 4 state. Input 4 state following the synchronization specified by INSYNC."]
pub struct SIN4_R(crate::FieldReader<bool, bool>);
impl SIN4_R {
    pub(crate) fn new(bits: bool) -> Self {
        SIN4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SIN4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Input 0 state. Input 0 state on the last SCT clock edge."]
    #[inline(always)]
    pub fn ain0(&self) -> AIN0_R {
        AIN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Input 1 state. Input 1 state on the last SCT clock edge."]
    #[inline(always)]
    pub fn ain1(&self) -> AIN1_R {
        AIN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Input 2 state. Input 2 state on the last SCT clock edge."]
    #[inline(always)]
    pub fn ain2(&self) -> AIN2_R {
        AIN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Input 3 state. Input 3 state on the last SCT clock edge."]
    #[inline(always)]
    pub fn ain3(&self) -> AIN3_R {
        AIN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Input 4 state. Input 4 state on the last SCT clock edge."]
    #[inline(always)]
    pub fn ain4(&self) -> AIN4_R {
        AIN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Input 0 state. Input 0 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub fn sin0(&self) -> SIN0_R {
        SIN0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Input 1 state. Input 1 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub fn sin1(&self) -> SIN1_R {
        SIN1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Input 2 state. Input 2 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub fn sin2(&self) -> SIN2_R {
        SIN2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Input 3 state. Input 3 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub fn sin3(&self) -> SIN3_R {
        SIN3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Input 4 state. Input 4 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub fn sin4(&self) -> SIN4_R {
        SIN4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SCT input register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [input](index.html) module"]
pub struct INPUT_SPEC;
impl crate::RegisterSpec for INPUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [input::R](R) reader structure"]
impl crate::Readable for INPUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [input::W](W) writer structure"]
impl crate::Writable for INPUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INPUT to value 0"]
impl crate::Resettable for INPUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
