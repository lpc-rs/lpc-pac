///Register `CONFIG` reader
pub struct R(crate::R<CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CONFIG` writer
pub struct W(crate::W<CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONFIG_SPEC>;
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
impl From<crate::W<CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
///Field `EM` reader - Endian mode.
pub struct EM_R(crate::FieldReader<bool, bool>);
impl EM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `EM` writer - Endian mode.
pub struct EM_W<'a> {
    w: &'a mut W,
}
impl<'a> EM_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
///Field `CLKR` reader - This bit must contain 0 for proper operation of the EMC.
pub struct CLKR_R(crate::FieldReader<bool, bool>);
impl CLKR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLKR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CLKR` writer - This bit must contain 0 for proper operation of the EMC.
pub struct CLKR_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
impl R {
    ///Bit 0 - Endian mode.
    #[inline(always)]
    pub fn em(&self) -> EM_R {
        EM_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 8 - This bit must contain 0 for proper operation of the EMC.
    #[inline(always)]
    pub fn clkr(&self) -> CLKR_R {
        CLKR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - Endian mode.
    #[inline(always)]
    pub fn em(&mut self) -> EM_W {
        EM_W { w: self }
    }
    ///Bit 8 - This bit must contain 0 for proper operation of the EMC.
    #[inline(always)]
    pub fn clkr(&mut self) -> CLKR_W {
        CLKR_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Configures operation of the memory controller
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [config](index.html) module
pub struct CONFIG_SPEC;
impl crate::RegisterSpec for CONFIG_SPEC {
    type Ux = u32;
}
///`read()` method returns [config::R](R) reader structure
impl crate::Readable for CONFIG_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [config::W](W) writer structure
impl crate::Writable for CONFIG_SPEC {
    type Writer = W;
}
///`reset()` method sets CONFIG to value 0
impl crate::Resettable for CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
