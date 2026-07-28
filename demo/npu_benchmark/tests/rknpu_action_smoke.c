#include <errno.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include <sys/ioctl.h>
#include <unistd.h>

#include "npu_interface.h"
#include "rknpu-ioctl.h"

static int run_action(int fd, uint32_t flag, uint32_t *value) {
  struct rknpu_action act = {
      .flags = flag,
      .value = value ? *value : 0,
  };

  int ret = ioctl(fd, DRM_IOCTL_RKNPU_ACTION, &act);
  if (ret == 0 && value) {
    *value = act.value;
  }
  return ret;
}

static bool expect_action_ok(int fd, uint32_t flag, uint32_t *value,
                             const char *name) {
  if (run_action(fd, flag, value) < 0) {
    printf("FAIL %-24s errno=%d (%s)\n", name, errno, strerror(errno));
    return false;
  }

  printf("PASS %-24s value=%u (0x%x)\n", name, value ? *value : 0,
         value ? *value : 0);
  return true;
}

static bool expect_action_fail(int fd, uint32_t flag, uint32_t value,
                               const char *name) {
  if (run_action(fd, flag, &value) == 0) {
    printf("FAIL %-24s expected failure, got value=%u (0x%x)\n", name, value,
           value);
    return false;
  }

  printf("PASS %-24s failed as expected errno=%d (%s)\n", name, errno,
         strerror(errno));
  return true;
}

int main(void) {
  bool ok = true;
  int fd = npu_open();
  if (fd < 0) {
    return 1;
  }

  uint32_t value = 0;

  value = 0;
  ok &= expect_action_ok(fd, RKNPU_GET_HW_VERSION, &value, "GET_HW_VERSION");

  value = 0;
  ok &= expect_action_ok(fd, RKNPU_GET_DRV_VERSION, &value, "GET_DRV_VERSION");

  value = 0;
  ok &= expect_action_ok(fd, RKNPU_ACT_RESET, &value, "ACT_RESET");

  value = 0;
  ok &= expect_action_ok(fd, RKNPU_ACT_CLR_TOTAL_RW_AMOUNT, &value,
                         "ACT_CLR_TOTAL_RW_AMOUNT");

  value = 0;
  ok &= expect_action_ok(fd, RKNPU_GET_DT_WR_AMOUNT, &value, "GET_DT_WR_AMOUNT");

  value = 0;
  ok &= expect_action_ok(fd, RKNPU_GET_DT_RD_AMOUNT, &value, "GET_DT_RD_AMOUNT");

  value = 0;
  ok &= expect_action_ok(fd, RKNPU_GET_WT_RD_AMOUNT, &value, "GET_WT_RD_AMOUNT");

  value = 0;
  ok &= expect_action_ok(fd, RKNPU_GET_TOTAL_RW_AMOUNT, &value,
                         "GET_TOTAL_RW_AMOUNT");

  value = 0;
  ok &= expect_action_ok(fd, RKNPU_GET_IOMMU_EN, &value, "GET_IOMMU_EN");

  value = 0;
  ok &= expect_action_ok(fd, RKNPU_GET_TOTAL_SRAM_SIZE, &value,
                         "GET_TOTAL_SRAM_SIZE");

  value = 0;
  ok &= expect_action_ok(fd, RKNPU_GET_FREE_SRAM_SIZE, &value,
                         "GET_FREE_SRAM_SIZE");

  value = 0;
  ok &= expect_action_ok(fd, RKNPU_GET_IOMMU_DOMAIN_ID, &value,
                         "GET_IOMMU_DOMAIN_ID");

  value = 7;
  ok &= expect_action_ok(fd, RKNPU_SET_IOMMU_DOMAIN_ID, &value,
                         "SET_IOMMU_DOMAIN_ID");

  value = 0;
  ok &= expect_action_ok(fd, RKNPU_GET_IOMMU_DOMAIN_ID, &value,
                         "GET_IOMMU_DOMAIN_ID");
  if (value != 7) {
    printf("FAIL %-24s expected 7, got %u\n", "GET_IOMMU_DOMAIN_ID", value);
    ok = false;
  }

  value = 0;
  ok &= expect_action_ok(fd, RKNPU_SET_IOMMU_DOMAIN_ID, &value,
                         "RESET_IOMMU_DOMAIN_ID");

  ok &= expect_action_fail(fd, RKNPU_SET_IOMMU_DOMAIN_ID, 16,
                           "SET_IOMMU_DOMAIN_ID_BAD");

  npu_close(fd);
  return ok ? 0 : 1;
}
