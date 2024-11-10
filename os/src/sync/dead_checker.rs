const MAX_RESOURCE: usize = 32;
const MAX_THREAD: usize = 32;

/// 死锁检测器
pub struct DeadChecker{
    available: [u32; MAX_RESOURCE],
    /// allocation[i][j] 表示 线程i 当前己分得第j类 资源的数量
    allocation: [[u32; MAX_RESOURCE]; MAX_THREAD], 
    /// need[i][j] 表示 线程i 还需要 第j类 资源的数量
    need: [[u32; MAX_RESOURCE]; MAX_THREAD],
}

impl DeadChecker {
    /// 创建
    pub fn new() -> Self {
        Self {
            available: [0; MAX_RESOURCE],
            allocation: [[0; MAX_RESOURCE]; MAX_THREAD], 
            need: [[0; MAX_RESOURCE]; MAX_THREAD],
        }
    }

    fn can_allo(&self, work: &[u32], need: &[u32]) -> bool {
        for i in 0..MAX_RESOURCE {
            if need[i] > work[i] {
                return false;
            }
        }
        true
    }
    fn add(&self, work: &mut [u32], need: &[u32]) {
        for i in 0..MAX_RESOURCE {
            work[i] += need[i];
        }
    }
    /// 设置第i类资源可用数量
    pub fn init_resource(&mut self, i: usize, num: usize) {
        self.available[i] = num as u32;
    }
    /// 释放资源
    pub fn release(&mut self, thread_id: usize, resource_id: usize) {
        self.available[resource_id] += 1;
        self.allocation[thread_id][resource_id] -= 1;
    }
    /// 申请资源
    pub fn try_request(&mut self, thread_id: usize, resource_id: usize, _tlen: usize) -> bool {
        self.need[thread_id][resource_id] += 1;
        let mut work = self.available;
        let mut finish = [false; MAX_THREAD];
        
        loop {
            let mut can_continue = false;
            for i in 0..MAX_THREAD {
                if !finish[i] && self.can_allo(work.as_ref(), self.need[i].as_ref()) {
                    self.add(work.as_mut(), self.allocation[i].as_ref());
                    finish[i] = true;
                    can_continue = true;
                }
            }
            if !can_continue {
                break;
            }
        }

        if finish.iter().any(|&x| !x) {
            return false;
        }
        true
    }
    /// finish
    pub fn finish(&mut self, thread_id: usize, resource_id: usize) {
        self.allocation[thread_id][resource_id] +=1;
        self.available[resource_id] -= 1;
        self.need[thread_id][resource_id] -= 1;
    }
}
