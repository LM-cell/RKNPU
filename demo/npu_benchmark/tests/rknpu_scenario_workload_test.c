/* RKNPU 场景和 lane 分配测试，最后修改日期：2026-08-12。 */

#include <assert.h>
#include <stddef.h>
#include <string.h>

#include "rknpu_scenario_workload.h"

/* 检查七个组合的 Task 数量与 core_scaling_benchmark 定义一致。 */
static void test_scenario_task_counts(void) {
    static const struct {
        const char *name;
        uint32_t tasks;
    } expected[] = {
        {"tiny_dispatch-shared", 96U},
        {"tiny_dispatch-unique", 96U},
        {"mid_balanced-shared", 48U},
        {"mid_balanced-unique", 12U},
        {"throughput_heavy-shared", 24U},
        {"throughput_heavy-unique", 4U},
        {"llama_decode_like-shared", 48U},
    };
    size_t count;
    const rknpu_scenario_case_t *scenarios = rknpu_scenario_cases(&count);

    assert(count == sizeof(expected) / sizeof(expected[0]));
    for (size_t index = 0; index < count; index++) {
        assert(strcmp(scenarios[index].name, expected[index].name) == 0);
        assert(scenarios[index].task_count == expected[index].tasks);
        assert(rknpu_scenario_find(expected[index].name) == &scenarios[index]);
    }
    assert(rknpu_scenario_find("unknown") == NULL);
}

/* 检查不能整除时余数优先分给前面的 lane，并且 core_mask 保持不变。 */
static void test_lane_distribution(void) {
    struct rknpu_submit submit;

    memset(&submit, 0, sizeof(submit));
    submit.core_mask = 0x7U;
    rknpu_distribute_tasks_to_lanes(&submit, 10U, 3U);
    assert(submit.core_mask == 0x7U);
    assert(submit.subcore_task[0].task_start == 0U);
    assert(submit.subcore_task[0].task_number == 4U);
    assert(submit.subcore_task[1].task_start == 4U);
    assert(submit.subcore_task[1].task_number == 3U);
    assert(submit.subcore_task[2].task_start == 7U);
    assert(submit.subcore_task[2].task_number == 3U);
    assert(submit.subcore_task[3].task_number == 0U);
    assert(submit.subcore_task[4].task_number == 0U);
}

int main(void) {
    test_scenario_task_counts();
    test_lane_distribution();
    return 0;
}
