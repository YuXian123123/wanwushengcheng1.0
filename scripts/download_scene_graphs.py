"""
下载场景图训练数据集

从 HuggingFace 下载 openvocab-scene-graphs 数据集，
转换为我们的训练格式。
"""

import json
import os
from pathlib import Path

def download_via_huggingface():
    """使用 huggingface datasets 库下载"""
    try:
        from datasets import load_dataset
        print("正在从 HuggingFace 下载数据集...")

        # 下载数据集
        dataset = load_dataset("anhkhoa1804/openvocab-scene-graphs", split="train")

        print(f"下载完成！共 {len(dataset)} 条数据")

        # 查看数据结构
        print("\n数据字段:", dataset.column_names)
        print("\n第一条数据示例:")
        example = dataset[0]
        for key, value in example.items():
            if isinstance(value, list):
                print(f"  {key}: {type(value).__name__} (len={len(value)})")
                if len(value) > 0:
                    print(f"    示例: {value[:3]}...")
            else:
                print(f"  {key}: {value}")

        return dataset

    except ImportError:
        print("需要安装 datasets 库: pip install datasets")
        return None
    except Exception as e:
        print(f"下载失败: {e}")
        return None


def download_via_requests():
    """直接下载 JSONL 文件"""
    import requests

    base_url = "https://huggingface.co/datasets/anhkhoa1804/openvocab-scene-graphs/resolve/main"
    files = ["gqa_train.jsonl", "vg_train.jsonl"]

    output_dir = Path("data/training/scene_graphs")
    output_dir.mkdir(parents=True, exist_ok=True)

    for filename in files:
        url = f"{base_url}/{filename}?download=true"
        output_path = output_dir / filename

        if output_path.exists():
            print(f"{filename} 已存在，跳过下载")
            continue

        print(f"正在下载 {filename}...")

        try:
            response = requests.get(url, stream=True)
            total_size = int(response.headers.get('content-length', 0))

            with open(output_path, 'wb') as f:
                downloaded = 0
                for chunk in response.iter_content(chunk_size=8192):
                    f.write(chunk)
                    downloaded += len(chunk)
                    if total_size > 0:
                        percent = (downloaded / total_size) * 100
                        print(f"\r  进度: {percent:.1f}%", end="")

            print(f"\n  完成！保存到 {output_path}")

        except Exception as e:
            print(f"  下载失败: {e}")

    return output_dir


def analyze_jsonl(filepath):
    """分析 JSONL 文件内容"""
    print(f"\n分析 {filepath}...")

    with open(filepath, 'r') as f:
        first_line = f.readline()
        data = json.loads(first_line)

    print("字段:")
    for key, value in data.items():
        if isinstance(value, list):
            print(f"  {key}: list (len={len(value)})")
            if len(value) > 0 and len(value) <= 5:
                print(f"    内容: {value}")
            elif len(value) > 5:
                print(f"    示例: {value[:5]}...")
        else:
            print(f"  {key}: {type(value).__name__} = {value}")

    # 统计文件行数
    with open(filepath, 'r') as f:
        line_count = sum(1 for _ in f)
    print(f"  总行数: {line_count}")

    return data


def convert_to_training_format(scene_graph_data):
    """
    将场景图数据转换为我们的训练格式

    输入格式 (openvocab-scene-graphs):
    - image_id: 图像ID
    - obj_boxes: [[x1,y1,x2,y2], ...] 边界框
    - obj_names: ["person", "chair", ...] 对象名称
    - pairs: [[subj_idx, obj_idx], ...] 关系对
    - rel_preds: ["sitting on", ...] 关系谓词
    - rel_is_pos: [True, ...] 是否为正例

    输出格式 (我们的训练格式):
    - text: 自然语言描述
    - entities: 实体列表
    - relations: 关系列表
    - layout_3d: 3D布局
    """
    training_samples = []

    for item in scene_graph_data:
        obj_names = item.get('obj_names', [])
        obj_boxes = item.get('obj_boxes', [])
        pairs = item.get('pairs', [])
        rel_preds = item.get('rel_preds', [])

        if len(obj_names) == 0:
            continue

        # 创建实体列表
        entities = []
        for i, name in enumerate(obj_names[:20]):  # 限制最多20个实体
            entity = {
                "id": f"obj_{i}",
                "name": name,
                "type": infer_entity_type(name),
                "attributes": {}
            }
            entities.append(entity)

        # 创建关系列表
        relations = []
        for (subj_idx, obj_idx), pred in zip(pairs, rel_preds):
            if subj_idx < len(entities) and obj_idx < len(entities):
                relation = {
                    "from": f"obj_{subj_idx}",
                    "to": f"obj_{obj_idx}",
                    "type": normalize_relation(pred)
                }
                relations.append(relation)

        # 生成文本描述
        text = generate_text_description(entities, relations)

        # 创建3D布局（从边界框推断）
        layout_3d = create_layout_from_boxes(obj_boxes[:20], obj_names[:20])

        sample = {
            "id": item.get('image_id', 'unknown'),
            "text": text,
            "language": "en",
            "entities": entities,
            "relations": relations[:30],  # 限制关系数量
            "layout_3d": layout_3d
        }

        training_samples.append(sample)

    return training_samples


def infer_entity_type(name):
    """从名称推断实体类型"""
    name_lower = name.lower()

    type_keywords = {
        "person": ["person", "man", "woman", "child", "people", "boy", "girl"],
        "animal": ["dog", "cat", "bird", "horse", "cow", "sheep"],
        "furniture": ["chair", "table", "sofa", "bed", "desk", "couch", "bench"],
        "building": ["house", "building", "room", "door", "window", "wall"],
        "vehicle": ["car", "truck", "bus", "bike", "motorcycle"],
        "plant": ["tree", "plant", "flower", "grass", "bush"],
        "object": ["book", "cup", "bottle", "phone", "lamp", "clock"]
    }

    for entity_type, keywords in type_keywords.items():
        if any(kw in name_lower for kw in keywords):
            return entity_type

    return "object"


def normalize_relation(pred):
    """规范化关系类型"""
    pred_lower = pred.lower()

    # 空间关系映射
    spatial_relations = {
        "on": "on",
        "in": "contains",
        "inside": "contains",
        "next to": "adjacent",
        "beside": "adjacent",
        "near": "adjacent",
        "under": "below",
        "below": "below",
        "above": "above",
        "behind": "behind",
        "in front of": "in_front",
        "wearing": "wears",
        "holding": "holds",
        "sitting on": "on",
        "standing on": "on",
        "lying on": "on"
    }

    for pattern, relation in spatial_relations.items():
        if pattern in pred_lower:
            return relation

    return "related"


def generate_text_description(entities, relations):
    """生成自然语言描述"""
    if len(entities) == 0:
        return "empty scene"

    # 简单描述：列出主要实体
    entity_names = [e["name"] for e in entities[:5]]

    if len(entities) <= 3:
        return "a scene with " + ", ".join(entity_names)
    else:
        return f"a scene with {', '.join(entity_names)} and more"


def create_layout_from_boxes(boxes, names):
    """从2D边界框创建3D布局"""
    nodes = []

    for i, (box, name) in enumerate(zip(boxes, names)):
        # 从边界框推断位置和大小
        x1, y1, x2, y2 = box

        # 中心位置
        cx = (x1 + x2) / 2
        cy = (y1 + y2) / 2

        # 大小
        width = x2 - x1
        height = y2 - y1

        # 归一化到合理范围
        scale_x = width / 100
        scale_y = height / 100
        scale_z = min(scale_x, scale_y)  # 深度估计

        node = {
            "entity_id": f"obj_{i}",
            "position": [cx / 10, 0.0, cy / 10],  # 简单映射到3D
            "rotation": [0.0, 0.0, 0.0, 1.0],
            "scale": [scale_x, scale_y, scale_z],
            "geometry": infer_geometry(name)
        }
        nodes.append(node)

    return {
        "root": "obj_0" if nodes else "",
        "nodes": nodes
    }


def infer_geometry(name):
    """推断几何类型"""
    name_lower = name.lower()

    if any(kw in name_lower for kw in ["person", "man", "woman", "child"]):
        return "capsule"
    elif any(kw in name_lower for kw in ["tree", "plant", "flower"]):
        return "cylinder"
    elif any(kw in name_lower for kw in ["house", "building", "room"]):
        return "box"
    else:
        return "box"


def main():
    print("=== 场景图训练数据下载 ===\n")

    # 方法1: 使用 huggingface datasets 库
    dataset = download_via_huggingface()

    if dataset is None:
        print("\n尝试直接下载 JSONL 文件...")
        output_dir = download_via_requests()

        # 分析下载的文件
        for jsonl_file in output_dir.glob("*.jsonl"):
            if jsonl_file.stat().st_size > 0:
                analyze_jsonl(jsonl_file)

    print("\n完成！")


if __name__ == "__main__":
    main()
