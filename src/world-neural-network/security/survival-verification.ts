/**
 * 生存绑定的安全验证机制
 * 
 * 实现核心蛊虫冗余系统和生存状态心跳协议
 */

// 核心蛊虫状态接口
export interface CoreWormState {
  id: string;
  isAlive: boolean;
  lastHeartbeat: Date;
  healthScore: number; // 0-100, 100为最佳状态
  isActive: boolean; // 是否为活跃核心
}

// 心跳信号接口
export interface HeartbeatSignal {
  wormId: string;
  timestamp: Date;
  signature: string; // 数字签名
  nonce: string; // 防重放攻击
}

// 核心蛊虫冗余系统
export class CoreWormRedundancySystem {
  private coreWorms: CoreWormState[] = [];
  private backupWorms: CoreWormState[] = [];
  private activeThreshold: number = 3; // 最少活跃核心数量
  private healthThreshold: number = 70; // 健康分数阈值

  constructor(coreWorms: CoreWormState[], backupWorms: CoreWormState[]) {
    this.coreWorms = coreWorms;
    this.backupWorms = backupWorms;
  }

  /**
   * 监控核心蛊虫健康状态
   */
  public monitorHealth(): void {
    console.log('监控核心蛊虫健康状态...');
    
    // 检查活跃核心数量
    const activeCores = this.coreWorms.filter(worm => worm.isActive && worm.isAlive);
    console.log(`当前活跃核心数量: ${activeCores.length}`);
    
    // 检查健康分数
    const lowHealthCores = this.coreWorms.filter(
      worm => worm.isAlive && worm.healthScore < this.healthThreshold
    );
    
    if (lowHealthCores.length > 0) {
      console.warn(`发现${lowHealthCores.length}个低健康分数的核心蛊虫:`, 
        lowHealthCores.map(w => w.id));
    }
    
    // 如果活跃核心数量低于阈值，激活备份
    if (activeCores.length < this.activeThreshold) {
      this.activateBackupWorms();
    }
  }

  /**
   * 激活备份蛊虫
   */
  private activateBackupWorms(): void {
    console.log('激活备份蛊虫...');
    
    const inactiveBackups = this.backupWorms.filter(worm => !worm.isActive && worm.isAlive);
    
    if (inactiveBackups.length === 0) {
      console.error('没有可用的备份蛊虫!');
      return;
    }
    
    // 激活足够的备份以满足阈值
    const needed = this.activeThreshold - this.coreWorms.filter(w => w.isActive && w.isAlive).length;
    const toActivate = inactiveBackups.slice(0, Math.min(needed, inactiveBackups.length));
    
    toActivate.forEach(worm => {
      worm.isActive = true;
      console.log(`激活备份蛊虫: ${worm.id}`);
    });
  }

  /**
   * 执行冗余切换演练
   */
  public performRedundancyDrill(): void {
    console.log('执行冗余切换演练...');
    
    // 模拟一个核心蛊虫失效
    if (this.coreWorms.length > 0) {
      const randomCore = this.coreWorms[Math.floor(Math.random() * this.coreWorms.length)];
      console.log(`模拟核心蛊虫失效: ${randomCore.id}`);
      randomCore.isAlive = false;
      randomCore.isActive = false;
      
      // 检查并激活备份
      this.monitorHealth();
      
      // 恢复模拟的失效
      randomCore.isAlive = true;
      console.log(`恢复模拟失效的核心蛊虫: ${randomCore.id}`);
    }
  }
}

// 生存状态心跳协议
export class SurvivalHeartbeatProtocol {
  private worms: Map<string, CoreWormState> = new Map();
  private heartbeatInterval: number = 5000; // 5秒心跳间隔
  private maxMissedBeats: number = 3; // 最大错过心跳次数
  private missedBeats: Map<string, number> = new Map();

  constructor() {
    // 初始化 missedBeats 计数器
    this.worms.forEach((_, wormId) => {
      this.missedBeats.set(wormId, 0);
    });
  }

  /**
   * 注册蛊虫
   */
  public registerWorm(wormState: CoreWormState): void {
    this.worms.set(wormState.id, wormState);
    this.missedBeats.set(wormState.id, 0);
    console.log(`注册蛊虫: ${wormState.id}`);
  }

  /**
   * 处理心跳信号
   */
  public processHeartbeat(signal: HeartbeatSignal): boolean {
    // 验证数字签名（简化实现）
    if (!this.verifySignature(signal)) {
      console.warn(`心跳信号签名验证失败: ${signal.wormId}`);
      return false;
    }

    // 验证时间戳（防止重放攻击）
    const now = new Date();
    const timeDiff = Math.abs(now.getTime() - signal.timestamp.getTime());
    if (timeDiff > 60000) { // 1分钟内有效
      console.warn(`心跳信号时间戳过期: ${signal.wormId}`);
      return false;
    }

    // 更新蛊虫状态
    const worm = this.worms.get(signal.wormId);
    if (!worm) {
      console.warn(`未知的蛊虫心跳: ${signal.wormId}`);
      return false;
    }

    worm.lastHeartbeat = new Date();
    worm.isAlive = true;
    
    // 重置错过心跳计数
    this.missedBeats.set(signal.wormId, 0);
    
    console.log(`处理心跳信号成功: ${signal.wormId}`);
    return true;
  }

  /**
   * 验证数字签名（简化实现）
   */
  private verifySignature(signal: HeartbeatSignal): boolean {
    // 实际实现应该使用加密库验证签名
    // 这里简化为检查签名是否存在
    return signal.signature && signal.signature.length > 0;
  }

  /**
   * 检查错过的心跳
   */
  public checkMissedHeartbeats(): void {
    console.log('检查错过的心跳...');
    
    const now = new Date();
    this.worms.forEach((worm, wormId) => {
      // 计算上次心跳到现在的时间
      const timeSinceLastBeat = now.getTime() - worm.lastHeartbeat.getTime();
      
      // 如果超过心跳间隔时间，则增加错过计数
      if (timeSinceLastBeat > this.heartbeatInterval) {
        const missed = this.missedBeats.get(wormId) || 0;
        this.missedBeats.set(wormId, missed + 1);
        
        console.warn(`蛊虫 ${wormId} 错过心跳 #${missed + 1}`);
        
        // 如果错过心跳次数超过阈值，标记为死亡
        if (missed + 1 >= this.maxMissedBeats) {
          worm.isAlive = false;
          worm.isActive = false;
          console.error(`蛊虫 ${wormId} 被标记为死亡`);
        }
      }
    });
  }

  /**
   * 启动心跳监控
   */
  public startMonitoring(): void {
    console.log('启动心跳监控...');
    
    // 定期检查错过的心跳
    setInterval(() => {
      this.checkMissedHeartbeats();
    }, this.heartbeatInterval);
  }
}

// 验证逻辑实现
export class SurvivalVerificationValidator {
  /**
   * 验证核心蛊虫冗余系统
   */
  public static validateRedundancySystem(system: CoreWormRedundancySystem): boolean {
    console.log('验证核心蛊虫冗余系统...');
    
    // 检查是否有足够的核心蛊虫
    const activeCores = system['coreWorms'].filter((worm: CoreWormState) => worm.isActive && worm.isAlive);
    const hasEnoughCores = activeCores.length >= system['activeThreshold'];
    
    // 检查是否有可用的备份蛊虫
    const availableBackups = system['backupWorms'].filter((worm: CoreWormState) => worm.isAlive);
    const hasBackups = availableBackups.length > 0;
    
    console.log(`验证结果 - 足够的核心: ${hasEnoughCores}, 有备份: ${hasBackups}`);
    return hasEnoughCores && hasBackups;
  }

  /**
   * 验证心跳协议
   */
  public static validateHeartbeatProtocol(protocol: SurvivalHeartbeatProtocol): boolean {
    console.log('验证心跳协议...');
    
    // 检查是否有注册的蛊虫
    const wormCount = protocol['worms'].size;
    const hasWorms = wormCount > 0;
    
    // 检查心跳间隔设置是否合理
    const validInterval = protocol['heartbeatInterval'] > 0 && protocol['heartbeatInterval'] <= 60000;
    
    console.log(`验证结果 - 有蛊虫: ${hasWorms}, 有效间隔: ${validInterval}`);
    return hasWorms && validInterval;
  }
}
