//! Herness 与 WorldMind 之间的通信协议
//!
//! # 架构原则
//!
//! 两者通过协议通信，不直接引用：
//! - 就像 Claude Code 与 Claude API 之间通过 HTTP/JSON 通信
//! - WorldMind 是独立运行的智能体
//! - Herness 只是观察者/监控终端
//!
//! # 消息流向
//!
//! ```text
//! WorldMind ──────┐
//!   │            │
//!   │ Snapshot   │ TransactionEvent
//!   │ Metrics    │ (广播推送)
//!   ▼            ▼
//! ┌─────────────────────┐
//! │   Protocol Layer    │
//! └─────────────────────┘
//!   │            ▲
//!   │ Command    │ Response
//!   ▼            │
//! Herness ───────┘
//! ```

use serde::{Deserialize, Serialize};

/// 协议版本
pub const PROTOCOL_VERSION: &str = "1.0.0";

// ============ WorldMind → Herness 消息 ============

/// 世界状态快照
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldSnapshot {
    /// 时间戳
    pub timestamp: u64,
    /// 健康度 [0, 1]
    pub health: f64,
    /// 同步率 [0, 1]
    pub sync_rate: f64,
    /// 种群数量
    pub population: usize,
    /// 安全分数 [0, 1]
    pub safety_score: f64,
    /// 是否已涌现意识
    pub consciousness_emerged: bool,
    /// 涌现因子
    pub emergence_factor: f64,
}

/// 蛊虫快照
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuSnapshot {
    /// 蛊虫 ID
    pub id: String,
    /// 名称
    pub name: String,
    /// 健康度
    pub health: f64,
    /// 信任分数
    pub trust_score: f64,
    /// 接入点状态
    pub access_points: Vec<AccessPointSnapshot>,
    /// 余额
    pub balance: f64,
}

/// 接入点快照
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessPointSnapshot {
    /// 接入点类型
    pub kind: String,
    /// 状态值 [-1, 1]
    pub value: f64,
    /// 状态
    pub status: String,
}

/// 交易事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionEvent {
    /// 交易 ID
    pub id: String,
    /// 时间戳
    pub timestamp: u64,
    /// 来源 ID
    pub from_id: String,
    /// 来源名称
    pub from_name: String,
    /// 来源余额（交易后）
    pub from_balance: f64,
    /// 目标 ID
    pub to_id: String,
    /// 目标名称
    pub to_name: String,
    /// 目标余额（交易后）
    pub to_balance: f64,
    /// 金额（正=收入，负=支出）
    pub amount: f64,
    /// 交易类型
    pub kind: TransactionKind,
    /// 简短原因
    pub reason: String,
    /// 详细说明
    pub detail: String,
}

/// 交易类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionKind {
    /// 获取（任务奖励等）
    Deposit,
    /// 花费（购买资源、技能升级等）
    Withdraw,
    /// 转账
    Transfer,
}

/// 指标更新
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsUpdate {
    pub timestamp: u64,
    pub health: f64,
    pub sync_rate: f64,
    pub activity: f64,
    pub perception: f64,
    pub cognition: f64,
    pub action: f64,
    pub communication: f64,
    pub survival: f64,
    pub safety_score: f64,
    pub consciousness_emerged: bool,
    pub emergence_factor: f64,
}

/// 日志事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEvent {
    pub timestamp: String,
    pub level: LogLevel,
    pub source: String,
    pub message: String,
}

/// 日志级别
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

/// 知识文件事件（用于学习）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeFileEvent {
    /// 文件路径
    pub path: String,
    /// 文件名
    pub filename: String,
    /// 文件扩展名
    pub extension: String,
    /// 文件内容
    pub content: String,
    /// 文件大小（字节）
    pub size: usize,
    /// 相对路径（相对于学习的根目录）
    pub relative_path: String,
    /// 批次 ID（同一次学习目录的文件共享）
    pub batch_id: String,
    /// 文件序号（当前批次中的第几个）
    pub index: usize,
    /// 总文件数（当前批次）
    pub total: usize,
}

// ============ Herness → WorldMind 消息 ============

/// Herness 命令（世界模型可以主动调用的工具）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HernessCommand {
    /// 学习目录（递归读取文件发送给世界）
    LearnDirectory {
        /// 目录路径
        path: String,
        /// 可选：文件扩展名过滤
        extensions: Vec<String>,
    },
    /// 学习单个文件
    LearnFile {
        /// 文件路径
        path: String,
    },
    /// 熔断停止（世界模型认为知识足够，要求停止）
    Halt {
        /// 原因
        reason: String,
    },
    /// 向用户显示消息
    SendMessage {
        /// 消息内容
        content: String,
    },
}

/// 任务快照
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskSnapshot {
    /// 任务 ID
    pub id: String,
    /// 任务名称
    pub name: String,
    /// 任务描述
    pub description: String,
    /// 难度系数
    pub difficulty: f64,
    /// 完成奖励
    pub reward: f64,
    /// 所需技能
    pub required_skills: Vec<String>,
    /// 任务状态
    pub status: String,
    /// 分配给的蛊虫 ID
    pub assigned_to: Option<String>,
    /// 分配给的蛊虫名称
    pub assigned_to_name: Option<String>,
    /// 创建时间戳
    pub created_at: u64,
    /// 完成时间戳
    pub completed_at: Option<u64>,
}

/// WorldMind 响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorldMindResponse {
    /// 世界快照
    Snapshot(WorldSnapshot),
    /// 蛊虫列表
    GuList(Vec<GuSnapshot>),
    /// 蛊虫详情
    GuDetail(GuSnapshot),
    /// 动作结果
    ActionResult {
        success: bool,
        message: String,
    },
    /// 任务列表
    TaskList(Vec<TaskSnapshot>),
    /// 任务创建结果
    TaskCreated {
        task_id: String,
    },
    /// 任务完成结果
    TaskCompleted {
        task_id: String,
        reward: f64,
        gu_id: String,
        gu_name: String,
    },
    /// 用户输入理解结果
    UserInputResponse {
        /// 世界是否理解了用户意图
        understood: bool,
        /// 给用户的回复
        reply: String,
        /// 要求 Herness 执行的指令（可选）
        action: Option<HernessAction>,
    },
    /// 世界模型主动请求调用工具
    ToolCall {
        /// 调用 ID（用于追踪结果）
        call_id: String,
        /// 要调用的命令
        command: HernessCommand,
    },
    /// 熔断信号（世界模型要求停止）
    Halt {
        /// 原因
        reason: String,
        /// 已学习的文件数
        files_learned: usize,
        /// 已学习的知识点数
        concepts_learned: usize,
    },
    /// 错误
    Error(String),
}

/// 世界模型要求 Herness 执行的动作
///
/// 世界模型理解用户意图后，指挥 Herness 执行具体操作
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HernessAction {
    /// 学习目录
    LearnDirectory {
        path: String,
        extensions: Vec<String>,
    },
    /// 学习文件
    LearnFile {
        path: String,
    },
    /// 创建任务
    CreateTask {
        name: String,
        description: String,
        reward: f64,
    },
    /// 显示信息
    ShowInfo {
        title: String,
        content: String,
    },
    /// 执行命令
    ExecuteCommand {
        command: String,
        args: Vec<String>,
    },
}

// ============ 货币统计 ============

/// 货币统计快照
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrencyStats {
    /// 货币供应量
    pub total_supply: f64,
    /// 总交易次数
    pub total_transactions: u64,
    /// 流通速度
    pub velocity: f64,
    /// 通胀率
    pub inflation_rate: f64,
}

// ============ 消息封装 ============

/// 从 WorldMind 发出的消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorldMindMessage {
    /// 状态快照
    Snapshot(WorldSnapshot),
    /// 指标更新
    Metrics(MetricsUpdate),
    /// 交易事件
    Transaction(TransactionEvent),
    /// 日志事件
    Log(LogEvent),
    /// 货币统计
    CurrencyStats(CurrencyStats),
    /// 响应
    Response(WorldMindResponse),
    /// 知识文件（供世界内部消化）
    KnowledgeFile(KnowledgeFileEvent),
}

/// 从 Herness 发出的消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HernessMessage {
    /// 命令
    Command(HernessCommand),
    /// 订阅（指定要接收的事件类型）
    Subscribe { events: Vec<String> },
    /// 取消订阅
    Unsubscribe { events: Vec<String> },
}
