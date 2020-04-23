#[doc = "Reader of register LSR"]
pub type R = crate::R<u32, super::LSR>;
#[doc = "Reader of field `IMP`"]
pub type IMP_R = crate::R<bool, bool>;
#[doc = "Lock Status. This bit is HIGH when the device is locked, and LOW when unlocked.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STATUS_A {
    #[doc = "0: Access permitted."]
    STATUS_0 = 0,
    #[doc = "1: Write access to the component is blocked. All writes to control registers are ignored. Reads are permitted."]
    STATUS_1 = 1,
}
impl From<STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `STATUS`"]
pub type STATUS_R = crate::R<bool, STATUS_A>;
impl STATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STATUS_A {
        match self.bits {
            false => STATUS_A::STATUS_0,
            true => STATUS_A::STATUS_1,
        }
    }
    #[doc = "Checks if the value of the field is `STATUS_0`"]
    #[inline(always)]
    pub fn is_status_0(&self) -> bool {
        *self == STATUS_A::STATUS_0
    }
    #[doc = "Checks if the value of the field is `STATUS_1`"]
    #[inline(always)]
    pub fn is_status_1(&self) -> bool {
        *self == STATUS_A::STATUS_1
    }
}
#[doc = "Reader of field `s8BIT`"]
pub type S8BIT_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Lock mechanism is implemented. This bit always reads 1."]
    #[inline(always)]
    pub fn imp(&self) -> IMP_R {
        IMP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Lock Status. This bit is HIGH when the device is locked, and LOW when unlocked."]
    #[inline(always)]
    pub fn status(&self) -> STATUS_R {
        STATUS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Access Lock Register size. This bit reads 0 to indicate a 32-bit register is present."]
    #[inline(always)]
    pub fn s8bit(&self) -> S8BIT_R {
        S8BIT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
