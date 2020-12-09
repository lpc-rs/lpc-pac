#[doc = "Reader of register DBADDR"]
pub type R = crate::R<u32, super::DBADDR>;
#[doc = "Writer for register DBADDR"]
pub type W = crate::W<u32, super::DBADDR>;
#[doc = "Register DBADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::DBADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SDL`"]
pub type SDL_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SDL`"]
pub struct SDL_W<'a> {
    w: &'a mut W,
}
impl<'a> SDL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Start of Descriptor List."]
    #[inline(always)]
    pub fn sdl(&self) -> SDL_R {
        SDL_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Start of Descriptor List."]
    #[inline(always)]
    pub fn sdl(&mut self) -> SDL_W {
        SDL_W { w: self }
    }
}
