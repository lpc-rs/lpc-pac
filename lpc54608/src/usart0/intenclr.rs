#[doc = "Register `INTENCLR` writer"]
pub struct W(crate::W<INTENCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTENCLR_SPEC>;
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
impl core::convert::From<crate::W<INTENCLR_SPEC>> for W {
    fn from(writer: crate::W<INTENCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXIDLECLR` writer - Writing 1 clears the corresponding bit in the INTENSET register."]
pub struct TXIDLECLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TXIDLECLR_W<'a> {
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
#[doc = "Field `DELTACTSCLR` writer - Writing 1 clears the corresponding bit in the INTENSET register."]
pub struct DELTACTSCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DELTACTSCLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `TXDISCLR` writer - Writing 1 clears the corresponding bit in the INTENSET register."]
pub struct TXDISCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDISCLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `DELTARXBRKCLR` writer - Writing 1 clears the corresponding bit in the INTENSET register."]
pub struct DELTARXBRKCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DELTARXBRKCLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `STARTCLR` writer - Writing 1 clears the corresponding bit in the INTENSET register."]
pub struct STARTCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> STARTCLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `FRAMERRCLR` writer - Writing 1 clears the corresponding bit in the INTENSET register."]
pub struct FRAMERRCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAMERRCLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `PARITYERRCLR` writer - Writing 1 clears the corresponding bit in the INTENSET register."]
pub struct PARITYERRCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> PARITYERRCLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `RXNOISECLR` writer - Writing 1 clears the corresponding bit in the INTENSET register."]
pub struct RXNOISECLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RXNOISECLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `ABERRCLR` writer - Writing 1 clears the corresponding bit in the INTENSET register."]
pub struct ABERRCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> ABERRCLR_W<'a> {
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
impl W {
    #[doc = "Bit 3 - Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    pub fn txidleclr(&mut self) -> TXIDLECLR_W {
        TXIDLECLR_W { w: self }
    }
    #[doc = "Bit 5 - Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    pub fn deltactsclr(&mut self) -> DELTACTSCLR_W {
        DELTACTSCLR_W { w: self }
    }
    #[doc = "Bit 6 - Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    pub fn txdisclr(&mut self) -> TXDISCLR_W {
        TXDISCLR_W { w: self }
    }
    #[doc = "Bit 11 - Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    pub fn deltarxbrkclr(&mut self) -> DELTARXBRKCLR_W {
        DELTARXBRKCLR_W { w: self }
    }
    #[doc = "Bit 12 - Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    pub fn startclr(&mut self) -> STARTCLR_W {
        STARTCLR_W { w: self }
    }
    #[doc = "Bit 13 - Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    pub fn framerrclr(&mut self) -> FRAMERRCLR_W {
        FRAMERRCLR_W { w: self }
    }
    #[doc = "Bit 14 - Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    pub fn parityerrclr(&mut self) -> PARITYERRCLR_W {
        PARITYERRCLR_W { w: self }
    }
    #[doc = "Bit 15 - Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    pub fn rxnoiseclr(&mut self) -> RXNOISECLR_W {
        RXNOISECLR_W { w: self }
    }
    #[doc = "Bit 16 - Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    pub fn aberrclr(&mut self) -> ABERRCLR_W {
        ABERRCLR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Clear register. Allows clearing any combination of bits in the INTENSET register. Writing a 1 to any implemented bit position causes the corresponding bit to be cleared.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenclr](index.html) module"]
pub struct INTENCLR_SPEC;
impl crate::RegisterSpec for INTENCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [intenclr::W](W) writer structure"]
impl crate::Writable for INTENCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for INTENCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
