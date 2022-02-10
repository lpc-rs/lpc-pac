///Register `MEMCTRL` reader
pub struct R(crate::R<MEMCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MEMCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MEMCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MEMCTRL_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MEMCTRL` writer
pub struct W(crate::W<MEMCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MEMCTRL_SPEC>;
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
impl From<crate::W<MEMCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MEMCTRL_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MASTER` reader - This field is used to enable SHA block as AHB bus master.
pub struct MASTER_R(crate::FieldReader<bool, bool>);
impl MASTER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MASTER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASTER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `MASTER` writer - This field is used to enable SHA block as AHB bus master.
pub struct MASTER_W<'a> {
    w: &'a mut W,
}
impl<'a> MASTER_W<'a> {
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
///Field `COUNT` reader - This field indicates the number of 512-bit blocks to copy starting at MEMADDR.
pub struct COUNT_R(crate::FieldReader<u16, u16>);
impl COUNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        COUNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COUNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `COUNT` writer - This field indicates the number of 512-bit blocks to copy starting at MEMADDR.
pub struct COUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> COUNT_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 16)) | ((value as u32 & 0x07ff) << 16);
        self.w
    }
}
impl R {
    ///Bit 0 - This field is used to enable SHA block as AHB bus master.
    #[inline(always)]
    pub fn master(&self) -> MASTER_R {
        MASTER_R::new((self.bits & 0x01) != 0)
    }
    ///Bits 16:26 - This field indicates the number of 512-bit blocks to copy starting at MEMADDR.
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    ///Bit 0 - This field is used to enable SHA block as AHB bus master.
    #[inline(always)]
    pub fn master(&mut self) -> MASTER_W {
        MASTER_W { w: self }
    }
    ///Bits 16:26 - This field indicates the number of 512-bit blocks to copy starting at MEMADDR.
    #[inline(always)]
    pub fn count(&mut self) -> COUNT_W {
        COUNT_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Memory Control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [memctrl](index.html) module
pub struct MEMCTRL_SPEC;
impl crate::RegisterSpec for MEMCTRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [memctrl::R](R) reader structure
impl crate::Readable for MEMCTRL_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [memctrl::W](W) writer structure
impl crate::Writable for MEMCTRL_SPEC {
    type Writer = W;
}
///`reset()` method sets MEMCTRL to value 0
impl crate::Resettable for MEMCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
