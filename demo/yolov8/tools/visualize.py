import cv2
import os
import re
import subprocess
from pathlib import Path

def parse_detection_file(file_path):
    """
    解析检测结果文件，提取图片路径和对应的检测结果
    
    Args:
        file_path: 检测结果文件路径
        
    Returns:
        dict: 键为图片路径，值为检测结果列表
    """
    detection_data = {}
    
    try:
        with open(file_path, 'r', encoding='utf-8') as file:
            content = file.read()
        
        # 按照 "image:" 分割内容，获取每个图片块
        image_blocks = re.split(r'\n(?=image:)', content)
        
        # 过滤掉空块和不包含图片信息的块
        image_blocks = [block.strip() for block in image_blocks if block.strip() and 'image:' in block]
        
        print(f"找到 {len(image_blocks)} 个图片块")
        
        for block in image_blocks:
            # 提取图片路径
            image_match = re.match(r'image:\s*(.+)', block)
            if not image_match:
                continue
                
            image_path = image_match.group(1).strip()
            
            # 将 .data 文件路径转换为 .jpg 文件路径
            if image_path.endswith('.data'):
                jpg_path = image_path.replace('.data', '.jpg')
            else:
                jpg_path = image_path
            
            # 去掉 model/ 文件夹前缀
            if jpg_path.startswith('model/'):
                jpg_path = jpg_path[len('model/'):]
            
            # 提取检测结果行
            detection_list = []
            lines = block.split('\n')[1:]  # 跳过第一行（image: ...）
            
            for line in lines:
                line = line.strip()
                if '@' in line and line:
                    # 解析格式: "person @ (211 241 282 506) 0.864"
                    match = re.match(r'(.+?)\s*@\s*\((\d+)\s+(\d+)\s+(\d+)\s+(\d+)\)\s+([\d.]+)', line)
                    if match:
                        class_name = match.group(1).strip()
                        x1, y1, x2, y2 = map(int, match.group(2, 3, 4, 5))
                        confidence = float(match.group(6))
                        
                        detection_list.append({
                            'class_name': class_name,
                            'bbox': [x1, y1, x2, y2],
                            'confidence': confidence
                        })
            
            detection_data[jpg_path] = detection_list
            print(f"解析到图片: {jpg_path}, 检测目标数: {len(detection_list)}")
            
    except Exception as e:
        print(f"解析检测文件时出错: {e}")
        import traceback
        traceback.print_exc()
    
    return detection_data

def draw_detections_on_image(image_path, detections, output_dir="."):
    """
    在图片上绘制检测结果并保存（增强可见性版本）
    """
    try:
        # 读取图片
        if not os.path.exists(image_path):
            print(f"警告: 图片文件不存在: {image_path}")
            return None
            
        image = cv2.imread(image_path)
        if image is None:
            print(f"警告: 无法读取图片: {image_path}")
            return None
        
        # 获取图片尺寸
        height, width = image.shape[:2]
        
        # 增强可见性的参数设置
        line_thickness = max(3, int(min(width, height) / 300))     # 更粗的边界框线条
        font_scale = max(0.6, min(width, height) / 600)            # 更大的字体大小
        font_thickness = max(2, int(min(width, height) / 400))     # 更粗的字体线条
        
        # 定义高对比度的颜色（增强亮度）
        colors = {
            'person': (0, 255, 0),     # 亮绿色
            'bus': (255, 165, 0),      # 橙色（更明显）
            'car': (0, 200, 255),      # 亮蓝色
            'bicycle': (255, 0, 255),  # 洋红色
            'motorcycle': (255, 255, 0), # 青色
            'truck': (0, 255, 255),    # 黄色
            'default': (255, 200, 0)   # 金色（默认）
        }
        
        # 绘制每个检测结果
        for i, detection in enumerate(detections):
            class_name = detection['class_name']
            x1, y1, x2, y2 = detection['bbox']
            confidence = detection['confidence']
            
            # 选择颜色
            color = colors.get(class_name.lower(), colors['default'])
            
            # 绘制更粗的边界框
            cv2.rectangle(image, (x1, y1), (x2, y2), color, line_thickness, lineType=cv2.LINE_AA)
            
            # 准备标签文本 - 格式为："类别 百分比!"
            confidence_percent = f"{confidence*100:.1f}%"
            label = f"{class_name} {confidence_percent}!"
            
            # 计算标签尺寸
            label_size = cv2.getTextSize(label, cv2.FONT_HERSHEY_SIMPLEX, font_scale, font_thickness)[0]
            
            # 调整标签位置 - 放在边界框上方居中位置
            label_x = x1
            label_y = max(y1 - 10, 20)  # 确保标签不会超出图片顶部
            
            # 如果标签宽度超过边界框宽度，则调整位置
            if label_x + label_size[0] > x2:
                label_x = max(0, x2 - label_size[0])
            
            # 添加文字阴影效果增强可读性
            shadow_color = (0, 0, 0)  # 黑色阴影
            shadow_offset = 1
            
            # 绘制文字阴影
            cv2.putText(image, label, (label_x + shadow_offset, label_y + shadow_offset), 
                       cv2.FONT_HERSHEY_SIMPLEX, font_scale, shadow_color, font_thickness, cv2.LINE_AA)
            
            # 绘制主文字（使用与边界框相同的颜色）
            cv2.putText(image, label, (label_x, label_y), 
                       cv2.FONT_HERSHEY_SIMPLEX, font_scale, color, font_thickness, cv2.LINE_AA)
        
        # 生成输出文件名
        original_path = Path(image_path)
        output_filename = f"{original_path.stem}_detected{original_path.suffix}"
        output_path = os.path.join(output_dir, output_filename)
        
        # 保存图片（提高质量）
        cv2.imwrite(output_path, image, [int(cv2.IMWRITE_JPEG_QUALITY), 95])
        print(f"✓ 已保存: {output_path}")
        
        return output_path
        
    except Exception as e:
        print(f"处理图片 {image_path} 时出错: {e}")
        return None

def process_all_detections(detection_file_path, output_dir="."):
    """
    处理所有检测结果
    
    Args:
        detection_file_path: 检测结果文件路径
        output_dir: 输出目录
    """
    # 创建输出目录
    os.makedirs(output_dir, exist_ok=True)
    
    # 解析检测文件
    print("开始解析检测结果文件...")
    detection_data = parse_detection_file(detection_file_path)
    
    if not detection_data:
        print("未找到有效的检测结果")
        return []
    
    print(f"\n找到 {len(detection_data)} 张图片的检测结果")
    print("开始处理图片...")
    print("-" * 50)
    
    # 处理统计
    stats = {
        'total': len(detection_data),
        'success': 0,
        'failed': 0
    }
    
    # 存储成功处理的图片路径
    processed_images = []
    
    # 处理每张图片
    for i, (image_path, detections) in enumerate(detection_data.items(), 1):
        print(f"[{i}/{stats['total']}] 处理: {image_path}")
        
        result = draw_detections_on_image(image_path, detections, output_dir)
        
        if result:
            processed_images.append(result)
            stats['success'] += 1
        else:
            stats['failed'] += 1
        print()
    
    # 输出统计结果
    print("=" * 50)
    print("处理完成！")
    print(f"总图片数: {stats['total']}")
    print(f"成功处理: {stats['success']}")
    print(f"处理失败: {stats['failed']}")
    print(f"输出目录: {os.path.abspath(output_dir)}")
    
    return processed_images

def open_images_with_xdg_open(image_paths):
    """
    使用 xdg-open 命令打开图片
    
    Args:
        image_paths: 图片路径列表
    """
    if not image_paths:
        print("没有图片需要打开")
        return
    
    try:
        print(f"\n正在打开 {len(image_paths)} 张处理后的图片...")
        # 使用 xdg-open 打开所有图片
        for image_path in image_paths:
            subprocess.run(['xdg-open', image_path], check=True)
        print("图片已在默认图片查看器中打开")
    except subprocess.CalledProcessError as e:
        print(f"打开图片时出错: {e}")
    except FileNotFoundError:
        print("错误: 系统中未找到 xdg-open 命令")

def main():
    """
    主函数
    """
    # 配置参数
    detection_file_path = "detection_results.txt"  # 检测结果文件路径
    output_dir = "./detected_images"              # 输出目录
    
    print("批量检测结果可视化脚本")
    print("=" * 50)
    
    # 检查检测文件是否存在
    if not os.path.exists(detection_file_path):
        print(f"错误: 检测结果文件不存在: {detection_file_path}")
        print("请将检测结果文件放在当前目录下，或修改detection_file_path变量")
        return
    
    try:
        # 处理所有检测结果
        processed_images = process_all_detections(detection_file_path, output_dir)
        
        # 处理完成后打开所有生成的图片
        if processed_images:
            open_images_with_xdg_open(processed_images)
        
    except Exception as e:
        print(f"处理过程中发生错误: {e}")
        import traceback
        traceback.print_exc()

if __name__ == "__main__":
    main()