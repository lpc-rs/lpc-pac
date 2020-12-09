#[doc = "Reader of register DEVTYPE"]
pub type R = crate::R<u32, super::DEVTYPE>;
#[doc = "Major Type and Class\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MAJORTYPE_A {
    #[doc = "3: Trace source"]
    MAJORTYPE_3 = 3,
}
impl From<MAJORTYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: MAJORTYPE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MajorType`"]
pub type MAJORTYPE_R = crate::R<u8, MAJORTYPE_A>;
impl MAJORTYPE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MAJORTYPE_A> {
        use crate::Variant::*;
        match self.bits {
            3 => Val(MAJORTYPE_A::MAJORTYPE_3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MAJORTYPE_3`"]
    #[inline(always)]
    pub fn is_major_type_3(&self) -> bool {
        *self == MAJORTYPE_A::MAJORTYPE_3
    }
}
#[doc = "Sub Type\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SUBTYPE_A {
    #[doc = "1: Processor trace"]
    SUBTYPE_1 = 1,
}
impl From<SUBTYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: SUBTYPE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SubType`"]
pub type SUBTYPE_R = crate::R<u8, SUBTYPE_A>;
impl SUBTYPE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SUBTYPE_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(SUBTYPE_A::SUBTYPE_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SUBTYPE_1`"]
    #[inline(always)]
    pub fn is_sub_type_1(&self) -> bool {
        *self == SUBTYPE_A::SUBTYPE_1
    }
}
impl R {
    #[doc = "Bits 0:3 - Major Type and Class"]
    #[inline(always)]
    pub fn major_type(&self) -> MAJORTYPE_R {
        MAJORTYPE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Sub Type"]
    #[inline(always)]
    pub fn sub_type(&self) -> SUBTYPE_R {
        SUBTYPE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
