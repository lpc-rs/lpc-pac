#[doc = "Register `SCR` reader"]
pub struct R(crate::R<SCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SCR_SPEC>> for R {
    fn from(reader: crate::R<SCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MaximumPortSize` reader - Maximum ETM port size bits \\[2:0\\]. These bits are used in conjunction with bit \\[9\\]. The value of these bits is b001."]
pub struct MAXIMUMPORTSIZE_R(crate::FieldReader<u8, u8>);
impl MAXIMUMPORTSIZE_R {
    pub(crate) fn new(bits: u8) -> Self {
        MAXIMUMPORTSIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAXIMUMPORTSIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFOFULLsupported` reader - FIFOFULL supported. The value of this bit is 1, indicating that FIFOFULL is supported. This bit is used in conjunction with bit \\[23\\]
of the ETMCCR."]
pub struct FIFOFULLSUPPORTED_R(crate::FieldReader<bool, bool>);
impl FIFOFULLSUPPORTED_R {
    pub(crate) fn new(bits: bool) -> Self {
        FIFOFULLSUPPORTED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFOFULLSUPPORTED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MaximumPortSize3` reader - Maximum ETM port size bit \\[3\\]. This bit is used in conjunction with bits \\[2:0\\]. Its value is 0. This has no effect on the TPIU trace port."]
pub struct MAXIMUMPORTSIZE3_R(crate::FieldReader<bool, bool>);
impl MAXIMUMPORTSIZE3_R {
    pub(crate) fn new(bits: bool) -> Self {
        MAXIMUMPORTSIZE3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAXIMUMPORTSIZE3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PortSizeSupported` reader - Port size supported. This bit reads as 1 if the currently selected port size is supported. This has no effect on the TPIU trace port."]
pub struct PORTSIZESUPPORTED_R(crate::FieldReader<bool, bool>);
impl PORTSIZESUPPORTED_R {
    pub(crate) fn new(bits: bool) -> Self {
        PORTSIZESUPPORTED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PORTSIZESUPPORTED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PortModeSupported` reader - Port mode supported. This bit reads as 1 if the currently selected port mode is supported. This has no effect on the TPIU trace port."]
pub struct PORTMODESUPPORTED_R(crate::FieldReader<bool, bool>);
impl PORTMODESUPPORTED_R {
    pub(crate) fn new(bits: bool) -> Self {
        PORTMODESUPPORTED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PORTMODESUPPORTED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `N` reader - These bits give the number of supported processors minus 1. The value of these bits is b000, indicating that there is only one processor connected."]
pub struct N_R(crate::FieldReader<u8, u8>);
impl N_R {
    pub(crate) fn new(bits: u8) -> Self {
        N_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for N_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NoFetchComparisons` reader - No Fetch comparisons. The value of this bit is 1, indicating that fetch comparisons are not implemented."]
pub struct NOFETCHCOMPARISONS_R(crate::FieldReader<bool, bool>);
impl NOFETCHCOMPARISONS_R {
    pub(crate) fn new(bits: bool) -> Self {
        NOFETCHCOMPARISONS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NOFETCHCOMPARISONS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:2 - Maximum ETM port size bits \\[2:0\\]. These bits are used in conjunction with bit \\[9\\]. The value of these bits is b001."]
    #[inline(always)]
    pub fn maximum_port_size(&self) -> MAXIMUMPORTSIZE_R {
        MAXIMUMPORTSIZE_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 8 - FIFOFULL supported. The value of this bit is 1, indicating that FIFOFULL is supported. This bit is used in conjunction with bit \\[23\\]
of the ETMCCR."]
    #[inline(always)]
    pub fn fifofullsupported(&self) -> FIFOFULLSUPPORTED_R {
        FIFOFULLSUPPORTED_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Maximum ETM port size bit \\[3\\]. This bit is used in conjunction with bits \\[2:0\\]. Its value is 0. This has no effect on the TPIU trace port."]
    #[inline(always)]
    pub fn maximum_port_size3(&self) -> MAXIMUMPORTSIZE3_R {
        MAXIMUMPORTSIZE3_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Port size supported. This bit reads as 1 if the currently selected port size is supported. This has no effect on the TPIU trace port."]
    #[inline(always)]
    pub fn port_size_supported(&self) -> PORTSIZESUPPORTED_R {
        PORTSIZESUPPORTED_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Port mode supported. This bit reads as 1 if the currently selected port mode is supported. This has no effect on the TPIU trace port."]
    #[inline(always)]
    pub fn port_mode_supported(&self) -> PORTMODESUPPORTED_R {
        PORTMODESUPPORTED_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:14 - These bits give the number of supported processors minus 1. The value of these bits is b000, indicating that there is only one processor connected."]
    #[inline(always)]
    pub fn n(&self) -> N_R {
        N_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bit 17 - No Fetch comparisons. The value of this bit is 1, indicating that fetch comparisons are not implemented."]
    #[inline(always)]
    pub fn no_fetch_comparisons(&self) -> NOFETCHCOMPARISONS_R {
        NOFETCHCOMPARISONS_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
#[doc = "System Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scr](index.html) module"]
pub struct SCR_SPEC;
impl crate::RegisterSpec for SCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scr::R](R) reader structure"]
impl crate::Readable for SCR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SCR to value 0x0002_0d09"]
impl crate::Resettable for SCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0002_0d09
    }
}
