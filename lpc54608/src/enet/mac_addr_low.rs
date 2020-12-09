#[doc = "Reader of register MAC_ADDR_LOW"]
pub type R = crate::R<u32, super::MAC_ADDR_LOW>;
#[doc = "Writer for register MAC_ADDR_LOW"]
pub type W = crate::W<u32, super::MAC_ADDR_LOW>;
#[doc = "Register MAC_ADDR_LOW `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::MAC_ADDR_LOW {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `A31_0`"]
pub type A31_0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `A31_0`"]
pub struct A31_0_W<'a> {
    w: &'a mut W,
}
impl<'a> A31_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - MAC Address0 \\[31:0\\]
This field contains the lower 32 bits of the 6-byte first MAC address."]
    #[inline(always)]
    pub fn a31_0(&self) -> A31_0_R {
        A31_0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - MAC Address0 \\[31:0\\]
This field contains the lower 32 bits of the 6-byte first MAC address."]
    #[inline(always)]
    pub fn a31_0(&mut self) -> A31_0_W {
        A31_0_W { w: self }
    }
}
