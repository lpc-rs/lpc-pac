///Register `USE2FS` reader
pub struct R(crate::R<USE2FS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USE2FS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USE2FS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USE2FS_SPEC>) -> Self {
        R(reader)
    }
}
///Register `USE2FS` writer
pub struct W(crate::W<USE2FS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USE2FS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<USE2FS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USE2FS_SPEC>) -> Self {
        W(writer)
    }
}
///Use 2FS register
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USE2FS_A {
    ///0: Use 1FS output for PCM data.
    USE_1FS = 0,
    ///1: Use 2FS output for PCM data.
    USE_2FS = 1,
}
impl From<USE2FS_A> for bool {
    #[inline(always)]
    fn from(variant: USE2FS_A) -> Self {
        variant as u8 != 0
    }
}
///Field `USE2FS` reader - Use 2FS register
pub struct USE2FS_R(crate::FieldReader<bool, USE2FS_A>);
impl USE2FS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USE2FS_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> USE2FS_A {
        match self.bits {
            false => USE2FS_A::USE_1FS,
            true => USE2FS_A::USE_2FS,
        }
    }
    ///Checks if the value of the field is `USE_1FS`
    #[inline(always)]
    pub fn is_use_1fs(&self) -> bool {
        **self == USE2FS_A::USE_1FS
    }
    ///Checks if the value of the field is `USE_2FS`
    #[inline(always)]
    pub fn is_use_2fs(&self) -> bool {
        **self == USE2FS_A::USE_2FS
    }
}
impl core::ops::Deref for USE2FS_R {
    type Target = crate::FieldReader<bool, USE2FS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `USE2FS` writer - Use 2FS register
pub struct USE2FS_W<'a> {
    w: &'a mut W,
}
impl<'a> USE2FS_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: USE2FS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Use 1FS output for PCM data.
    #[inline(always)]
    pub fn use_1fs(self) -> &'a mut W {
        self.variant(USE2FS_A::USE_1FS)
    }
    ///Use 2FS output for PCM data.
    #[inline(always)]
    pub fn use_2fs(self) -> &'a mut W {
        self.variant(USE2FS_A::USE_2FS)
    }
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    ///Bit 0 - Use 2FS register
    #[inline(always)]
    pub fn use2fs(&self) -> USE2FS_R {
        USE2FS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - Use 2FS register
    #[inline(always)]
    pub fn use2fs(&mut self) -> USE2FS_W {
        USE2FS_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Use 2FS register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [use2fs](index.html) module
pub struct USE2FS_SPEC;
impl crate::RegisterSpec for USE2FS_SPEC {
    type Ux = u32;
}
///`read()` method returns [use2fs::R](R) reader structure
impl crate::Readable for USE2FS_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [use2fs::W](W) writer structure
impl crate::Writable for USE2FS_SPEC {
    type Writer = W;
}
///`reset()` method sets USE2FS to value 0
impl crate::Resettable for USE2FS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
