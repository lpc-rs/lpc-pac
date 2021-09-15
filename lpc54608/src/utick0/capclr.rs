#[doc = "Register `CAPCLR` writer"]
pub struct W(crate::W<CAPCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAPCLR_SPEC>;
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
impl From<crate::W<CAPCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAPCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAPCLR0` writer - Clear capture 0. Writing 1 to this bit clears the CAP0 register value."]
pub struct CAPCLR0_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPCLR0_W<'a> {
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
#[doc = "Field `CAPCLR1` writer - Clear capture 1. Writing 1 to this bit clears the CAP1 register value."]
pub struct CAPCLR1_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPCLR1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `CAPCLR2` writer - Clear capture 2. Writing 1 to this bit clears the CAP2 register value."]
pub struct CAPCLR2_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPCLR2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `CAPCLR3` writer - Clear capture 3. Writing 1 to this bit clears the CAP3 register value."]
pub struct CAPCLR3_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPCLR3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Clear capture 0. Writing 1 to this bit clears the CAP0 register value."]
    #[inline(always)]
    pub fn capclr0(&mut self) -> CAPCLR0_W {
        CAPCLR0_W { w: self }
    }
    #[doc = "Bit 1 - Clear capture 1. Writing 1 to this bit clears the CAP1 register value."]
    #[inline(always)]
    pub fn capclr1(&mut self) -> CAPCLR1_W {
        CAPCLR1_W { w: self }
    }
    #[doc = "Bit 2 - Clear capture 2. Writing 1 to this bit clears the CAP2 register value."]
    #[inline(always)]
    pub fn capclr2(&mut self) -> CAPCLR2_W {
        CAPCLR2_W { w: self }
    }
    #[doc = "Bit 3 - Clear capture 3. Writing 1 to this bit clears the CAP3 register value."]
    #[inline(always)]
    pub fn capclr3(&mut self) -> CAPCLR3_W {
        CAPCLR3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Capture clear register.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [capclr](index.html) module"]
pub struct CAPCLR_SPEC;
impl crate::RegisterSpec for CAPCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [capclr::W](W) writer structure"]
impl crate::Writable for CAPCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CAPCLR to value 0"]
impl crate::Resettable for CAPCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
