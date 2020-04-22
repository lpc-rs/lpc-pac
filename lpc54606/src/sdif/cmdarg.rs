#[doc = "Reader of register CMDARG"]
pub type R = crate::R<u32, super::CMDARG>;
#[doc = "Writer for register CMDARG"]
pub type W = crate::W<u32, super::CMDARG>;
#[doc = "Register CMDARG `reset()`'s with value 0"]
impl crate::ResetValue for super::CMDARG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CMD_ARG`"]
pub type CMD_ARG_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CMD_ARG`"]
pub struct CMD_ARG_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_ARG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Value indicates command argument to be passed to card."]
    #[inline(always)]
    pub fn cmd_arg(&self) -> CMD_ARG_R {
        CMD_ARG_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Value indicates command argument to be passed to card."]
    #[inline(always)]
    pub fn cmd_arg(&mut self) -> CMD_ARG_W {
        CMD_ARG_W { w: self }
    }
}
