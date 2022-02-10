///Register `MAC_TXQ_PRIO_MAP` reader
pub struct R(crate::R<MAC_TXQ_PRIO_MAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_TXQ_PRIO_MAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_TXQ_PRIO_MAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_TXQ_PRIO_MAP_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MAC_TXQ_PRIO_MAP` writer
pub struct W(crate::W<MAC_TXQ_PRIO_MAP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_TXQ_PRIO_MAP_SPEC>;
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
impl From<crate::W<MAC_TXQ_PRIO_MAP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC_TXQ_PRIO_MAP_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PSTQ0` reader - Priorities Selected in Transmit Queue 0 This field holds the priorities assigned to Tx Queue 0 by the software.
pub struct PSTQ0_R(crate::FieldReader<u8, u8>);
impl PSTQ0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PSTQ0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSTQ0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PSTQ0` writer - Priorities Selected in Transmit Queue 0 This field holds the priorities assigned to Tx Queue 0 by the software.
pub struct PSTQ0_W<'a> {
    w: &'a mut W,
}
impl<'a> PSTQ0_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
///Field `PSTQ1` reader - Priorities Selected in Transmit Queue 1 This bit is similar to the PSTQ0 bit.
pub struct PSTQ1_R(crate::FieldReader<u8, u8>);
impl PSTQ1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PSTQ1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSTQ1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PSTQ1` writer - Priorities Selected in Transmit Queue 1 This bit is similar to the PSTQ0 bit.
pub struct PSTQ1_W<'a> {
    w: &'a mut W,
}
impl<'a> PSTQ1_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
impl R {
    ///Bits 0:7 - Priorities Selected in Transmit Queue 0 This field holds the priorities assigned to Tx Queue 0 by the software.
    #[inline(always)]
    pub fn pstq0(&self) -> PSTQ0_R {
        PSTQ0_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Priorities Selected in Transmit Queue 1 This bit is similar to the PSTQ0 bit.
    #[inline(always)]
    pub fn pstq1(&self) -> PSTQ1_R {
        PSTQ1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - Priorities Selected in Transmit Queue 0 This field holds the priorities assigned to Tx Queue 0 by the software.
    #[inline(always)]
    pub fn pstq0(&mut self) -> PSTQ0_W {
        PSTQ0_W { w: self }
    }
    ///Bits 8:15 - Priorities Selected in Transmit Queue 1 This bit is similar to the PSTQ0 bit.
    #[inline(always)]
    pub fn pstq1(&mut self) -> PSTQ1_W {
        PSTQ1_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///no description available
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mac_txq_prio_map](index.html) module
pub struct MAC_TXQ_PRIO_MAP_SPEC;
impl crate::RegisterSpec for MAC_TXQ_PRIO_MAP_SPEC {
    type Ux = u32;
}
///`read()` method returns [mac_txq_prio_map::R](R) reader structure
impl crate::Readable for MAC_TXQ_PRIO_MAP_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [mac_txq_prio_map::W](W) writer structure
impl crate::Writable for MAC_TXQ_PRIO_MAP_SPEC {
    type Writer = W;
}
///`reset()` method sets MAC_TXQ_PRIO_MAP to value 0
impl crate::Resettable for MAC_TXQ_PRIO_MAP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
