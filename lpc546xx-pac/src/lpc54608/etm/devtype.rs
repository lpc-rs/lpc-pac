///Register `DEVTYPE` reader
pub struct R(crate::R<DEVTYPE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEVTYPE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEVTYPE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEVTYPE_SPEC>) -> Self {
        R(reader)
    }
}
///Major Type and Class
///
///Value on reset: 3
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MAJORTYPE_A {
    ///3: Trace source
    MAJORTYPE_3 = 3,
}
impl From<MAJORTYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: MAJORTYPE_A) -> Self {
        variant as _
    }
}
///Field `MajorType` reader - Major Type and Class
pub struct MAJORTYPE_R(crate::FieldReader<u8, MAJORTYPE_A>);
impl MAJORTYPE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MAJORTYPE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<MAJORTYPE_A> {
        match self.bits {
            3 => Some(MAJORTYPE_A::MAJORTYPE_3),
            _ => None,
        }
    }
    ///Checks if the value of the field is `MAJORTYPE_3`
    #[inline(always)]
    pub fn is_major_type_3(&self) -> bool {
        **self == MAJORTYPE_A::MAJORTYPE_3
    }
}
impl core::ops::Deref for MAJORTYPE_R {
    type Target = crate::FieldReader<u8, MAJORTYPE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Sub Type
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SUBTYPE_A {
    ///1: Processor trace
    SUBTYPE_1 = 1,
}
impl From<SUBTYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: SUBTYPE_A) -> Self {
        variant as _
    }
}
///Field `SubType` reader - Sub Type
pub struct SUBTYPE_R(crate::FieldReader<u8, SUBTYPE_A>);
impl SUBTYPE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SUBTYPE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<SUBTYPE_A> {
        match self.bits {
            1 => Some(SUBTYPE_A::SUBTYPE_1),
            _ => None,
        }
    }
    ///Checks if the value of the field is `SUBTYPE_1`
    #[inline(always)]
    pub fn is_sub_type_1(&self) -> bool {
        **self == SUBTYPE_A::SUBTYPE_1
    }
}
impl core::ops::Deref for SUBTYPE_R {
    type Target = crate::FieldReader<u8, SUBTYPE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    ///Bits 0:3 - Major Type and Class
    #[inline(always)]
    pub fn major_type(&self) -> MAJORTYPE_R {
        MAJORTYPE_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Sub Type
    #[inline(always)]
    pub fn sub_type(&self) -> SUBTYPE_R {
        SUBTYPE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
///CoreSight Device Type Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [devtype](index.html) module
pub struct DEVTYPE_SPEC;
impl crate::RegisterSpec for DEVTYPE_SPEC {
    type Ux = u32;
}
///`read()` method returns [devtype::R](R) reader structure
impl crate::Readable for DEVTYPE_SPEC {
    type Reader = R;
}
///`reset()` method sets DEVTYPE to value 0x13
impl crate::Resettable for DEVTYPE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x13
    }
}
