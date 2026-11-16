#[doc = "Register `WP0` reader"]
pub type R = crate::R<Wp0Spec>;
#[doc = "Register `WP0` writer"]
pub type W = crate::W<Wp0Spec>;
#[doc = "Field `WP0_SADDR` reader - Write protection area 0 end address offset"]
pub type Wp0SaddrR = crate::FieldReader;
#[doc = "Field `WP0_SADDR` writer - Write protection area 0 end address offset"]
pub type Wp0SaddrW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `WP0_EADDR` reader - Write protection area 0 end address offset"]
pub type Wp0EaddrR = crate::FieldReader;
#[doc = "Field `WP0_EADDR` writer - Write protection area 0 end address offset"]
pub type Wp0EaddrW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Write protection area 0 end address offset"]
    #[inline(always)]
    pub fn wp0_saddr(&self) -> Wp0SaddrR {
        Wp0SaddrR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Write protection area 0 end address offset"]
    #[inline(always)]
    pub fn wp0_eaddr(&self) -> Wp0EaddrR {
        Wp0EaddrR::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Write protection area 0 end address offset"]
    #[inline(always)]
    pub fn wp0_saddr(&mut self) -> Wp0SaddrW<'_, Wp0Spec> {
        Wp0SaddrW::new(self, 0)
    }
    #[doc = "Bits 16:21 - Write protection area 0 end address offset"]
    #[inline(always)]
    pub fn wp0_eaddr(&mut self) -> Wp0EaddrW<'_, Wp0Spec> {
        Wp0EaddrW::new(self, 16)
    }
}
#[doc = "Write protection area 0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`wp0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wp0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wp0Spec;
impl crate::RegisterSpec for Wp0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wp0::R`](R) reader structure"]
impl crate::Readable for Wp0Spec {}
#[doc = "`write(|w| ..)` method takes [`wp0::W`](W) writer structure"]
impl crate::Writable for Wp0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WP0 to value 0"]
impl crate::Resettable for Wp0Spec {}
