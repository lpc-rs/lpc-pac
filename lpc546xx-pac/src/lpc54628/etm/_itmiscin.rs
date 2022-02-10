///Register `_ITMISCIN` reader
pub struct R(crate::R<_ITMISCIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<_ITMISCIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<_ITMISCIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<_ITMISCIN_SPEC>) -> Self {
        R(reader)
    }
}
///Field `EXTIN` reader - A read of these bits returns the value of the EXTIN\[1:0\]
///input pins.
pub struct EXTIN_R(crate::FieldReader<u8, u8>);
impl EXTIN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EXTIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTIN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `COREHALT` reader - A read of this bit returns the value of the COREHALT input pin.
pub struct COREHALT_R(crate::FieldReader<bool, bool>);
impl COREHALT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COREHALT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COREHALT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    ///Bits 0:1 - A read of these bits returns the value of the EXTIN\[1:0\]
    ///input pins.
    #[inline(always)]
    pub fn extin(&self) -> EXTIN_R {
        EXTIN_R::new((self.bits & 0x03) as u8)
    }
    ///Bit 4 - A read of this bit returns the value of the COREHALT input pin.
    #[inline(always)]
    pub fn corehalt(&self) -> COREHALT_R {
        COREHALT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
///Integration Test Miscelaneous Inputs Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [_itmiscin](index.html) module
pub struct _ITMISCIN_SPEC;
impl crate::RegisterSpec for _ITMISCIN_SPEC {
    type Ux = u32;
}
///`read()` method returns [_itmiscin::R](R) reader structure
impl crate::Readable for _ITMISCIN_SPEC {
    type Reader = R;
}
///`reset()` method sets _ITMISCIN to value 0
impl crate::Resettable for _ITMISCIN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
