///Register `ETHPHYSEL` reader
pub struct R(crate::R<ETHPHYSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETHPHYSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETHPHYSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETHPHYSEL_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ETHPHYSEL` writer
pub struct W(crate::W<ETHPHYSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETHPHYSEL_SPEC>;
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
impl From<crate::W<ETHPHYSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETHPHYSEL_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PHY_SEL` reader - PHY interface select.
pub struct PHY_SEL_R(crate::FieldReader<bool, bool>);
impl PHY_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PHY_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PHY_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PHY_SEL` writer - PHY interface select.
pub struct PHY_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PHY_SEL_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
impl R {
    ///Bit 2 - PHY interface select.
    #[inline(always)]
    pub fn phy_sel(&self) -> PHY_SEL_R {
        PHY_SEL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    ///Bit 2 - PHY interface select.
    #[inline(always)]
    pub fn phy_sel(&mut self) -> PHY_SEL_W {
        PHY_SEL_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Ethernet PHY Selection
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ethphysel](index.html) module
pub struct ETHPHYSEL_SPEC;
impl crate::RegisterSpec for ETHPHYSEL_SPEC {
    type Ux = u32;
}
///`read()` method returns [ethphysel::R](R) reader structure
impl crate::Readable for ETHPHYSEL_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ethphysel::W](W) writer structure
impl crate::Writable for ETHPHYSEL_SPEC {
    type Writer = W;
}
///`reset()` method sets ETHPHYSEL to value 0
impl crate::Resettable for ETHPHYSEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
