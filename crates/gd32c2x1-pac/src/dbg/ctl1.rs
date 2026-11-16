#[doc = "Register `CTL1` reader"]
pub type R = crate::R<Ctl1Spec>;
#[doc = "Register `CTL1` writer"]
pub type W = crate::W<Ctl1Spec>;
#[doc = "Field `RTC_HOLD` reader - RTC hold bit"]
pub type RtcHoldR = crate::BitReader;
#[doc = "Field `RTC_HOLD` writer - RTC hold bit"]
pub type RtcHoldW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 10 - RTC hold bit"]
    #[inline(always)]
    pub fn rtc_hold(&self) -> RtcHoldR {
        RtcHoldR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 10 - RTC hold bit"]
    #[inline(always)]
    pub fn rtc_hold(&mut self) -> RtcHoldW<'_, Ctl1Spec> {
        RtcHoldW::new(self, 10)
    }
}
#[doc = "Control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl1Spec;
impl crate::RegisterSpec for Ctl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl1::R`](R) reader structure"]
impl crate::Readable for Ctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl1::W`](W) writer structure"]
impl crate::Writable for Ctl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTL1 to value 0"]
impl crate::Resettable for Ctl1Spec {}
