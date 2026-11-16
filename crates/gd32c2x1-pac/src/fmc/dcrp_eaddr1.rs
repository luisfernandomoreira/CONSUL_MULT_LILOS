#[doc = "Register `DCRP_EADDR1` reader"]
pub type R = crate::R<DcrpEaddr1Spec>;
#[doc = "Register `DCRP_EADDR1` writer"]
pub type W = crate::W<DcrpEaddr1Spec>;
#[doc = "Field `DCRP1_EADDR` reader - DCRP area 1 end address offset"]
pub type Dcrp1EaddrR = crate::FieldReader;
#[doc = "Field `DCRP1_EADDR` writer - DCRP area 1 end address offset"]
pub type Dcrp1EaddrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - DCRP area 1 end address offset"]
    #[inline(always)]
    pub fn dcrp1_eaddr(&self) -> Dcrp1EaddrR {
        Dcrp1EaddrR::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - DCRP area 1 end address offset"]
    #[inline(always)]
    pub fn dcrp1_eaddr(&mut self) -> Dcrp1EaddrW<'_, DcrpEaddr1Spec> {
        Dcrp1EaddrW::new(self, 0)
    }
}
#[doc = "DCRP end address register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcrp_eaddr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcrp_eaddr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcrpEaddr1Spec;
impl crate::RegisterSpec for DcrpEaddr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcrp_eaddr1::R`](R) reader structure"]
impl crate::Readable for DcrpEaddr1Spec {}
#[doc = "`write(|w| ..)` method takes [`dcrp_eaddr1::W`](W) writer structure"]
impl crate::Writable for DcrpEaddr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DCRP_EADDR1 to value 0"]
impl crate::Resettable for DcrpEaddr1Spec {}
