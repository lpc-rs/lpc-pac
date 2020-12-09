#[doc = "Reader of register CIDR1"]
pub type R = crate::R<u32, super::CIDR1>;
#[doc = "Reader of field `Preamble`"]
pub type PREAMBLE_R = crate::R<u8, u8>;
#[doc = "Component class\n\nValue on reset: 9"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum COMPONENTCLASS_A {
    #[doc = "1: ROM table."]
    COMPONENTCLASS_1 = 1,
    #[doc = "9: CoreSight component."]
    COMPONENTCLASS_9 = 9,
    #[doc = "15: PrimeCell of system component with no standardized register layout, for backward compatibility."]
    COMPONENTCLASS_15 = 15,
}
impl From<COMPONENTCLASS_A> for u8 {
    #[inline(always)]
    fn from(variant: COMPONENTCLASS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ComponentClass`"]
pub type COMPONENTCLASS_R = crate::R<u8, COMPONENTCLASS_A>;
impl COMPONENTCLASS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, COMPONENTCLASS_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(COMPONENTCLASS_A::COMPONENTCLASS_1),
            9 => Val(COMPONENTCLASS_A::COMPONENTCLASS_9),
            15 => Val(COMPONENTCLASS_A::COMPONENTCLASS_15),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `COMPONENTCLASS_1`"]
    #[inline(always)]
    pub fn is_component_class_1(&self) -> bool {
        *self == COMPONENTCLASS_A::COMPONENTCLASS_1
    }
    #[doc = "Checks if the value of the field is `COMPONENTCLASS_9`"]
    #[inline(always)]
    pub fn is_component_class_9(&self) -> bool {
        *self == COMPONENTCLASS_A::COMPONENTCLASS_9
    }
    #[doc = "Checks if the value of the field is `COMPONENTCLASS_15`"]
    #[inline(always)]
    pub fn is_component_class_15(&self) -> bool {
        *self == COMPONENTCLASS_A::COMPONENTCLASS_15
    }
}
impl R {
    #[doc = "Bits 0:3 - Preamble"]
    #[inline(always)]
    pub fn preamble(&self) -> PREAMBLE_R {
        PREAMBLE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Component class"]
    #[inline(always)]
    pub fn component_class(&self) -> COMPONENTCLASS_R {
        COMPONENTCLASS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
