#[doc = "Register `ATL_PTD_SKIP_MAP` reader"]
pub struct R(crate::R<ATL_PTD_SKIP_MAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ATL_PTD_SKIP_MAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<ATL_PTD_SKIP_MAP_SPEC>> for R {
    fn from(reader: crate::R<ATL_PTD_SKIP_MAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ATL_PTD_SKIP_MAP` writer"]
pub struct W(crate::W<ATL_PTD_SKIP_MAP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ATL_PTD_SKIP_MAP_SPEC>;
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
impl core::convert::From<crate::W<ATL_PTD_SKIP_MAP_SPEC>> for W {
    fn from(writer: crate::W<ATL_PTD_SKIP_MAP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ATL_SKIP` reader - When a bit in the PTD Skip Map is set to logic 1, the corresponding PTD will be skipped, independent of the V bit setting."]
pub struct ATL_SKIP_R(crate::FieldReader<u32, u32>);
impl ATL_SKIP_R {
    pub(crate) fn new(bits: u32) -> Self {
        ATL_SKIP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ATL_SKIP_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ATL_SKIP` writer - When a bit in the PTD Skip Map is set to logic 1, the corresponding PTD will be skipped, independent of the V bit setting."]
pub struct ATL_SKIP_W<'a> {
    w: &'a mut W,
}
impl<'a> ATL_SKIP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - When a bit in the PTD Skip Map is set to logic 1, the corresponding PTD will be skipped, independent of the V bit setting."]
    #[inline(always)]
    pub fn atl_skip(&self) -> ATL_SKIP_R {
        ATL_SKIP_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - When a bit in the PTD Skip Map is set to logic 1, the corresponding PTD will be skipped, independent of the V bit setting."]
    #[inline(always)]
    pub fn atl_skip(&mut self) -> ATL_SKIP_W {
        ATL_SKIP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Skip map for each ATL PTD\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [atl_ptd_skip_map](index.html) module"]
pub struct ATL_PTD_SKIP_MAP_SPEC;
impl crate::RegisterSpec for ATL_PTD_SKIP_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [atl_ptd_skip_map::R](R) reader structure"]
impl crate::Readable for ATL_PTD_SKIP_MAP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [atl_ptd_skip_map::W](W) writer structure"]
impl crate::Writable for ATL_PTD_SKIP_MAP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ATL_PTD_SKIP_MAP to value 0"]
impl crate::Resettable for ATL_PTD_SKIP_MAP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
