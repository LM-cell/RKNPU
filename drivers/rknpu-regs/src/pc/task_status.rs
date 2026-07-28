#[doc = "Register `TASK_STATUS` reader"]
pub type R = crate::R<TaskStatusSpec>;
#[doc = "Register `TASK_STATUS` writer"]
pub type W = crate::W<TaskStatusSpec>;
#[doc = "Field `TASK_STATUS` reader - 任务状态（见下表）"]
pub type TaskStatusR = crate::FieldReader<u32>;
#[doc = "Field `TASK_STATUS` writer - 任务状态（见下表）"]
pub type TaskStatusW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:27 - 任务状态（见下表）"]
    #[inline(always)]
    pub fn task_status(&self) -> TaskStatusR {
        TaskStatusR::new(self.bits & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:27 - 任务状态（见下表）"]
    #[inline(always)]
    pub fn task_status(&mut self) -> TaskStatusW<'_, TaskStatusSpec> {
        TaskStatusW::new(self, 0)
    }
}
#[doc = "task_status\n\nYou can [`read`](crate::Reg::read) this register and get [`task_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`task_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TaskStatusSpec;
impl crate::RegisterSpec for TaskStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`task_status::R`](R) reader structure"]
impl crate::Readable for TaskStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`task_status::W`](W) writer structure"]
impl crate::Writable for TaskStatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASK_STATUS to value 0"]
impl crate::Resettable for TaskStatusSpec {}
