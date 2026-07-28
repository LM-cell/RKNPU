#include "libnpu.h"

#include <errno.h>
#include <inttypes.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static size_t parse_size(int argc, char **argv) {
  char *end = NULL;
  unsigned long long value = 4096;

  if (argc <= 1) {
    return (size_t)value;
  }

  errno = 0;
  value = strtoull(argv[1], &end, 0);
  if (errno != 0 || end == argv[1] || *end != '\0' || value == 0) {
    fprintf(stderr, "invalid size: %s\n", argv[1]);
    exit(2);
  }

  return (size_t)value;
}

int main(int argc, char **argv) {
  size_t size = parse_size(argc, argv);
  uint64_t dma_addr = 0;
  uint8_t *cpu_ptr = dma_malloc(size, &dma_addr);
  uint64_t checksum = 0;

  if (cpu_ptr == NULL) {
    fprintf(stderr, "dma_malloc(%zu) failed: %s\n", size, strerror(errno));
    return 1;
  }

  printf("dma_malloc ok\n");
  printf("  size     = %zu bytes\n", size);
  printf("  cpu_ptr  = %p\n", (void *)cpu_ptr);
  printf("  dma_addr = 0x%016" PRIx64 "\n", dma_addr);

  for (size_t i = 0; i < size; ++i) {
    cpu_ptr[i] = (uint8_t)(i & 0xffu);
    checksum += cpu_ptr[i];
  }

  printf("pattern written\n");
  printf("  first    = 0x%02x\n", cpu_ptr[0]);
  printf("  last     = 0x%02x\n", cpu_ptr[size - 1]);
  printf("  checksum = %" PRIu64 "\n", checksum);

  memset(cpu_ptr, 0, size);
  if (dma_free(cpu_ptr) != 0) {
    fprintf(stderr, "dma_free(%p) failed: %s\n", (void *)cpu_ptr, strerror(errno));
    return 1;
  }

  puts("dma_free ok");
  return 0;
}
