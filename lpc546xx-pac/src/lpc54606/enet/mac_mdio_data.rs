///Register `MAC_MDIO_DATA` reader
pub struct R(crate::R<MAC_MDIO_DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_MDIO_DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_MDIO_DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_MDIO_DATA_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MAC_MDIO_DATA` writer
pub struct W(crate::W<MAC_MDIO_DATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_MDIO_DATA_SPEC>;
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
impl From<crate::W<MAC_MDIO_DATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC_MDIO_DATA_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MD` reader - MII Data This field contains the 16-bit data value read from the PHY after a Management Read operation or the 16-bit data value to be written to the PHY before a Management Write operation.
pub struct MD_R(crate::FieldReader<u16, u16>);
impl MD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        MD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MD_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `MD` writer - MII Data This field contains the 16-bit data value read from the PHY after a Management Read operation or the 16-bit data value to be written to the PHY before a Management Write operation.
pub struct MD_W<'a> {
    w: &'a mut W,
}
impl<'a> MD_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    ///Bits 0:15 - MII Data This field contains the 16-bit data value read from the PHY after a Management Read operation or the 16-bit data value to be written to the PHY before a Management Write operation.
    #[inline(always)]
    pub fn md(&self) -> MD_R {
        MD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - MII Data This field contains the 16-bit data value read from the PHY after a Management Read operation or the 16-bit data value to be written to the PHY before a Management Write operation.
    #[inline(always)]
    pub fn md(&mut self) -> MD_W {
        MD_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///MDIO Data register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mac_mdio_data](index.html) module
pub struct MAC_MDIO_DATA_SPEC;
impl crate::RegisterSpec for MAC_MDIO_DATA_SPEC {
    type Ux = u32;
}
///`read()` method returns [mac_mdio_data::R](R) reader structure
impl crate::Readable for MAC_MDIO_DATA_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [mac_mdio_data::W](W) writer structure
impl crate::Writable for MAC_MDIO_DATA_SPEC {
    type Writer = W;
}
///`reset()` method sets MAC_MDIO_DATA to value 0
impl crate::Resettable for MAC_MDIO_DATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
