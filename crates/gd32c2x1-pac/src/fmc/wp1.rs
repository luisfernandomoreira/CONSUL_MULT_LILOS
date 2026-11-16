#[doc = "Register `WP1` reader"]
pub type R = crate::R<Wp1Spec>;
#[doc = "Register `WP1` writer"]
pub type W = crate::W<Wp1Spec>;
#[doc = "Field `WP1_SADDR` reader - Write protection area 1 start address offset"]
pub type Wp1SaddrR = crate::FieldReader;
#[doc = "Field `WP1_SADDR` writer - Write protection area 1 start address offset"]
pub type Wp1SaddrW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `WP1_EADDR` reader - Write protection area 1 end address offset"]
pub type Wp1EaddrR = crate::FieldReader;
#[doc = "Field `WP1_EADDR` writer - Write protection area 1 end address offset"]
pub type Wp1EaddrW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Write protection area 1 start address offset"]
    #[inline(always)]
    pub fn wp1_saddr(&self) -> Wp1SaddrR {
        Wp1SaddrR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Write protection area 1 end address offset"]
    #[inline(always)]
    pub fn wp1_eaddr(&self) -> Wp1EaddrR {
        Wp1EaddrR::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Write protection area 1 start address offset"]
    #[inline(always)]
    pub fn wp1_saddr(&mut self) -> Wp1SaddrW<'_, Wp1Spec> {
        Wp1SaddrW::new(self, 0)
    }
    #[doc = "Bits 16:21 - Write protection area 1 end address offset"]
    #[inline(always)]
    pub fn wp1_eaddr(&mut self) -> Wp1EaddrW<'_, Wp1Spec> {
        Wp1EaddrW::new(self, 16)
    }
}
#[doc = "Write protection area 1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`wp1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wp1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wp1Spec;
impl crate::RegisterSpec for Wp1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wp1::R`](R) reader structure"]
impl crate::Readable for Wp1Spec {}
#[doc = "`write(|w| ..)` method takes [`wp1::W`](W) writer structure"]
impl crate::Writable for Wp1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WP1 to value 0"]
impl crate::Resettable for Wp1Spec {}
