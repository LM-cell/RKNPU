project('npu_benchmark', 'c', version: '1.0.0')

incdir = include_directories('include')
lib_src = ['src/npu_interface.c','src/npu_matmul.c']
thread_dep = dependency('threads')
# lib = library('rk3588-npu',lib_src, include_directories : incdir)
lib = static_library('rk3588-npu', lib_src, include_directories : incdir)

test_matmul_4_36_16  = executable('matmul_4_36_16', 'tests/matmul_4_36_16.c', include_directories : incdir, link_with : lib)
test('matmul_4x36x16',test_matmul_4_36_16, is_parallel : false)

# Test inputs fp16 and output fp32
test_matmul_fp16  = executable('matmul_fp16', 'tests/matmul_fp16.c', include_directories : incdir, link_with : lib, link_args : '-lm')
test('matmul fp16 1x32x16',test_matmul_fp16, is_parallel : false , args : ['1', '32' ,'16'])
test('matmul fp16 1x64x64',test_matmul_fp16, is_parallel : false , args : ['1', '64' ,'64'])
test('matmul fp16 1x1024x1024',test_matmul_fp16, is_parallel : false , args : ['1', '1024' ,'1024'])
test('matmul fp16 1x4096x4096',test_matmul_fp16, is_parallel : false , args : ['1', '4096' ,'4096'])
test('matmul fp16 4x32x16',test_matmul_fp16, is_parallel : false , args : ['4', '32' ,'16'])
# test max feature data for one task
test('matmul fp16 384x384x4096',test_matmul_fp16, is_parallel : false , args : ['384', '384' ,'4096'])

matmul_multi_process = executable(
    'matmul_multi_process',
    'tests/matmul_multi_process.c',
    include_directories : incdir,
    link_with : lib,
    link_args : '-lm'
)
test(
    'matmul multi process',
    matmul_multi_process,
    is_parallel : false,
    timeout : 600
)

matmul_llama_concurrent = executable(
    'matmul_llama_concurrent',
    'tests/matmul_llama_concurrent.c'
)

three_core_task = executable(
    'three_core_task',
    'tests/three_core_task.c',
    include_directories : incdir,
    link_with : lib,
    link_args : '-lm'
)

core_scaling_benchmark = executable(
    'core_scaling_benchmark',
    'tests/core_scaling_benchmark.c',
    include_directories : incdir,
    link_with : lib,
    dependencies : thread_dep,
    link_args : '-lm'
)

rknpu_action_smoke = executable(
    'rknpu_action_smoke',
    'tests/rknpu_action_smoke.c',
    include_directories : incdir,
    link_with : lib
)
test('rknpu action smoke', rknpu_action_smoke, is_parallel : false)

test_matmul_int8  = executable('matmul_int8', 'tests/matmul_int8.c', include_directories : incdir, link_with : lib, link_args : '-lm')
test('matmul int8 1x32x32',test_matmul_int8, is_parallel : false , args : ['1','32','32'])
test('matmul int8 1x64x64',test_matmul_int8, is_parallel : false , args : ['1','64','64'])
test('matmul int8 1x1024x1024',test_matmul_int8, is_parallel : false , args : ['1','1024','1024'])
test('matmul int8 1x4096x4096',test_matmul_int8, is_parallel : false , args : ['1','4096','4096'])
# test max feature data for one task
test('matmul int8 544x544x4096',test_matmul_int8, is_parallel : false , args : ['544','544','4096'])

# Test inputs fp16 and output fp16
test_matmul_fp16_fp16  = executable('matmul_fp16_fp16', 'tests/matmul_fp16_fp16.c', include_directories : incdir, link_with : lib, link_args : '-lm')
test('matmul fp16_fp16 1x32x16',test_matmul_fp16_fp16, is_parallel : false , args : ['1', '32' ,'16'])
test('matmul fp16_fp16 1x768x768',test_matmul_fp16_fp16, is_parallel : false , args : ['1', '768' ,'768'])
test('matmul fp16_fp16 1x768x2048',test_matmul_fp16_fp16, is_parallel : false , args : ['1', '768' ,'2048'])
test('matmul fp16_fp16 1x8192x8192',test_matmul_fp16_fp16, is_parallel : false , args : ['1', '8192' ,'8192'])

# Benchmark executable
bench_mark = executable('bench_mark', 'tests/bench_mark.c', include_directories : incdir, link_with : lib, link_args : '-lm')

# Benchmark tests with different iteration counts
# Quick test (10 iterations) - for fast verification
test('benchmark quick', bench_mark,
     is_parallel : false,
     args : ['10'],
     timeout : 60)

# Fast test (100 iterations) - for regular testing
test('benchmark fast', bench_mark,
     is_parallel : false,
     args : ['100'],
     timeout : 300)

# Standard test (500 iterations) - for proper benchmarking
test('benchmark standard', bench_mark,
     is_parallel : false,
     args : ['500'],
     timeout : 1200)
