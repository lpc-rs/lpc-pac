#[doc = "Reader of register USE2FS"]
pub type R = crate::R<u32, super::USE2FS>;
#[doc = "Writer for register USE2FS"]
pub type W = crate::W<u32, super::USE2FS>;
#[doc = "Register USE2FS `reset()`'s with value 0"]
impl crate::ResetValue for super::USE2FS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Use 2FS register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USE2FS_A {
    #[doc = "0: Use 1FS output for PCM data."]
    USE_1FS = 0,
    #[doc = "1: Use 2FS output for PCM data."]
    USE_2FS = 1,
}
impl From<USE2FS_A> for bool {
    #[inline(always)]
    fn from(variant: USE2FS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `USE2FS`"]
pub type USE2FS_R = crate::R<bool, USE2FS_A>;
impl USE2FS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USE2FS_A {
        match self.bits {
            false => USE2FS_A::USE_1FS,
            true => USE2FS_A::USE_2FS,
        }
    }
    #[doc = "Checks if the value of the field is `USE_1FS`"]
    #[inline(always)]
    pub fn is_use_1fs(&self) -> bool {
        *self == USE2FS_A::USE_1FS
    }
    #[doc = "Checks if the value of the field is `USE_2FS`"]
    #[inline(always)]
    pub fn is_use_2fs(&self) -> bool {
        *self == USE2FS_A::USE_2FS
    }
}
#[doc = "Write proxy for field `USE2FS`"]
pub struct USE2FS_W<'a> {
    w: &'a mut W,
}
impl<'a> USE2FS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USE2FS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Use 1FS output for PCM data."]
    #[inline(always)]
    pub fn use_1fs(self) -> &'a mut W {
        self.variant(USE2FS_A::USE_1FS)
    }
    #[doc = "Use 2FS output for PCM data."]
    #[inline(always)]
    pub fn use_2fs(self) -> &'a mut W {
        self.variant(USE2FS_A::USE_2FS)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Use 2FS register"]
    #[inline(always)]
    pub fn use2fs(&self) -> USE2FS_R {
        USE2FS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Use 2FS register"]
    #[inline(always)]
    pub fn use2fs(&mut self) -> USE2FS_W {
        USE2FS_W { w: self }
    }
}
