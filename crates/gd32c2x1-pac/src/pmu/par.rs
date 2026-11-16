#[doc = "Register `PAR` reader"]
pub type R = crate::R<ParSpec>;
#[doc = "Register `PAR` writer"]
pub type W = crate::W<ParSpec>;
#[doc = "Field `TSW_IRC48MCNT` reader - When enter Deep-sleep/Deep-sleep1 mode, switch to IRC48M clock."]
pub type TswIrc48mcntR = crate::FieldReader;
#[doc = "Field `TSW_IRC48MCNT` writer - When enter Deep-sleep/Deep-sleep1 mode, switch to IRC48M clock."]
pub type TswIrc48mcntW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TWK_EFLASH` reader - EFLASH wake up from Deep-sleep/Deep-sleep1 mode counter."]
pub type TwkEflashR = crate::FieldReader;
#[doc = "Field `TWK_EFLASH` writer - EFLASH wake up from Deep-sleep/Deep-sleep1 mode counter."]
pub type TwkEflashW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 16:20 - When enter Deep-sleep/Deep-sleep1 mode, switch to IRC48M clock."]
    #[inline(always)]
    pub fn tsw_irc48mcnt(&self) -> TswIrc48mcntR {
        TswIrc48mcntR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:28 - EFLASH wake up from Deep-sleep/Deep-sleep1 mode counter."]
    #[inline(always)]
    pub fn twk_eflash(&self) -> TwkEflashR {
        TwkEflashR::new(((self.bits >> 21) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:20 - When enter Deep-sleep/Deep-sleep1 mode, switch to IRC48M clock."]
    #[inline(always)]
    pub fn tsw_irc48mcnt(&mut self) -> TswIrc48mcntW<'_, ParSpec> {
        TswIrc48mcntW::new(self, 16)
    }
    #[doc = "Bits 21:28 - EFLASH wake up from Deep-sleep/Deep-sleep1 mode counter."]
    #[inline(always)]
    pub fn twk_eflash(&mut self) -> TwkEflashW<'_, ParSpec> {
        TwkEflashW::new(self, 21)
    }
}
#[doc = "Parameter register\n\nYou can [`read`](crate::Reg::read) this register and get [`par::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`par::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ParSpec;
impl crate::RegisterSpec for ParSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`par::R`](R) reader structure"]
impl crate::Readable for ParSpec {}
#[doc = "`write(|w| ..)` method takes [`par::W`](W) writer structure"]
impl crate::Writable for ParSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PAR to value 0x190a_0000"]
impl crate::Resettable for ParSpec {
    const RESET_VALUE: u32 = 0x190a_0000;
}
