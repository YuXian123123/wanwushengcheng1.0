/**
 * 恶意节点检测和防御
 * 
 * 实现行为异常检测系统和意识污染防护墙
 */

// 蛊虫行为模式接口
export interface WormBehaviorPattern {
  wormId: string;
  timestamp: Date;
  activityType: string;
  dataVolume: number;
  connectionCount: number;
  anomalyScore: number; // 异常分数，0-1，1为最异常
}

// 恶意行为特征接口
export interface MaliciousBehaviorSignature {
  id: string;
  name: string;
  patterns: string[]; // 行为模式特征
  severity: 'low' | 'medium' | 'high' | 'critical';
}

// 行为异常检测系统
export class BehaviorAnomalyDetectionSystem {
  private normalBehaviorProfiles: Map<string, any> = new Map(); // 正常行为配置文件
  private maliciousSignatures: MaliciousBehaviorSignature[] = [];
  private behaviorHistory: WormBehaviorPattern[] = [];
  private anomalyThreshold: number = 0.8; // 异常阈值

  constructor() {}

  /**
   * 添加恶意行为特征
   */
  public addMaliciousSignature(signature: MaliciousBehaviorSignature): void {
    this.maliciousSignatures.push(signature);
    console.log(`添加恶意行为特征: ${signature.name} (${signature.id})`);
  }

  /**
   * 收集正常行为数据
   */
  public collectNormalBehaviorData(wormId: string, pattern: any): void {
    if (!this.normalBehaviorProfiles.has(wormId)) {
      this.normalBehaviorProfiles.set(wormId, []);
    }
    
    const profiles = this.normalBehaviorProfiles.get(wormId);
    profiles.push(pattern);
    
    // 限制存储的历史数据量
    if (profiles.length > 100) {
      profiles.shift();
    }
    
    console.log(`收集正常行为数据: 蛊虫 ${wormId}`);
  }

  /**
   * 分析行为模式
   */
  public analyzeBehaviorPattern(pattern: WormBehaviorPattern): number {
    // 计算与正常行为的偏差
    const deviationScore = this.calculateDeviationScore(pattern);
    
    // 检查是否匹配已知的恶意特征
    const signatureMatchScore = this.checkSignatureMatch(pattern);
    
    // 综合异常分数
    const anomalyScore = Math.max(deviationScore, signatureMatchScore);
    pattern.anomalyScore = anomalyScore;
    
    // 存储行为历史
    this.behaviorHistory.push(pattern);
    
    // 限制历史数据量
    if (this.behaviorHistory.length > 1000) {
      this.behaviorHistory.shift();
    }
    
    console.log(`分析行为模式: 蛊虫 ${pattern.wormId}, 异常分数 ${anomalyScore.toFixed(2)}`);
    return anomalyScore;
  }

  /**
   * 计算与正常行为的偏差分数
   */
  private calculateDeviationScore(pattern: WormBehaviorPattern): number {
    const normalPatterns = this.normalBehaviorProfiles.get(pattern.wormId);
    if (!normalPatterns || normalPatterns.length === 0) {
      // 如果没有正常行为数据，返回中等异常分数
      return 0.5;
    }
    
    // 简化的偏差计算
    // 实际实现中会使用更复杂的机器学习模型
    const recentPatterns = normalPatterns.slice(-10); // 最近10个正常模式
    
    let totalDeviation = 0;
    recentPatterns.forEach(normalPattern => {
      // 计算数据量偏差
      const dataDeviation = Math.abs(pattern.dataVolume - normalPattern.dataVolume) / 
                           Math.max(pattern.dataVolume, normalPattern.dataVolume, 1);
      
      // 计算连接数偏差
      const connectionDeviation = Math.abs(pattern.connectionCount - normalPattern.connectionCount) / 
                                Math.max(pattern.connectionCount, normalPattern.connectionCount, 1);
      
      // 综合偏差
      totalDeviation += (dataDeviation + connectionDeviation) / 2;
    });
    
    const averageDeviation = totalDeviation / recentPatterns.length;
    
    // 将偏差转换为0-1的分数
    return Math.min(1, averageDeviation);
  }

  /**
   * 检查是否匹配恶意特征
   */
  private checkSignatureMatch(pattern: WormBehaviorPattern): number {
    // 简化的特征匹配
    // 实际实现中会使用更复杂的模式匹配算法
    
    let maxMatchScore = 0;
    
    this.maliciousSignatures.forEach(signature => {
      let matchCount = 0;
      
      signature.patterns.forEach(patternStr => {
        // 检查活动类型匹配
        if (pattern.activityType.includes(patternStr)) {
          matchCount++;
        }
        
        // 可以添加更多匹配规则
      });
      
      // 计算匹配分数
      const matchScore = matchCount / signature.patterns.length;
      
      if (matchScore > maxMatchScore) {
        maxMatchScore = matchScore;
      }
    });
    
    return maxMatchScore;
  }

  /**
   * 标记异常节点
   */
  public flagAnomalousNode(wormId: string, anomalyScore: number): boolean {
    if (anomalyScore >= this.anomalyThreshold) {
      console.warn(`标记异常节点: 蛊虫 ${wormId}, 异常分数 ${anomalyScore.toFixed(2)}`);
      
      // 可以在这里触发其他操作，如隔离节点、发送警报等
      return true;
    }
    
    return false;
  }

  /**
   * 获取异常节点列表
   */
  public getAnomalousNodes(): string[] {
    const anomalousNodes: string[] = [];
    
    // 从行为历史中找出高异常分数的节点
    const recentHistory = this.behaviorHistory.slice(-100); // 最近100条记录
    
    recentHistory.forEach(pattern => {
      if (pattern.anomalyScore >= this.anomalyThreshold && 
          !anomalousNodes.includes(pattern.wormId)) {
        anomalousNodes.push(pattern.wormId);
      }
    });
    
    return anomalousNodes;
  }

  /**
   * 验证异常检测系统
   */
  public validateAnomalyDetection(): boolean {
    console.log('验证异常检测系统...');
    
    // 检查是否有恶意特征和正常行为数据
    const hasSignatures = this.maliciousSignatures.length > 0;
    const hasNormalData = this.normalBehaviorProfiles.size > 0;
    
    console.log(`验证结果 - 有恶意特征: ${hasSignatures}, 有正常数据: ${hasNormalData}`);
    return hasSignatures && hasNormalData;
  }
}

// 意识污染防护墙
export class ConsciousnessContaminationFirewall {
  private protectionLayers: number = 3; // 防护层数
  private layerIntegrity: boolean[] = [true, true, true]; // 各层完整性状态
  private blockedPackets: number = 0; // 拦截的数据包数量
  private maxBlockedPackets: number = 1000; // 最大拦截数量记录

  constructor() {}

  /**
   * 检查防护墙完整性
   */
  public checkIntegrity(): boolean[] {
    console.log('检查防护墙完整性...');
    
    // 简化的完整性检查
    // 实际实现中会进行更复杂的验证
    
    // 随机模拟完整性检查结果
    this.layerIntegrity = this.layerIntegrity.map(() => Math.random() > 0.1);
    
    console.log(`防护墙完整性状态: ${this.layerIntegrity.join(', ')}`);
    return [...this.layerIntegrity];
  }

  /**
   * 拦截恶意数据包
   */
  public interceptMaliciousPacket(packet: any): boolean {
    // 简化的恶意数据包检测
    // 实际实现中会进行深度包检测
    
    // 模拟检测逻辑
    const isMalicious = Math.random() > 0.8; // 20%概率为恶意数据包
    
    if (isMalicious) {
      this.blockedPackets++;
      if (this.blockedPackets > this.maxBlockedPackets) {
        this.blockedPackets = 1; // 重置计数
      }
      
      console.warn('拦截恶意数据包');
      return true;
    }
    
    return false;
  }

  /**
   * 获取拦截统计
   */
  public getInterceptionStats(): { blocked: number; layers: boolean[] } {
    return {
      blocked: this.blockedPackets,
      layers: [...this.layerIntegrity]
    };
  }

  /**
   * 修复防护墙
   */
  public repairFirewall(): void {
    console.log('修复防护墙...');
    
    // 修复所有层
    this.layerIntegrity = this.layerIntegrity.map(() => true);
    
    console.log('防护墙修复完成');
  }

  /**
   * 验证防护墙
   */
  public validateFirewall(): boolean {
    console.log('验证防护墙...');
    
    // 检查所有层是否完整
    const allLayersIntact = this.layerIntegrity.every(intact => intact);
    
    console.log(`验证结果 - 所有层完整: ${allLayersIntact}`);
    return allLayersIntact;
  }
}

// 恶意节点检测验证逻辑
export class MaliciousNodeDetectionValidator {
  /**
   * 验证行为异常检测系统
   */
  public static validateAnomalyDetection(system: BehaviorAnomalyDetectionSystem): boolean {
    console.log('验证行为异常检测系统...');
    
    return system.validateAnomalyDetection();
  }

  /**
   * 验证意识污染防护墙
   */
  public static validateContaminationFirewall(firewall: ConsciousnessContaminationFirewall): boolean {
    console.log('验证意识污染防护墙...');
    
    return firewall.validateFirewall();
  }
}
