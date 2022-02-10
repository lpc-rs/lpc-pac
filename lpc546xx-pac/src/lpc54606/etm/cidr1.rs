///Register `CIDR1` reader
pub struct R(crate::R<CIDR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CIDR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CIDR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CIDR1_SPEC>) -> Self {
        R(reader)
    }
}
///Field `Preamble` reader - Preamble
pub struct PREAMBLE_R(crate::FieldReader<u8, u8>);
impl PREAMBLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PREAMBLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PREAMBLE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Component class
///
///Value on reset: 9
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum COMPONENTCLASS_A {
    ///1: ROM table.
    COMPONENTCLASS_1 = 1,
    ///9: CoreSight component.
    COMPONENTCLASS_9 = 9,
    ///15: PrimeCell of system component with no standardized register layout, for backward compatibility.
    COMPONENTCLASS_15 = 15,
}
impl From<COMPONENTCLASS_A> for u8 {
    #[inline(always)]
    fn from(variant: COMPONENTCLASS_A) -> Self {
        variant as _
    }
}
///Field `ComponentClass` reader - Component class
pub struct COMPONENTCLASS_R(crate::FieldReader<u8, COMPONENTCLASS_A>);
impl COMPONENTCLASS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        COMPONENTCLASS_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<COMPONENTCLASS_A> {
        match self.bits {
            1 => Some(COMPONENTCLASS_A::COMPONENTCLASS_1),
            9 => Some(COMPONENTCLASS_A::COMPONENTCLASS_9),
            15 => Some(COMPONENTCLASS_A::COMPONENTCLASS_15),
            _ => None,
        }
    }
    ///Checks if the value of the field is `COMPONENTCLASS_1`
    #[inline(always)]
    pub fn is_component_class_1(&self) -> bool {
        **self == COMPONENTCLASS_A::COMPONENTCLASS_1
    }
    ///Checks if the value of the field is `COMPONENTCLASS_9`
    #[inline(always)]
    pub fn is_component_class_9(&self) -> bool {
        **self == COMPONENTCLASS_A::COMPONENTCLASS_9
    }
    ///Checks if the value of the field is `COMPONENTCLASS_15`
    #[inline(always)]
    pub fn is_component_class_15(&self) -> bool {
        **self == COMPONENTCLASS_A::COMPONENTCLASS_15
    }
}
impl core::ops::Deref for COMPONENTCLASS_R {
    type Target = crate::FieldReader<u8, COMPONENTCLASS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    ///Bits 0:3 - Preamble
    #[inline(always)]
    pub fn preamble(&self) -> PREAMBLE_R {
        PREAMBLE_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Component class
    #[inline(always)]
    pub fn component_class(&self) -> COMPONENTCLASS_R {
        COMPONENTCLASS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
///Component Identification Register 1
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cidr1](index.html) module
pub struct CIDR1_SPEC;
impl crate::RegisterSpec for CIDR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [cidr1::R](R) reader structure
impl crate::Readable for CIDR1_SPEC {
    type Reader = R;
}
///`reset()` method sets CIDR1 to value 0x90
impl crate::Resettable for CIDR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x90
    }
}
