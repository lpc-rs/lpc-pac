#[doc = "Reader of register INPUT"]
pub type R = crate::R<u32, super::INPUT>;
#[doc = "Writer for register INPUT"]
pub type W = crate::W<u32, super::INPUT>;
#[doc = "Register INPUT `reset()`'s with value 0"]
impl crate::ResetValue for super::INPUT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AIN0`"]
pub type AIN0_R = crate::R<bool, bool>;
#[doc = "Reader of field `AIN1`"]
pub type AIN1_R = crate::R<bool, bool>;
#[doc = "Reader of field `AIN2`"]
pub type AIN2_R = crate::R<bool, bool>;
#[doc = "Reader of field `AIN3`"]
pub type AIN3_R = crate::R<bool, bool>;
#[doc = "Reader of field `AIN4`"]
pub type AIN4_R = crate::R<bool, bool>;
#[doc = "Reader of field `SIN0`"]
pub type SIN0_R = crate::R<bool, bool>;
#[doc = "Reader of field `SIN1`"]
pub type SIN1_R = crate::R<bool, bool>;
#[doc = "Reader of field `SIN2`"]
pub type SIN2_R = crate::R<bool, bool>;
#[doc = "Reader of field `SIN3`"]
pub type SIN3_R = crate::R<bool, bool>;
#[doc = "Reader of field `SIN4`"]
pub type SIN4_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Input 0 state. Input 0 state on the last SCT clock edge."]
    #[inline(always)]
    pub fn ain0(&self) -> AIN0_R {
        AIN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Input 1 state. Input 1 state on the last SCT clock edge."]
    #[inline(always)]
    pub fn ain1(&self) -> AIN1_R {
        AIN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Input 2 state. Input 2 state on the last SCT clock edge."]
    #[inline(always)]
    pub fn ain2(&self) -> AIN2_R {
        AIN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Input 3 state. Input 3 state on the last SCT clock edge."]
    #[inline(always)]
    pub fn ain3(&self) -> AIN3_R {
        AIN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Input 4 state. Input 4 state on the last SCT clock edge."]
    #[inline(always)]
    pub fn ain4(&self) -> AIN4_R {
        AIN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Input 0 state. Input 0 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub fn sin0(&self) -> SIN0_R {
        SIN0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Input 1 state. Input 1 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub fn sin1(&self) -> SIN1_R {
        SIN1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Input 2 state. Input 2 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub fn sin2(&self) -> SIN2_R {
        SIN2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Input 3 state. Input 3 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub fn sin3(&self) -> SIN3_R {
        SIN3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Input 4 state. Input 4 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub fn sin4(&self) -> SIN4_R {
        SIN4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {}
