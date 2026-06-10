# 蛊虫技能网络存储设计

## 目录结构

```
D:\ai_006\knowledge\
├── concepts/                 # 概念知识库（已存在）
│   ├── base/                 # 基础概念
│   └── domain/               # 领域概念
├── skills/                   # 技能知识库（新增）
│   ├── shared/               # 共享技能（高信任度）
│   │   ├── html-basics.html
│   │   └── css-layout.html
│   └── private/              # 私有技能（隔离）
│       ├── gu_a1b2c3/        # 蛊虫 A 的私有技能
│       │   └── experiments.html
│       └── gu_d4e5f6/        # 蛊虫 B 的私有技能
│           └── notes.html
├── gus/                      # 蛊虫索引（新增）
│   ├── gu_a1b2c3.html        # 蛊虫 A 的技能索引
│   └── gu_d4e5f6.html        # 蛊虫 B 的技能索引
├── flows/                    # 流程知识（已存在）
└── meta/                     # 元索引（已存在）
    └── concept_index.html
```

---

## HTML 格式规范

### 1. 蛊虫索引文件 (gus/gu_xxx.html)

```html
<!DOCTYPE html>
<html lang="zh-CN" data-type="gu-index">
<head>
    <meta charset="UTF-8">
    <meta name="gu-id" content="gu_a1b2c3d4">
    <meta name="gu-name" content="朱红猎手">
    <meta name="created-at" content="2026-05-31T10:00:00Z">
    <meta name="trust-score" content="0.85">
    <meta name="skill-count" content="3">
    <title>朱红猎手 - 蛊虫技能索引</title>
</head>
<body>
    <article class="gu-profile">
        <header>
            <h1>🔴 朱红猎手</h1>
            <div class="gu-meta">
                <span class="gu-id">gu_a1b2c3d4</span>
                <span class="trust">信任度: 85%</span>
            </div>
        </header>

        <!-- 技能列表（引用） -->
        <section class="skills">
            <h2>已掌握技能</h2>
            <ul class="skill-list">
                <li>
                    <a href="../skills/shared/html-basics.html" rel="skill" data-mastery="0.8">
                        HTML 基础
                        <span class="mastery">掌握度: 80%</span>
                    </a>
                </li>
                <li>
                    <a href="../skills/shared/css-layout.html" rel="skill" data-mastery="0.6">
                        CSS 布局
                        <span class="mastery">掌握度: 60%</span>
                    </a>
                </li>
                <li>
                    <a href="../skills/private/gu_a1b2c3d4/experiments.html" rel="skill" data-mastery="0.3">
                        实验笔记
                        <span class="mastery private">私有技能</span>
                    </a>
                </li>
            </ul>
        </section>

        <!-- 知识图谱连接 -->
        <section class="knowledge-graph">
            <h2>知识连接</h2>
            <ul class="relation-list">
                <li>▸ 学习了 <a href="../concepts/domain/web/html.html">HTML</a></li>
                <li>▸ 学习了 <a href="../concepts/domain/web/css.html">CSS</a></li>
            </ul>
        </section>

        <!-- LNN 状态快照（机器可读） -->
        <section class="lnn-state" hidden>
            <data name="perception" value="0.65">
            <data name="cognitive" value="0.72">
            <data name="behavior" value="0.58">
            <data name="comm" value="0.45">
            <data name="survival" value="0.88">
        </section>
    </article>
</body>
</html>
```

### 2. 技能文件 (skills/shared/html-basics.html)

```html
<!DOCTYPE html>
<html lang="zh-CN" data-type="skill">
<head>
    <meta charset="UTF-8">
    <meta name="skill-id" content="skill_html_basics">
    <meta name="skill-type" content="shared">
    <meta name="created-by" content="gu_a1b2c3d4">
    <meta name="created-at" content="2026-05-31T10:00:00Z">
    <meta name="consensus-status" content="approved">
    <meta name="ref-count" content="12">
    <title>HTML 基础 - 技能</title>
</head>
<body>
    <article class="skill">
        <header>
            <h1>🌐 HTML 基础</h1>
            <div class="skill-meta">
                <span class="skill-type shared">共享技能</span>
                <span class="ref-count">被引用: 12次</span>
            </div>
        </header>

        <!-- 技能定义 -->
        <section class="definition">
            <h2>定义</h2>
            <p>HTML（HyperText Markup Language）是构建网页的标准标记语言。
            由标签、属性、元素构成文档结构。</p>
        </section>

        <!-- 核心概念（链接到概念库） -->
        <section class="concepts">
            <h2>核心概念</h2>
            <ul class="concept-list">
                <li><a href="../../concepts/domain/web/html/element.html" rel="concept">元素</a></li>
                <li><a href="../../concepts/domain/web/html/attribute.html" rel="concept">属性</a></li>
                <li><a href="../../concepts/domain/web/html/tag.html" rel="concept">标签</a></li>
            </ul>
        </section>

        <!-- 知识粒子（从 CognisParser 提取） -->
        <section class="particles" hidden>
            <data name="entities" value='["HTML", "element", "tag", "attribute", "div", "span"]'>
            <data name="code-languages" value='["html"]'>
            <data name="keywords" value='["markup", "web", "structure"]'>
            <data name="main-topic" value="HTML">
        </section>

        <!-- 相关技能 -->
        <section class="relations">
            <h2>相关技能</h2>
            <ul class="relation-list">
                <li>▸ <a href="./css-basics.html" rel="prerequisite">前置: CSS 基础</a></li>
                <li>▸ <a href="./javascript-basics.html" rel="next-step">进阶: JavaScript</a></li>
            </ul>
        </section>

        <!-- 学习者索引（谁学了这个技能） -->
        <section class="learners">
            <h2>学习者</h2>
            <ul class="learner-list">
                <li><a href="../../gus/gu_a1b2c3d4.html">朱红猎手</a> - 掌握度 80%</li>
                <li><a href="../../gus/gu_d4e5f6g7.html">碧绿探索者</a> - 掌握度 60%</li>
            </ul>
        </section>
    </article>
</body>
</html>
```

---

## 引用关系图

```
┌─────────────────────────────────────────────────────────────────┐
│                         知识网络引用图                           │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  gus/gu_a1b2.html ─────┬───────────────────────────────────────│
│       │                 │                                       │
│       │ rel="skill"     │ rel="skill"                           │
│       ▼                 ▼                                       │
│  skills/shared/     skills/private/                             │
│  html-basics.html   gu_a1b2/experiments.html                    │
│       │                 ▲                                       │
│       │ rel="concept"   │ 学习记录                               │
│       ▼                 │                                       │
│  concepts/domain/   ────┘                                       │
│  web/html/element.html                                          │
│                                                                  │
│  ════════════════════════════════════════════════════════════   │
│                                                                  │
│  引用类型:                                                       │
│  • rel="skill"      蛊虫掌握的技能                               │
│  • rel="concept"    技能涉及的概念                               │
│  • rel="prerequisite" 前置技能                                   │
│  • rel="next-step"  进阶技能                                     │
│  • rel="learned"    学习记录                                     │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

## 存储策略

### 共享 vs 私有

| 条件 | 存储位置 | 可见性 |
|------|----------|--------|
| 信任度 > 0.8 且共识通过 | `skills/shared/` | 全局可见 |
| 信任度 ≤ 0.8 或未验证 | `skills/private/gu_xxx/` | 仅创建者可见 |
| 实验性/草稿 | `skills/private/gu_xxx/` | 仅创建者可见 |

### 引用计数

每次蛊虫学习一个技能时：
1. 增加技能文件的 `ref-count`
2. 更新学习者的 `mastery` 值
3. 触发共识验证流程

---

## 实现优先级

1. **Phase 1**: 创建 `gus/` 目录和蛊虫索引文件格式
2. **Phase 2**: 创建 `skills/` 目录和技能文件格式
3. **Phase 3**: 实现 `KnowledgeStorage` 模块，读写 HTML 文件
4. **Phase 4**: 集成到 `receive_knowledge_file()` 流程

---

*设计版本: v1.0*
*设计日期: 2026-05-31*
