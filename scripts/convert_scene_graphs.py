"""
将场景图数据转换为训练格式

从 HuggingFace 的 openvocab-scene-graphs 数据集
转换为我们的 text→3D 训练格式。
"""

import json
import random
from pathlib import Path
from collections import Counter


def load_scene_graph_dataset():
    """加载已下载的场景图数据集"""
    from datasets import load_dataset

    print("加载场景图数据集...")
    dataset = load_dataset("anhkhoa1804/openvocab-scene-graphs", split="train")
    print(f"共 {len(dataset)} 条数据")
    return dataset


def infer_entity_type(name):
    """从名称推断实体类型"""
    name_lower = name.lower()

    type_keywords = {
        "person": ["person", "man", "woman", "child", "people", "boy", "girl", "kid", "baby", "lady", "gentleman"],
        "animal": ["dog", "cat", "bird", "horse", "cow", "sheep", "fish", "elephant", "lion", "tiger", "bear", "deer", "rabbit", "mouse", "snake", "frog", "duck", "chicken", "pig", "goat"],
        "furniture": ["chair", "table", "sofa", "bed", "desk", "couch", "bench", "stool", "shelf", "cabinet", "dresser", "wardrobe", "bookshelf", "nightstand", "ottoman", "recliner"],
        "building": ["house", "building", "room", "door", "window", "wall", "floor", "ceiling", "roof", "stairs", "balcony", "garage", "kitchen", "bathroom", "bedroom", "living"],
        "vehicle": ["car", "truck", "bus", "bike", "motorcycle", "bicycle", "train", "plane", "airplane", "boat", "ship", "van", "scooter", "taxi"],
        "plant": ["tree", "plant", "flower", "grass", "bush", "leaf", "branch", "trunk", "vine", "palm", "rose", "tulip"],
        "food": ["food", "fruit", "vegetable", "meat", "bread", "rice", "pasta", "soup", "salad", "sandwich", "pizza", "burger", "apple", "banana", "orange", "grape", "tomato", "potato", "carrot"],
        "object": ["book", "cup", "bottle", "phone", "lamp", "clock", "vase", "picture", "painting", "mirror", "curtain", "rug", "carpet", "pillow", "blanket", "towel", "box", "bag", "hat", "shoe", "shirt", "pants", "coat", "jacket", "glasses", "watch", "ring", "necklace", "toy", "ball", "bat", "racket", "instrument", "guitar", "piano", "violin", "drum", "screen", "monitor", "keyboard", "mouse", "laptop", "tablet", "camera", "remote", "battery", "cable", "wire", "plug", "switch", "knob", "handle", "lever", "button", "dial", "gauge", "meter", "scale", "thermometer", "compass", "map", "chart", "graph", "diagram", "sign", "label", "tag", "sticker", "stamp", "seal", "logo", "symbol", "icon", "flag", "banner", "poster", "card", "paper", "document", "file", "folder", "envelope", "letter", "note", "memo", "report", "article", "story", "poem", "song", "lyrics", "script", "code", "program", "algorithm", "formula", "equation", "expression", "function", "variable", "constant", "parameter", "argument", "result", "output", "input", "value", "data", "info", "message", "signal", "wave", "pulse", "tone", "sound", "noise", "voice", "speech", "word", "phrase", "sentence", "paragraph", "chapter", "section", "part", "piece", "fragment", "chunk", "block", "segment", "element", "component", "module", "unit", "item", "object", "entity", "thing", "stuff", "material", "substance", "matter", "content", "filling", "core", "center", "middle", "edge", "side", "corner", "tip", "end", "base", "top", "bottom", "front", "back", "left", "right", "north", "south", "east", "west", "up", "down", "in", "out", "over", "under", "above", "below", "between", "among", "through", "across", "along", "around", "behind", "before", "after", "next", "previous", "first", "last", "start", "finish", "begin", "stop", "pause", "resume", "continue", "repeat", "loop", "cycle", "iteration", "step", "stage", "phase", "level", "layer", "tier", "rank", "grade", "score", "point", "mark", "line", "curve", "shape", "form", "pattern", "texture", "color", "shade", "tone", "hue", "saturation", "brightness", "contrast", "opacity", "transparency", "visibility", "clarity", "resolution", "quality", "size", "dimension", "length", "width", "height", "depth", "thickness", "weight", "mass", "volume", "area", "surface", "perimeter", "radius", "diameter", "circumference", "angle", "slope", "gradient", "direction", "orientation", "rotation", "translation", "scale", "transform", "matrix", "vector", "tensor", "array", "list", "set", "collection", "group", "cluster", "batch", "bundle", "pack", "package", "parcel", "crate", "container", "vessel", "tank", "reservoir", "pool", "pond", "lake", "river", "stream", "ocean", "sea", "bay", "gulf", "strait", "channel", "canal", "ditch", "trench", "hole", "pit", "cavity", "gap", "space", "void", "vacuum", "air", "gas", "liquid", "fluid", "solid", "crystal", "glass", "metal", "plastic", "rubber", "wood", "stone", "rock", "sand", "dust", "dirt", "soil", "mud", "clay", "cement", "concrete", "brick", "tile", "slate", "marble", "granite", "limestone", "sandstone", "quartz", "diamond", "gold", "silver", "bronze", "copper", "iron", "steel", "aluminum", "titanium", "lead", "tin", "zinc", "nickel", "chromium", "manganese", "cobalt", "platinum", "palladium", "iridium", "osmium", "ruthenium", "rhodium", "tungsten", "molybdenum", "vanadium", "niobium", "tantalum", "hafnium", "zirconium", "yttrium", "scandium", "lanthanum", "cerium", "praseodymium", "neodymium", "promethium", "samarium", "europium", "gadolinium", "terbium", "dysprosium", "holmium", "erbium", "thulium", "ytterbium", "lutetium", "actinium", "thorium", "protactinium", "uranium", "neptunium", "plutonium", "americium", "curium", "berkelium", "californium", "einsteinium", "fermium", "mendelevium", "nobelium", "lawrencium", "rutherfordium", "dubnium", "seaborgium", "bohrium", "hassium", "meitnerium", "darmstadtium", "roentgenium", "copernicium", "nihonium", "flerovium", "moscovium", "livermorium", "tennessine", "oganesson"]
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
        "on": ["on", "on top of", "upon", "over"],
        "contains": ["in", "inside", "within", "contained in", "contained by"],
        "adjacent": ["next to", "beside", "near", "adjacent to", "by", "alongside", "next"],
        "below": ["under", "below", "beneath", "underneath"],
        "above": ["above", "over", "higher than"],
        "behind": ["behind", "in back of", "back of"],
        "in_front": ["in front of", "before", "facing"],
        "wears": ["wearing", "wears", "worn by", "dressed in"],
        "holds": ["holding", "holds", "carrying", "carries"],
        "part_of": ["part of", "part", "component of", "belongs to"],
        "attached": ["attached to", "connected to", "linked to", "mounted on"],
        "covers": ["covers", "covering", "covered by"],
        "supports": ["supports", "supporting", "supported by"]
    }

    for relation, patterns in spatial_relations.items():
        if any(pattern in pred_lower for pattern in patterns):
            return relation

    # 检查方向性关系
    if "left" in pred_lower:
        return "left_of"
    if "right" in pred_lower:
        return "right_of"

    return "related"


def infer_geometry(entity_type):
    """根据实体类型推断几何形状"""
    geometry_map = {
        "person": "capsule",
        "animal": "capsule",
        "plant": "cylinder",
        "building": "box",
        "furniture": "box",
        "vehicle": "box",
        "food": "sphere",
        "object": "box"
    }
    return geometry_map.get(entity_type, "box")


def generate_text_description(entities, relations):
    """生成自然语言描述"""
    if len(entities) == 0:
        return "empty scene"

    # 获取实体名称
    entity_names = [e["name"] for e in entities[:5]]

    # 构建描述
    if len(entities) == 1:
        return f"a scene with a {entity_names[0]}"
    elif len(entities) <= 3:
        return f"a scene with {', '.join(entity_names)}"
    else:
        return f"a scene with {', '.join(entity_names)} and {len(entities) - 5} more objects"


def create_layout_from_boxes(boxes, names, entity_types):
    """从2D边界框创建3D布局"""
    nodes = []

    # 归一化坐标范围
    min_x, min_y = float('inf'), float('inf')
    max_x, max_y = float('-inf'), float('-inf')

    for box in boxes:
        x1, y1, x2, y2 = box
        min_x = min(min_x, x1)
        min_y = min(min_y, y1)
        max_x = max(max_x, x2)
        max_y = max(max_y, y2)

    range_x = max_x - min_x if max_x > min_x else 1
    range_y = max_y - min_y if max_y > min_y else 1

    for i, (box, name, entity_type) in enumerate(zip(boxes, names, entity_types)):
        x1, y1, x2, y2 = box

        # 归一化位置到 [-5, 5] 范围
        cx = ((x1 + x2) / 2 - min_x) / range_x * 10 - 5
        cy = ((y1 + y2) / 2 - min_y) / range_y * 10 - 5

        # 大小
        width = (x2 - x1) / range_x * 2
        height = (y2 - y1) / range_y * 2

        # 根据实体类型调整缩放
        type_scales = {
            "person": [0.5, 1.8, 0.3],
            "animal": [0.4, 0.4, 0.6],
            "plant": [0.3, 1.5, 0.3],
            "building": [3.0, 2.0, 3.0],
            "furniture": [0.8, 0.6, 0.8],
            "vehicle": [1.5, 0.6, 0.8],
            "food": [0.2, 0.1, 0.2],
            "object": [0.3, 0.3, 0.3]
        }

        default_scale = type_scales.get(entity_type, [0.5, 0.5, 0.5])

        node = {
            "entity_id": f"obj_{i}",
            "position": [cx, 0.0, cy],
            "rotation": [0.0, 0.0, 0.0, 1.0],
            "scale": default_scale,
            "geometry": infer_geometry(entity_type)
        }
        nodes.append(node)

    return {
        "root": "obj_0" if nodes else "",
        "nodes": nodes
    }


def convert_sample(item, sample_id):
    """转换单个样本"""
    obj_names = item.get('obj_names', [])
    obj_boxes = item.get('obj_boxes', [])
    pairs = item.get('pairs', [])
    rel_preds = item.get('rel_preds', [])
    rel_is_pos = item.get('rel_is_pos', [])

    if len(obj_names) == 0:
        return None

    # 过滤掉 Freebase ID 名称（以 /m/ 开头的）
    # 保留有可读名称的样本
    readable_names = []
    readable_boxes = []
    readable_indices = []

    for i, name in enumerate(obj_names):
        if name and not name.startswith('/m/') and not name.startswith('/'):
            readable_names.append(name)
            readable_boxes.append(obj_boxes[i])
            readable_indices.append(i)

    # 如果没有可读名称，跳过此样本
    if len(readable_names) == 0:
        return None

    # 限制实体数量
    max_entities = min(20, len(readable_names))
    readable_names = readable_names[:max_entities]
    readable_boxes = readable_boxes[:max_entities]
    readable_indices = readable_indices[:max_entities]

    # 推断实体类型
    entity_types = [infer_entity_type(name) for name in readable_names]

    # 创建实体列表
    entities = []
    for i, (name, etype) in enumerate(zip(readable_names, entity_types)):
        entity = {
            "id": f"obj_{i}",
            "name": name,
            "type": etype,
            "attributes": {}
        }
        entities.append(entity)

    # 创建关系列表（需要映射原始索引到新索引）
    old_to_new = {old_idx: new_idx for new_idx, old_idx in enumerate(readable_indices)}
    relations = []

    for (subj_idx, obj_idx), pred, is_pos in zip(pairs, rel_preds, rel_is_pos):
        # 只处理正例和可读实体之间的关系
        if is_pos and subj_idx in old_to_new and obj_idx in old_to_new:
            relation = {
                "from": f"obj_{old_to_new[subj_idx]}",
                "to": f"obj_{old_to_new[obj_idx]}",
                "type": normalize_relation(pred)
            }
            relations.append(relation)

    # 限制关系数量
    relations = relations[:30]

    # 生成文本描述
    text = generate_text_description(entities, relations)

    # 创建3D布局
    layout_3d = create_layout_from_boxes(readable_boxes, readable_names, entity_types)

    sample = {
        "id": f"scene_{sample_id:06d}",
        "text": text,
        "language": "en",
        "entities": entities,
        "relations": relations,
        "layout_3d": layout_3d
    }

    return sample


def main():
    print("=== 场景图数据转换 ===\n")

    # 加载数据集
    dataset = load_scene_graph_dataset()

    # 统计信息
    entity_type_counter = Counter()
    relation_type_counter = Counter()

    # 转换数据
    print("\n转换数据...")
    converted_samples = []

    # 采样部分数据（太多会占用大量内存）
    sample_size = min(10000, len(dataset))  # 最多转换10000条
    indices = random.sample(range(len(dataset)), sample_size)

    for i, idx in enumerate(indices):
        item = dataset[idx]
        sample = convert_sample(item, i)

        if sample:
            converted_samples.append(sample)

            # 统计
            for entity in sample["entities"]:
                entity_type_counter[entity["type"]] += 1
            for relation in sample["relations"]:
                relation_type_counter[relation["type"]] += 1

        if (i + 1) % 1000 == 0:
            print(f"  已转换 {i + 1}/{sample_size} 条")

    print(f"\n转换完成！共 {len(converted_samples)} 条有效样本")

    # 打印统计信息
    print("\n实体类型统计:")
    for etype, count in entity_type_counter.most_common(10):
        print(f"  {etype}: {count}")

    print("\n关系类型统计:")
    for rtype, count in relation_type_counter.most_common(10):
        print(f"  {rtype}: {count}")

    # 保存为训练格式
    output_path = Path("data/training/scene_graphs_converted.json")
    output_path.parent.mkdir(parents=True, exist_ok=True)

    training_data = {
        "version": "1.0",
        "description": "Converted from openvocab-scene-graphs dataset",
        "data": converted_samples
    }

    with open(output_path, 'w', encoding='utf-8') as f:
        json.dump(training_data, f, indent=2, ensure_ascii=False)

    print(f"\n已保存到 {output_path}")
    print(f"文件大小: {output_path.stat().st_size / 1024 / 1024:.2f} MB")

    # 打印示例
    print("\n示例数据:")
    for sample in converted_samples[:3]:
        print(f"\n  文本: {sample['text']}")
        print(f"  实体数: {len(sample['entities'])}")
        print(f"  关系数: {len(sample['relations'])}")


if __name__ == "__main__":
    main()
