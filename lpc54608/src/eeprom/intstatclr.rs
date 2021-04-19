#[doc = "Register `INTSTATCLR` writer"]
pub struct W(crate::W<INTSTATCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTSTATCLR_SPEC>;
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
impl core::convert::From<crate::W<INTSTATCLR_SPEC>> for W {
    fn from(writer: crate::W<INTSTATCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PROG_CLR_ST` writer - Clear program operation finished interrupt status bit for EEPROM device."]
pub struct PROG_CLR_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> PROG_CLR_ST_W<'a> {
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
impl W {
    #[doc = "Bit 2 - Clear program operation finished interrupt status bit for EEPROM device."]
    #[inline(always)]
    pub fn prog_clr_st(&mut self) -> PROG_CLR_ST_W {
        PROG_CLR_ST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EEPROM interrupt status clear\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intstatclr](index.html) module"]
pub struct INTSTATCLR_SPEC;
impl crate::RegisterSpec for INTSTATCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [intstatclr::W](W) writer structure"]
impl crate::Writable for INTSTATCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTSTATCLR to value 0"]
impl crate::Resettable for INTSTATCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
