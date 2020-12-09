#[doc = "Reader of register AUTHSTATUS"]
pub type R = crate::R<u32, super::AUTHSTATUS>;
#[doc = "Reader of field `NSID`"]
pub type NSID_R = crate::R<u8, u8>;
#[doc = "Permission for Non-secure non-invasive debug.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum NSNID_A {
    #[doc = "2: Non-secure non-invasive debug disabled"]
    NSNID_2 = 2,
    #[doc = "3: Non-secure non-invasive debug enabled"]
    NSNID_3 = 3,
}
impl From<NSNID_A> for u8 {
    #[inline(always)]
    fn from(variant: NSNID_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `NSNID`"]
pub type NSNID_R = crate::R<u8, NSNID_A>;
impl NSNID_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, NSNID_A> {
        use crate::Variant::*;
        match self.bits {
            2 => Val(NSNID_A::NSNID_2),
            3 => Val(NSNID_A::NSNID_3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NSNID_2`"]
    #[inline(always)]
    pub fn is_nsnid_2(&self) -> bool {
        *self == NSNID_A::NSNID_2
    }
    #[doc = "Checks if the value of the field is `NSNID_3`"]
    #[inline(always)]
    pub fn is_nsnid_3(&self) -> bool {
        *self == NSNID_A::NSNID_3
    }
}
#[doc = "Reader of field `SID`"]
pub type SID_R = crate::R<u8, u8>;
#[doc = "Reader of field `SNID`"]
pub type SNID_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:1 - Reads as b00, Non-secure invasive debug not supported by the ETM."]
    #[inline(always)]
    pub fn nsid(&self) -> NSID_R {
        NSID_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Permission for Non-secure non-invasive debug."]
    #[inline(always)]
    pub fn nsnid(&self) -> NSNID_R {
        NSNID_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Reads as b00, Secure invasive debug not supported by the ETM."]
    #[inline(always)]
    pub fn sid(&self) -> SID_R {
        SID_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Permission for Secure non-invasive debug."]
    #[inline(always)]
    pub fn snid(&self) -> SNID_R {
        SNID_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
