#[doc = "Register `CH4CV` reader"]
pub type R = crate::R<Ch4cvSpec>;
#[doc = "Register `CH4CV` writer"]
pub type W = crate::W<Ch4cvSpec>;
#[doc = "Field `CH4VAL` reader - Capture or compare value of channel 4"]
pub type Ch4valR = crate::FieldReader<u16>;
#[doc = "Field `CH4VAL` writer - Capture or compare value of channel 4"]
pub type Ch4valW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CCH4CH0` reader - Combine Channel 4 and Channel 0"]
pub type Cch4ch0R = crate::BitReader;
#[doc = "Field `CCH4CH0` writer - Combine Channel 4 and Channel 0"]
pub type Cch4ch0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCH4CH1` reader - Combine Channel 4 and Channel 1"]
pub type Cch4ch1R = crate::BitReader;
#[doc = "Field `CCH4CH1` writer - Combine Channel 4 and Channel 1"]
pub type Cch4ch1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCH4CH2` reader - Combine Channel 4 and Channel 2"]
pub type Cch4ch2R = crate::BitReader;
#[doc = "Field `CCH4CH2` writer - Combine Channel 4 and Channel 2"]
pub type Cch4ch2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Capture or compare value of channel 4"]
    #[inline(always)]
    pub fn ch4val(&self) -> Ch4valR {
        Ch4valR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 29 - Combine Channel 4 and Channel 0"]
    #[inline(always)]
    pub fn cch4ch0(&self) -> Cch4ch0R {
        Cch4ch0R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Combine Channel 4 and Channel 1"]
    #[inline(always)]
    pub fn cch4ch1(&self) -> Cch4ch1R {
        Cch4ch1R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Combine Channel 4 and Channel 2"]
    #[inline(always)]
    pub fn cch4ch2(&self) -> Cch4ch2R {
        Cch4ch2R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Capture or compare value of channel 4"]
    #[inline(always)]
    pub fn ch4val(&mut self) -> Ch4valW<'_, Ch4cvSpec> {
        Ch4valW::new(self, 0)
    }
    #[doc = "Bit 29 - Combine Channel 4 and Channel 0"]
    #[inline(always)]
    pub fn cch4ch0(&mut self) -> Cch4ch0W<'_, Ch4cvSpec> {
        Cch4ch0W::new(self, 29)
    }
    #[doc = "Bit 30 - Combine Channel 4 and Channel 1"]
    #[inline(always)]
    pub fn cch4ch1(&mut self) -> Cch4ch1W<'_, Ch4cvSpec> {
        Cch4ch1W::new(self, 30)
    }
    #[doc = "Bit 31 - Combine Channel 4 and Channel 2"]
    #[inline(always)]
    pub fn cch4ch2(&mut self) -> Cch4ch2W<'_, Ch4cvSpec> {
        Cch4ch2W::new(self, 31)
    }
}
#[doc = "Channel 4 capture/compare value register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch4cv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4cv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch4cvSpec;
impl crate::RegisterSpec for Ch4cvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch4cv::R`](R) reader structure"]
impl crate::Readable for Ch4cvSpec {}
#[doc = "`write(|w| ..)` method takes [`ch4cv::W`](W) writer structure"]
impl crate::Writable for Ch4cvSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH4CV to value 0"]
impl crate::Resettable for Ch4cvSpec {}
