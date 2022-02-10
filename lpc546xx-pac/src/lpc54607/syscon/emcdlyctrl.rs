///Register `EMCDLYCTRL` reader
pub struct R(crate::R<EMCDLYCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMCDLYCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMCDLYCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMCDLYCTRL_SPEC>) -> Self {
        R(reader)
    }
}
///Register `EMCDLYCTRL` writer
pub struct W(crate::W<EMCDLYCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EMCDLYCTRL_SPEC>;
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
impl From<crate::W<EMCDLYCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EMCDLYCTRL_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CMD_DELAY` reader - Programmable delay value for EMC outputs in command delayed mode.
pub struct CMD_DELAY_R(crate::FieldReader<u8, u8>);
impl CMD_DELAY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CMD_DELAY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMD_DELAY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CMD_DELAY` writer - Programmable delay value for EMC outputs in command delayed mode.
pub struct CMD_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_DELAY_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
///Field `FBCLK_DELAY` reader - Programmable delay value for the feedback clock that controls input data sampling.
pub struct FBCLK_DELAY_R(crate::FieldReader<u8, u8>);
impl FBCLK_DELAY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FBCLK_DELAY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FBCLK_DELAY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `FBCLK_DELAY` writer - Programmable delay value for the feedback clock that controls input data sampling.
pub struct FBCLK_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> FBCLK_DELAY_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | ((value as u32 & 0x1f) << 8);
        self.w
    }
}
impl R {
    ///Bits 0:4 - Programmable delay value for EMC outputs in command delayed mode.
    #[inline(always)]
    pub fn cmd_delay(&self) -> CMD_DELAY_R {
        CMD_DELAY_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 8:12 - Programmable delay value for the feedback clock that controls input data sampling.
    #[inline(always)]
    pub fn fbclk_delay(&self) -> FBCLK_DELAY_R {
        FBCLK_DELAY_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    ///Bits 0:4 - Programmable delay value for EMC outputs in command delayed mode.
    #[inline(always)]
    pub fn cmd_delay(&mut self) -> CMD_DELAY_W {
        CMD_DELAY_W { w: self }
    }
    ///Bits 8:12 - Programmable delay value for the feedback clock that controls input data sampling.
    #[inline(always)]
    pub fn fbclk_delay(&mut self) -> FBCLK_DELAY_W {
        FBCLK_DELAY_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///EMC clock delay control
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [emcdlyctrl](index.html) module
pub struct EMCDLYCTRL_SPEC;
impl crate::RegisterSpec for EMCDLYCTRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [emcdlyctrl::R](R) reader structure
impl crate::Readable for EMCDLYCTRL_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [emcdlyctrl::W](W) writer structure
impl crate::Writable for EMCDLYCTRL_SPEC {
    type Writer = W;
}
///`reset()` method sets EMCDLYCTRL to value 0x0210
impl crate::Resettable for EMCDLYCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0210
    }
}
