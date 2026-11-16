#[doc = "Register `CHCTL3` reader"]
pub type R = crate::R<Chctl3Spec>;
#[doc = "Register `CHCTL3` writer"]
pub type W = crate::W<Chctl3Spec>;
#[doc = "Field `CH4COMFEN` reader - Channel 4 output compare fast enable"]
pub type Ch4comfenR = crate::BitReader;
#[doc = "Field `CH4COMFEN` writer - Channel 4 output compare fast enable"]
pub type Ch4comfenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4COMSEN` reader - Channel 4 compare output shadow enable"]
pub type Ch4comsenR = crate::BitReader;
#[doc = "Field `CH4COMSEN` writer - Channel 4 compare output shadow enable"]
pub type Ch4comsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4COMCTL` reader - Channel 4 compare output control"]
pub type Ch4comctlR = crate::FieldReader;
#[doc = "Field `CH4COMCTL` writer - Channel 4 compare output control"]
pub type Ch4comctlW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CH4COMCEN` reader - Channel 4 output compare clear enable."]
pub type Ch4comcenR = crate::BitReader;
#[doc = "Field `CH4COMCEN` writer - Channel 4 output compare clear enable."]
pub type Ch4comcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4COMCTL_3` reader - Channel 4 compare output control"]
pub type Ch4comctl3R = crate::BitReader;
#[doc = "Field `CH4COMCTL_3` writer - Channel 4 compare output control"]
pub type Ch4comctl3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - Channel 4 output compare fast enable"]
    #[inline(always)]
    pub fn ch4comfen(&self) -> Ch4comfenR {
        Ch4comfenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 4 compare output shadow enable"]
    #[inline(always)]
    pub fn ch4comsen(&self) -> Ch4comsenR {
        Ch4comsenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Channel 4 compare output control"]
    #[inline(always)]
    pub fn ch4comctl(&self) -> Ch4comctlR {
        Ch4comctlR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Channel 4 output compare clear enable."]
    #[inline(always)]
    pub fn ch4comcen(&self) -> Ch4comcenR {
        Ch4comcenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - Channel 4 compare output control"]
    #[inline(always)]
    pub fn ch4comctl_3(&self) -> Ch4comctl3R {
        Ch4comctl3R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Channel 4 output compare fast enable"]
    #[inline(always)]
    pub fn ch4comfen(&mut self) -> Ch4comfenW<'_, Chctl3Spec> {
        Ch4comfenW::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 4 compare output shadow enable"]
    #[inline(always)]
    pub fn ch4comsen(&mut self) -> Ch4comsenW<'_, Chctl3Spec> {
        Ch4comsenW::new(self, 3)
    }
    #[doc = "Bits 4:6 - Channel 4 compare output control"]
    #[inline(always)]
    pub fn ch4comctl(&mut self) -> Ch4comctlW<'_, Chctl3Spec> {
        Ch4comctlW::new(self, 4)
    }
    #[doc = "Bit 7 - Channel 4 output compare clear enable."]
    #[inline(always)]
    pub fn ch4comcen(&mut self) -> Ch4comcenW<'_, Chctl3Spec> {
        Ch4comcenW::new(self, 7)
    }
    #[doc = "Bit 16 - Channel 4 compare output control"]
    #[inline(always)]
    pub fn ch4comctl_3(&mut self) -> Ch4comctl3W<'_, Chctl3Spec> {
        Ch4comctl3W::new(self, 16)
    }
}
#[doc = "Channel control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`chctl3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chctl3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Chctl3Spec;
impl crate::RegisterSpec for Chctl3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chctl3::R`](R) reader structure"]
impl crate::Readable for Chctl3Spec {}
#[doc = "`write(|w| ..)` method takes [`chctl3::W`](W) writer structure"]
impl crate::Writable for Chctl3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHCTL3 to value 0"]
impl crate::Resettable for Chctl3Spec {}
