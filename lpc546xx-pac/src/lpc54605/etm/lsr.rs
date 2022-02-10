///Register `LSR` reader
pub struct R(crate::R<LSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LSR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `IMP` reader - Lock mechanism is implemented. This bit always reads 1.
pub struct IMP_R(crate::FieldReader<bool, bool>);
impl IMP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Lock Status. This bit is HIGH when the device is locked, and LOW when unlocked.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STATUS_A {
    ///0: Access permitted.
    STATUS_0 = 0,
    ///1: Write access to the component is blocked. All writes to control registers are ignored. Reads are permitted.
    STATUS_1 = 1,
}
impl From<STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: STATUS_A) -> Self {
        variant as u8 != 0
    }
}
///Field `STATUS` reader - Lock Status. This bit is HIGH when the device is locked, and LOW when unlocked.
pub struct STATUS_R(crate::FieldReader<bool, STATUS_A>);
impl STATUS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STATUS_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> STATUS_A {
        match self.bits {
            false => STATUS_A::STATUS_0,
            true => STATUS_A::STATUS_1,
        }
    }
    ///Checks if the value of the field is `STATUS_0`
    #[inline(always)]
    pub fn is_status_0(&self) -> bool {
        **self == STATUS_A::STATUS_0
    }
    ///Checks if the value of the field is `STATUS_1`
    #[inline(always)]
    pub fn is_status_1(&self) -> bool {
        **self == STATUS_A::STATUS_1
    }
}
impl core::ops::Deref for STATUS_R {
    type Target = crate::FieldReader<bool, STATUS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `s8BIT` reader - Access Lock Register size. This bit reads 0 to indicate a 32-bit register is present.
pub struct S8BIT_R(crate::FieldReader<bool, bool>);
impl S8BIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        S8BIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S8BIT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    ///Bit 0 - Lock mechanism is implemented. This bit always reads 1.
    #[inline(always)]
    pub fn imp(&self) -> IMP_R {
        IMP_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - Lock Status. This bit is HIGH when the device is locked, and LOW when unlocked.
    #[inline(always)]
    pub fn status(&self) -> STATUS_R {
        STATUS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - Access Lock Register size. This bit reads 0 to indicate a 32-bit register is present.
    #[inline(always)]
    pub fn s8bit(&self) -> S8BIT_R {
        S8BIT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
///Lock Status Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [lsr](index.html) module
pub struct LSR_SPEC;
impl crate::RegisterSpec for LSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [lsr::R](R) reader structure
impl crate::Readable for LSR_SPEC {
    type Reader = R;
}
///`reset()` method sets LSR to value 0x01
impl crate::Resettable for LSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
