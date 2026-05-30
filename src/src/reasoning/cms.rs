//! 认知市场经济系统 (CMS) - 创新设计
//!
//! 基于市场经济的认知资源分配系统

use std::collections::HashMap;

/// 认知币
pub type CognitiveCoins = f64;

/// 交易类型
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransactionType {
    /// 学习获得
    LearningReward,
    /// 推理消费
    InferenceCost,
    /// 正确推理奖励
    CorrectReward,
    /// 错误推理惩罚
    WrongPenalty,
    /// 知识购买
    KnowledgePurchase,
    /// 知识销售
    KnowledgeSale,
}

/// 交易记录
#[derive(Debug, Clone)]
pub struct Transaction {
    /// 交易ID
    pub id: String,
    /// 蛊虫ID
    pub gu_id: String,
    /// 交易类型
    pub tx_type: TransactionType,
    /// 金额
    pub amount: f64,
    /// 时间戳
    pub timestamp: u64,
    /// 描述
    pub description: String,
}

/// 知识商品
#[derive(Debug, Clone)]
pub struct KnowledgeProduct {
    /// 商品ID
    pub id: String,
    /// 知识内容
    pub content: String,
    /// 生产者
    pub producer: String,
    /// 基础价格
    pub base_price: f64,
    /// 当前价格（动态）
    pub current_price: f64,
    /// 需求量
    pub demand: u32,
    /// 供应量
    pub supply: u32,
}

/// 推理任务
#[derive(Debug, Clone)]
pub struct InferenceTask {
    /// 任务ID
    pub id: String,
    /// 难度系数
    pub difficulty: f64,
    /// 紧急程度
    pub urgency: f64,
    /// 基础价格
    pub base_price: f64,
    /// 发布者
    pub publisher: String,
    /// 竞标者
    pub bidders: Vec<String>,
    /// 中标者
    pub winner: Option<String>,
}

impl InferenceTask {
    /// 计算任务价格
    pub fn calculate_price(&self) -> f64 {
        self.base_price * self.difficulty * self.urgency
    }
}

/// 认知市场经济系统
pub struct CognitiveMarketSystem {
    /// 蛊虫余额
    balances: HashMap<String, CognitiveCoins>,
    /// 知识市场
    knowledge_market: HashMap<String, KnowledgeProduct>,
    /// 推理任务市场
    task_market: HashMap<String, InferenceTask>,
    /// 交易历史
    transactions: Vec<Transaction>,
    /// 市场配置
    config: MarketConfig,
}

/// 市场配置
#[derive(Debug, Clone)]
pub struct MarketConfig {
    /// 初始认知币
    pub initial_coins: f64,
    /// 学习奖励系数
    pub learning_reward: f64,
    /// 推理消费系数
    pub inference_cost: f64,
    /// 正确推理奖励系数
    pub correct_reward: f64,
    /// 错误推理惩罚系数
    pub wrong_penalty: f64,
    /// 价格调节系数
    pub price_adjustment: f64,
}

impl Default for MarketConfig {
    fn default() -> Self {
        Self {
            initial_coins: 100.0,
            learning_reward: 1.0,
            inference_cost: 1.0,
            correct_reward: 2.0,
            wrong_penalty: 0.5,
            price_adjustment: 0.1,
        }
    }
}

impl CognitiveMarketSystem {
    /// 创建新的认知市场
    pub fn new(config: MarketConfig) -> Self {
        Self {
            balances: HashMap::new(),
            knowledge_market: HashMap::new(),
            task_market: HashMap::new(),
            transactions: Vec::new(),
            config,
        }
    }

    /// 注册蛊虫
    pub fn register_gu(&mut self, gu_id: String) {
        self.balances.insert(gu_id, self.config.initial_coins);
    }

    /// 获取余额
    pub fn balance(&self, gu_id: &str) -> f64 {
        self.balances.get(gu_id).copied().unwrap_or(0.0)
    }

    /// 学习奖励
    pub fn reward_learning(&mut self, gu_id: &str, amount: f64) -> f64 {
        let reward = amount * self.config.learning_reward;
        self.add_coins(gu_id, reward, TransactionType::LearningReward, "学习奖励");
        reward
    }

    /// 推理消费
    pub fn cost_inference(&mut self, gu_id: &str, complexity: f64) -> Option<f64> {
        let cost = complexity * self.config.inference_cost;

        if self.balance(gu_id) < cost {
            return None;
        }

        self.subtract_coins(gu_id, cost, TransactionType::InferenceCost, "推理消费");
        Some(cost)
    }

    /// 正确推理奖励
    pub fn reward_correct(&mut self, gu_id: &str, task_price: f64) -> f64 {
        let reward = task_price * self.config.correct_reward;
        self.add_coins(gu_id, reward, TransactionType::CorrectReward, "正确推理奖励");
        reward
    }

    /// 错误推理惩罚
    pub fn penalize_wrong(&mut self, gu_id: &str, task_price: f64) -> f64 {
        let penalty = task_price * self.config.wrong_penalty;
        self.subtract_coins(gu_id, penalty, TransactionType::WrongPenalty, "错误推理惩罚");
        penalty
    }

    /// 发布知识
    pub fn publish_knowledge(&mut self, product: KnowledgeProduct) {
        // 动态定价
        let mut product = product;
        product.current_price = product.base_price;
        self.knowledge_market.insert(product.id.clone(), product);
    }

    /// 购买知识
    pub fn purchase_knowledge(&mut self, buyer: &str, product_id: &str) -> Option<f64> {
        let product = self.knowledge_market.get(product_id)?.clone();
        let price = product.current_price;

        if self.balance(buyer) < price {
            return None;
        }

        // 买家支付
        self.subtract_coins(buyer, price, TransactionType::KnowledgePurchase, "购买知识");

        // 卖家收款
        self.add_coins(&product.producer, price, TransactionType::KnowledgeSale, "出售知识");

        // 更新需求
        if let Some(p) = self.knowledge_market.get_mut(product_id) {
            p.demand += 1;
            self.adjust_price(product_id);
        }

        Some(price)
    }

    /// 发布推理任务
    pub fn publish_task(&mut self, task: InferenceTask) {
        self.task_market.insert(task.id.clone(), task);
    }

    /// 竞标任务
    pub fn bid_task(&mut self, task_id: &str, bidder: &str) -> bool {
        if let Some(task) = self.task_market.get_mut(task_id) {
            if !task.bidders.contains(&bidder.to_string()) {
                task.bidders.push(bidder.to_string());
                return true;
            }
        }
        false
    }

    /// 选择中标者
    pub fn award_task(&mut self, task_id: &str, winner: &str) -> Option<f64> {
        let task = self.task_market.get(task_id)?.clone();

        if !task.bidders.contains(&winner.to_string()) {
            return None;
        }

        let price = task.calculate_price();

        // 发布者支付
        self.subtract_coins(&task.publisher, price, TransactionType::InferenceCost, "任务支付");

        // 中标者收款
        self.add_coins(winner, price, TransactionType::CorrectReward, "任务奖励");

        // 更新任务
        if let Some(t) = self.task_market.get_mut(task_id) {
            t.winner = Some(winner.to_string());
        }

        Some(price)
    }

    /// 动态价格调节
    fn adjust_price(&mut self, product_id: &str) {
        if let Some(product) = self.knowledge_market.get_mut(product_id) {
            let demand_supply_ratio = if product.supply > 0 {
                product.demand as f64 / product.supply as f64
            } else {
                product.demand as f64
            };

            let adjustment = self.config.price_adjustment * (demand_supply_ratio - 1.0);
            product.current_price = product.base_price * (1.0 + adjustment);
        }
    }

    /// 添加认知币
    fn add_coins(&mut self, gu_id: &str, amount: f64, tx_type: TransactionType, desc: &str) {
        let balance = self.balances.entry(gu_id.to_string()).or_insert(0.0);
        *balance += amount;

        self.transactions.push(Transaction {
            id: format!("tx_{}", self.transactions.len()),
            gu_id: gu_id.to_string(),
            tx_type,
            amount,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            description: desc.to_string(),
        });
    }

    /// 扣除认知币
    fn subtract_coins(&mut self, gu_id: &str, amount: f64, tx_type: TransactionType, desc: &str) {
        let balance = self.balances.entry(gu_id.to_string()).or_insert(0.0);
        *balance = (*balance - amount).max(0.0);

        self.transactions.push(Transaction {
            id: format!("tx_{}", self.transactions.len()),
            gu_id: gu_id.to_string(),
            tx_type,
            amount: -amount,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            description: desc.to_string(),
        });
    }

    /// 获取交易历史
    pub fn transaction_history(&self) -> &[Transaction] {
        &self.transactions
    }

    /// 市场统计
    pub fn stats(&self) -> HashMap<String, f64> {
        let mut stats = HashMap::new();

        let total_coins: f64 = self.balances.values().sum();
        stats.insert("total_coins".to_string(), total_coins);
        stats.insert("knowledge_products".to_string(), self.knowledge_market.len() as f64);
        stats.insert("active_tasks".to_string(), self.task_market.len() as f64);
        stats.insert("transactions".to_string(), self.transactions.len() as f64);

        stats
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_market_creation() {
        let market = CognitiveMarketSystem::new(MarketConfig::default());
        assert!(market.balances.is_empty());
    }

    #[test]
    fn test_register_gu() {
        let mut market = CognitiveMarketSystem::new(MarketConfig::default());
        market.register_gu("gu_1".to_string());

        assert_eq!(market.balance("gu_1"), 100.0);
    }

    #[test]
    fn test_learning_reward() {
        let mut market = CognitiveMarketSystem::new(MarketConfig::default());
        market.register_gu("gu_1".to_string());

        let reward = market.reward_learning("gu_1", 10.0);
        assert_eq!(reward, 10.0);
        assert_eq!(market.balance("gu_1"), 110.0);
    }

    #[test]
    fn test_inference_cost() {
        let mut market = CognitiveMarketSystem::new(MarketConfig::default());
        market.register_gu("gu_1".to_string());

        let cost = market.cost_inference("gu_1", 5.0);
        assert!(cost.is_some());
        assert_eq!(market.balance("gu_1"), 95.0);
    }

    #[test]
    fn test_insufficient_balance() {
        let mut market = CognitiveMarketSystem::new(MarketConfig::default());
        market.register_gu("gu_1".to_string());

        let cost = market.cost_inference("gu_1", 200.0);
        assert!(cost.is_none());
        assert_eq!(market.balance("gu_1"), 100.0);
    }

    #[test]
    fn test_knowledge_trade() {
        let mut market = CognitiveMarketSystem::new(MarketConfig::default());
        market.register_gu("buyer".to_string());
        market.register_gu("seller".to_string());

        market.publish_knowledge(KnowledgeProduct {
            id: "k1".to_string(),
            content: "知识内容".to_string(),
            producer: "seller".to_string(),
            base_price: 10.0,
            current_price: 10.0,
            demand: 0,
            supply: 1,
        });

        let price = market.purchase_knowledge("buyer", "k1");
        assert!(price.is_some());
        assert_eq!(market.balance("buyer"), 90.0);
        assert_eq!(market.balance("seller"), 110.0);
    }

    #[test]
    fn test_task_bidding() {
        let mut market = CognitiveMarketSystem::new(MarketConfig::default());
        market.register_gu("publisher".to_string());
        market.register_gu("bidder".to_string());

        market.publish_task(InferenceTask {
            id: "task_1".to_string(),
            difficulty: 1.0,
            urgency: 1.0,
            base_price: 10.0,
            publisher: "publisher".to_string(),
            bidders: Vec::new(),
            winner: None,
        });

        assert!(market.bid_task("task_1", "bidder"));

        let reward = market.award_task("task_1", "bidder");
        assert!(reward.is_some());
        assert_eq!(market.balance("bidder"), 120.0); // 100 + 20 (2x reward)
    }
}
