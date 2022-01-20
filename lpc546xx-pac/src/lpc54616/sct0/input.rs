#[doc = "Register `INPUT` reader"]
pub struct R(crate::R<INPUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INPUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INPUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INPUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `AIN0` reader - Input 0 state. Input 0 state on the last SCT clock edge."]
pub struct AIN0_R(crate::FieldReader<bool, bool>);
impl AIN0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AIN0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AIN0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AIN1` reader - Input 1 state. Input 1 state on the last SCT clock edge."]
pub struct AIN1_R(crate::FieldReader<bool, bool>);
impl AIN1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AIN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AIN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AIN2` reader - Input 2 state. Input 2 state on the last SCT clock edge."]
pub struct AIN2_R(crate::FieldReader<bool, bool>);
impl AIN2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AIN2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AIN2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AIN3` reader - Input 3 state. Input 3 state on the last SCT clock edge."]
pub struct AIN3_R(crate::FieldReader<bool, bool>);
impl AIN3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AIN3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AIN3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AIN4` reader - Input 4 state. Input 4 state on the last SCT clock edge."]
pub struct AIN4_R(crate::FieldReader<bool, bool>);
impl AIN4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AIN4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AIN4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AIN5` reader - Input 5 state. Input 5 state on the last SCT clock edge."]
pub struct AIN5_R(crate::FieldReader<bool, bool>);
impl AIN5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AIN5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AIN5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AIN6` reader - Input 6 state. Input 6 state on the last SCT clock edge."]
pub struct AIN6_R(crate::FieldReader<bool, bool>);
impl AIN6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AIN6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AIN6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AIN7` reader - Input 7 state. Input 7 state on the last SCT clock edge."]
pub struct AIN7_R(crate::FieldReader<bool, bool>);
impl AIN7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AIN7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AIN7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AIN8` reader - Input 8 state. Input 8 state on the last SCT clock edge."]
pub struct AIN8_R(crate::FieldReader<bool, bool>);
impl AIN8_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AIN8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AIN8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AIN9` reader - Input 9 state. Input 9 state on the last SCT clock edge."]
pub struct AIN9_R(crate::FieldReader<bool, bool>);
impl AIN9_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AIN9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AIN9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AIN10` reader - Input 10 state. Input 10 state on the last SCT clock edge."]
pub struct AIN10_R(crate::FieldReader<bool, bool>);
impl AIN10_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AIN10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AIN10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AIN11` reader - Input 11 state. Input 11 state on the last SCT clock edge."]
pub struct AIN11_R(crate::FieldReader<bool, bool>);
impl AIN11_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AIN11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AIN11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AIN12` reader - Input 12 state. Input 12 state on the last SCT clock edge."]
pub struct AIN12_R(crate::FieldReader<bool, bool>);
impl AIN12_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AIN12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AIN12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AIN13` reader - Input 13 state. Input 13 state on the last SCT clock edge."]
pub struct AIN13_R(crate::FieldReader<bool, bool>);
impl AIN13_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AIN13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AIN13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AIN14` reader - Input 14 state. Input 14 state on the last SCT clock edge."]
pub struct AIN14_R(crate::FieldReader<bool, bool>);
impl AIN14_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AIN14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AIN14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AIN15` reader - Input 15 state. Input 15 state on the last SCT clock edge."]
pub struct AIN15_R(crate::FieldReader<bool, bool>);
impl AIN15_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AIN15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AIN15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIN0` reader - Input 0 state. Input 0 state following the synchronization specified by INSYNC."]
pub struct SIN0_R(crate::FieldReader<bool, bool>);
impl SIN0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SIN0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SIN0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIN1` reader - Input 1 state. Input 1 state following the synchronization specified by INSYNC."]
pub struct SIN1_R(crate::FieldReader<bool, bool>);
impl SIN1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SIN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SIN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIN2` reader - Input 2 state. Input 2 state following the synchronization specified by INSYNC."]
pub struct SIN2_R(crate::FieldReader<bool, bool>);
impl SIN2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SIN2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SIN2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIN3` reader - Input 3 state. Input 3 state following the synchronization specified by INSYNC."]
pub struct SIN3_R(crate::FieldReader<bool, bool>);
impl SIN3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SIN3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SIN3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIN4` reader - Input 4 state. Input 4 state following the synchronization specified by INSYNC."]
pub struct SIN4_R(crate::FieldReader<bool, bool>);
impl SIN4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SIN4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SIN4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIN5` reader - Input 5 state. Input 5 state following the synchronization specified by INSYNC."]
pub struct SIN5_R(crate::FieldReader<bool, bool>);
impl SIN5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SIN5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SIN5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIN6` reader - Input 6 state. Input 6 state following the synchronization specified by INSYNC."]
pub struct SIN6_R(crate::FieldReader<bool, bool>);
impl SIN6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SIN6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SIN6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIN7` reader - Input 7 state. Input 7 state following the synchronization specified by INSYNC."]
pub struct SIN7_R(crate::FieldReader<bool, bool>);
impl SIN7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SIN7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SIN7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIN8` reader - Input 8 state. Input 8 state following the synchronization specified by INSYNC."]
pub struct SIN8_R(crate::FieldReader<bool, bool>);
impl SIN8_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SIN8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SIN8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIN9` reader - Input 9 state. Input 9 state following the synchronization specified by INSYNC."]
pub struct SIN9_R(crate::FieldReader<bool, bool>);
impl SIN9_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SIN9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SIN9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIN10` reader - Input 10 state. Input 10 state following the synchronization specified by INSYNC."]
pub struct SIN10_R(crate::FieldReader<bool, bool>);
impl SIN10_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SIN10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SIN10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIN11` reader - Input 11 state. Input 11 state following the synchronization specified by INSYNC."]
pub struct SIN11_R(crate::FieldReader<bool, bool>);
impl SIN11_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SIN11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SIN11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIN12` reader - Input 12 state. Input 12 state following the synchronization specified by INSYNC."]
pub struct SIN12_R(crate::FieldReader<bool, bool>);
impl SIN12_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SIN12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SIN12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIN13` reader - Input 13 state. Input 13 state following the synchronization specified by INSYNC."]
pub struct SIN13_R(crate::FieldReader<bool, bool>);
impl SIN13_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SIN13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SIN13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIN14` reader - Input 14 state. Input 14 state following the synchronization specified by INSYNC."]
pub struct SIN14_R(crate::FieldReader<bool, bool>);
impl SIN14_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SIN14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SIN14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIN15` reader - Input 15 state. Input 15 state following the synchronization specified by INSYNC."]
pub struct SIN15_R(crate::FieldReader<bool, bool>);
impl SIN15_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SIN15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SIN15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
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
    #[doc = "Bit 5 - Input 5 state. Input 5 state on the last SCT clock edge."]
    #[inline(always)]
    pub fn ain5(&self) -> AIN5_R {
        AIN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Input 6 state. Input 6 state on the last SCT clock edge."]
    #[inline(always)]
    pub fn ain6(&self) -> AIN6_R {
        AIN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Input 7 state. Input 7 state on the last SCT clock edge."]
    #[inline(always)]
    pub fn ain7(&self) -> AIN7_R {
        AIN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Input 8 state. Input 8 state on the last SCT clock edge."]
    #[inline(always)]
    pub fn ain8(&self) -> AIN8_R {
        AIN8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Input 9 state. Input 9 state on the last SCT clock edge."]
    #[inline(always)]
    pub fn ain9(&self) -> AIN9_R {
        AIN9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Input 10 state. Input 10 state on the last SCT clock edge."]
    #[inline(always)]
    pub fn ain10(&self) -> AIN10_R {
        AIN10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Input 11 state. Input 11 state on the last SCT clock edge."]
    #[inline(always)]
    pub fn ain11(&self) -> AIN11_R {
        AIN11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Input 12 state. Input 12 state on the last SCT clock edge."]
    #[inline(always)]
    pub fn ain12(&self) -> AIN12_R {
        AIN12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Input 13 state. Input 13 state on the last SCT clock edge."]
    #[inline(always)]
    pub fn ain13(&self) -> AIN13_R {
        AIN13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Input 14 state. Input 14 state on the last SCT clock edge."]
    #[inline(always)]
    pub fn ain14(&self) -> AIN14_R {
        AIN14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Input 15 state. Input 15 state on the last SCT clock edge."]
    #[inline(always)]
    pub fn ain15(&self) -> AIN15_R {
        AIN15_R::new(((self.bits >> 15) & 0x01) != 0)
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
    #[doc = "Bit 21 - Input 5 state. Input 5 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub fn sin5(&self) -> SIN5_R {
        SIN5_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Input 6 state. Input 6 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub fn sin6(&self) -> SIN6_R {
        SIN6_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Input 7 state. Input 7 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub fn sin7(&self) -> SIN7_R {
        SIN7_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Input 8 state. Input 8 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub fn sin8(&self) -> SIN8_R {
        SIN8_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Input 9 state. Input 9 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub fn sin9(&self) -> SIN9_R {
        SIN9_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Input 10 state. Input 10 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub fn sin10(&self) -> SIN10_R {
        SIN10_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Input 11 state. Input 11 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub fn sin11(&self) -> SIN11_R {
        SIN11_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Input 12 state. Input 12 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub fn sin12(&self) -> SIN12_R {
        SIN12_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Input 13 state. Input 13 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub fn sin13(&self) -> SIN13_R {
        SIN13_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Input 14 state. Input 14 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub fn sin14(&self) -> SIN14_R {
        SIN14_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Input 15 state. Input 15 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub fn sin15(&self) -> SIN15_R {
        SIN15_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
#[doc = "SCT input register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [input](index.html) module"]
pub struct INPUT_SPEC;
impl crate::RegisterSpec for INPUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [input::R](R) reader structure"]
impl crate::Readable for INPUT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INPUT to value 0"]
impl crate::Resettable for INPUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
