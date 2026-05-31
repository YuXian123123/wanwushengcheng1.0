//! 审计追踪 - 螺丝咕姆第五层防护

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 审计条目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    /// 条目ID
    pub id: u64,
    /// 用户ID
    pub user: Uuid,
    /// 操作类型
    pub action: String,
    /// 内容哈希
    pub content_hash: String,
    /// 时间戳
    pub timestamp: u64,
    /// 元数据
    pub metadata: serde_json::Value,
}

impl AuditEntry {
    pub fn new(user: Uuid, action: &str, content_hash: &str) -> Self {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Self {
            id: 0, // 将在添加时分配
            user,
            action: action.to_string(),
            content_hash: content_hash.to_string(),
            timestamp,
            metadata: serde_json::json!({}),
        }
    }

    pub fn with_metadata(mut self, metadata: serde_json::Value) -> Self {
        self.metadata = metadata;
        self
    }
}

/// 审计日志
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLog {
    /// 条目列表
    entries: Vec<AuditEntry>,
    /// 下一个ID
    next_id: u64,
    /// 最大条目数
    max_entries: usize,
}

impl AuditLog {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            next_id: 1,
            max_entries: 10000,
        }
    }

    /// 添加审计条目（不可变）
    pub fn add(&self, mut entry: AuditEntry) -> Self {
        let mut new_log = self.clone();
        entry.id = self.next_id;
        new_log.next_id += 1;
        new_log.entries.push(entry);

        // 保留最近的条目
        if new_log.entries.len() > self.max_entries {
            new_log.entries.remove(0);
        }

        new_log
    }

    /// 查询用户的所有记录
    pub fn by_user(&self, user: &Uuid) -> Vec<&AuditEntry> {
        self.entries.iter().filter(|e| e.user == *user).collect()
    }

    /// 查询特定时间范围内的记录
    pub fn by_time_range(&self, start: u64, end: u64) -> Vec<&AuditEntry> {
        self.entries.iter()
            .filter(|e| e.timestamp >= start && e.timestamp <= end)
            .collect()
    }

    /// 查询特定操作的记录
    pub fn by_action(&self, action: &str) -> Vec<&AuditEntry> {
        self.entries.iter()
            .filter(|e| e.action == action)
            .collect()
    }

    /// 统计用户操作次数
    pub fn count_user_actions(&self, user: &Uuid) -> u64 {
        self.by_user(user).len() as u64
    }

    /// 获取用户最近的操作
    pub fn recent_user_actions(&self, user: &Uuid, limit: usize) -> Vec<&AuditEntry> {
        let mut user_entries: Vec<_> = self.by_user(user);
        user_entries.sort_by_key(|e| std::cmp::Reverse(e.timestamp));
        user_entries.into_iter().take(limit).collect()
    }

    /// 导出审计日志
    pub fn export(&self) -> String {
        serde_json::to_string_pretty(&self.entries).unwrap_or_default()
    }

    /// 获取总条目数
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// 是否为空
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

impl Default for AuditLog {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audit_entry_creation() {
        let user = Uuid::new_v4();
        let entry = AuditEntry::new(user, "send_message", "abc123");

        assert_eq!(entry.action, "send_message");
        assert_eq!(entry.content_hash, "abc123");
    }

    #[test]
    fn test_audit_log_add() {
        let log = AuditLog::new();
        let user = Uuid::new_v4();
        let entry = AuditEntry::new(user, "send_message", "abc123");

        let new_log = log.add(entry);
        assert_eq!(new_log.len(), 1);
    }

    #[test]
    fn test_audit_log_query() {
        let log = AuditLog::new();
        let user1 = Uuid::new_v4();
        let user2 = Uuid::new_v4();

        let log = log.add(AuditEntry::new(user1, "send_message", "hash1"));
        let log = log.add(AuditEntry::new(user1, "send_message", "hash2"));
        let log = log.add(AuditEntry::new(user2, "send_message", "hash3"));

        assert_eq!(log.by_user(&user1).len(), 2);
        assert_eq!(log.by_user(&user2).len(), 1);
        assert_eq!(log.count_user_actions(&user1), 2);
    }

    #[test]
    fn test_audit_log_export() {
        let log = AuditLog::new();
        let user = Uuid::new_v4();
        let entry = AuditEntry::new(user, "test", "hash");

        let log = log.add(entry);
        let exported = log.export();

        assert!(exported.contains("test"));
    }
}