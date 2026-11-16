#[doc = "Register `DCRP_EADDR0` reader"]
pub type R = crate::R<DcrpEaddr0Spec>;
#[doc = "Register `DCRP_EADDR0` writer"]
pub type W = crate::W<DcrpEaddr0Spec>;
#[doc = "Field `DCRP0_EADDR` reader - DCRP area 0 end address offset"]
pub type Dcrp0EaddrR = crate::FieldReader;
#[doc = "Field `DCRP0_EADDR` writer - DCRP area 0 end address offset"]
pub type Dcrp0EaddrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `DCRP_EREN` reader - DCRP area erase enable configuration bit."]
pub type DcrpErenR = crate::BitReader;
#[doc = "Field `DCRP_EREN` writer - DCRP area erase enable configuration bit."]
pub type DcrpErenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - DCRP area 0 end address offset"]
    #[inline(always)]
    pub fn dcrp0_eaddr(&self) -> Dcrp0EaddrR {
        Dcrp0EaddrR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 31 - DCRP area erase enable configuration bit."]
    #[inline(always)]
    pub fn dcrp_eren(&self) -> DcrpErenR {
        DcrpErenR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - DCRP area 0 end address offset"]
    #[inline(always)]
    pub fn dcrp0_eaddr(&mut self) -> Dcrp0EaddrW<'_, DcrpEaddr0Spec> {
        Dcrp0EaddrW::new(self, 0)
    }
    #[doc = "Bit 31 - DCRP area erase enable configuration bit."]
    #[inline(always)]
    pub fn dcrp_eren(&mut self) -> DcrpErenW<'_, DcrpEaddr0Spec> {
        DcrpErenW::new(self, 31)
    }
}
#[doc = "DCRP end address register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcrp_eaddr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcrp_eaddr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcrpEaddr0Spec;
impl crate::RegisterSpec for DcrpEaddr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcrp_eaddr0::R`](R) reader structure"]
impl crate::Readable for DcrpEaddr0Spec {}
#[doc = "`write(|w| ..)` method takes [`dcrp_eaddr0::W`](W) writer structure"]
impl crate::Writable for DcrpEaddr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DCRP_EADDR0 to value 0"]
impl crate::Resettable for DcrpEaddr0Spec {}
