// Copyright (c) 2023 by Rockchip Electronics Co., Ltd. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

/*-------------------------------------------
                Includes
-------------------------------------------*/
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "yolov8.h"
#include "image_utils.h"
#include "file_utils.h"
#include "image_drawing.h"

#if defined(RV1106_1103) 
    #include "dma_alloc.hpp"
#endif

/*-------------------------------------------
                  Main Function
-------------------------------------------*/
int main(int argc, char **argv)
{
    // 使用固定的模型路径
    printf("yolov8 start\n");
    const char *model_path = "model/yolov8.rknn";
    
    // 定义要处理的图像文件数组
    const char *image_paths[] = {
        "model/picture1.jpg",
        "model/picture2.jpg",
        "model/picture3.jpg"
    };
    int image_count = sizeof(image_paths) / sizeof(image_paths[0]);

    int ret;
    rknn_app_context_t rknn_app_ctx;
    memset(&rknn_app_ctx, 0, sizeof(rknn_app_context_t));
    
    // 文件指针声明
    FILE *result_file = NULL;
    FILE *read_file = NULL;

    init_post_process();

    ret = init_yolov8_model(model_path, &rknn_app_ctx);
    if (ret != 0)
    {
        printf("init_yolov8_model fail! ret=%d model_path=%s\n", ret, model_path);
        goto out;
    }
    printf("init_yolov8_model success!\n");

    // 打开结果文件用于写入
    result_file = fopen("detection_results.txt", "w");
    if (!result_file) {
        printf("Failed to create detection_results.txt\n");
        goto out;
    }

    // 循环处理每个图像文件
    for (int i = 0; i < image_count; i++) {
        const char *image_path = image_paths[i];
        printf("Processing image: %s\n", image_path);

        image_buffer_t src_image;
        memset(&src_image, 0, sizeof(image_buffer_t));
        ret = read_image(image_path, &src_image);

        if (ret != 0)
        {
            printf("read image fail! ret=%d image_path=%s\n", ret, image_path);
            continue; // 继续处理下一个图像
        }

#if defined(RV1106_1103) 
        //RV1106 rga requires that input and output bufs are memory allocated by dma
        ret = dma_buf_alloc(RV1106_CMA_HEAP_PATH, src_image.size, &rknn_app_ctx.img_dma_buf.dma_buf_fd, 
                           (void **) & (rknn_app_ctx.img_dma_buf.dma_buf_virt_addr));
        memcpy(rknn_app_ctx.img_dma_buf.dma_buf_virt_addr, src_image.virt_addr, src_image.size);
        dma_sync_cpu_to_device(rknn_app_ctx.img_dma_buf.dma_buf_fd);
        free(src_image.virt_addr);
        src_image.virt_addr = (unsigned char *)rknn_app_ctx.img_dma_buf.dma_buf_virt_addr;
        src_image.fd = rknn_app_ctx.img_dma_buf.dma_buf_fd;
        rknn_app_ctx.img_dma_buf.size = src_image.size;
#endif

        object_detect_result_list od_results;
        ret = inference_yolov8_model(&rknn_app_ctx, &src_image, &od_results);
        if (ret != 0)
        {
            printf("inference_yolov8_model fail! ret=%d\n", ret);
#if defined(RV1106_1103) 
            dma_buf_free(rknn_app_ctx.img_dma_buf.size, &rknn_app_ctx.img_dma_buf.dma_buf_fd, 
                    rknn_app_ctx.img_dma_buf.dma_buf_virt_addr);
#else
            free(src_image.virt_addr);
#endif
            continue; // 继续处理下一个图像
        }

        // 将结果写入文件
        fprintf(result_file, "image: %s\n", image_path);
        for (int j = 0; j < od_results.count; j++)
        {
            object_detect_result *det_result = &(od_results.results[j]);
            fprintf(result_file, "%s @ (%d %d %d %d) %.3f\n", coco_cls_to_name(det_result->cls_id),
                   det_result->box.left, det_result->box.top,
                   det_result->box.right, det_result->box.bottom,
                   det_result->prop);
        }
        fprintf(result_file, "\n");

        // 画框和概率
        char text[256];
        for (int j = 0; j < od_results.count; j++)
        {
            object_detect_result *det_result = &(od_results.results[j]);
            printf("%s @ (%d %d %d %d) %.3f\n", coco_cls_to_name(det_result->cls_id),
                   det_result->box.left, det_result->box.top,
                   det_result->box.right, det_result->box.bottom,
                   det_result->prop);
            int x1 = det_result->box.left;
            int y1 = det_result->box.top;
            int x2 = det_result->box.right;
            int y2 = det_result->box.bottom;

            draw_rectangle(&src_image, x1, y1, x2 - x1, y2 - y1, COLOR_BLUE, 3);

            sprintf(text, "%s %.1f%%", coco_cls_to_name(det_result->cls_id), det_result->prop * 100);
            draw_text(&src_image, text, x1, y1 - 20, COLOR_RED, 10);
        }

        // 为每个输出文件生成不同的文件名
        char output_path[256];
        snprintf(output_path, sizeof(output_path), "out_%d.png", i+1);
        write_image(output_path, &src_image);
        printf("Result saved to: %s\n", output_path);

#if defined(RV1106_1103) 
        dma_buf_free(rknn_app_ctx.img_dma_buf.size, &rknn_app_ctx.img_dma_buf.dma_buf_fd, 
                rknn_app_ctx.img_dma_buf.dma_buf_virt_addr);
#else
        free(src_image.virt_addr);
#endif
    }

    // 关闭结果文件
    if (result_file) {
        fclose(result_file);
        result_file = NULL;
    }

    // 读取并打印结果文件内容
    printf("\n=== Detection Results Summary ===\n");
    read_file = fopen("detection_results.txt", "r");
    if (read_file) {
        char line[512];
        while (fgets(line, sizeof(line), read_file)) {
            printf("%s", line);
        }
        fclose(read_file);
        read_file = NULL;
    } else {
        printf("Failed to read detection_results.txt\n");
    }

out:
    deinit_post_process();

    ret = release_yolov8_model(&rknn_app_ctx);
    if (ret != 0)
    {
        printf("release_yolov8_model fail! ret=%d\n", ret);
    }

    // 确保文件被关闭（只在异常退出时）
    if (result_file) {
        fclose(result_file);
    }
    if (read_file) {
        fclose(read_file);
    }

    return 0;
}