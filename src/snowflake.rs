use chrono;

const START_STMP: i64 = 1602832597000;

// 每一部分占用的位数
const SEQUENCE_BIT: i64 = 6; // 序列号占用的位数
const MACHINE_BIT: i64 = 5; // 机器标识占用的位数
const DATACENTER_BIT: i64 = 5; // 数据中心占用的位数

// 每一部分的最大值
const MAX_DATACENTER_NUM: i64 = -1 ^ (-1 << DATACENTER_BIT); // 31
const MAX_MACHINE_NUM: i64 = -1 ^ (-1 << MACHINE_BIT); // 31
const MAX_SEQUENCE: i64 = -1 ^ (-1 << SEQUENCE_BIT); // 63

// 每一部分向左的位移
const MACHINE_LEFT: i64 = SEQUENCE_BIT;
const DATACENTER_LEFT: i64 = SEQUENCE_BIT + MACHINE_BIT;
const TIMESTMP_LEFT: i64 = DATACENTER_LEFT + DATACENTER_BIT;

pub struct Snowflake {
    data_center_id: i64,
    machine_id: i64,
    last_stmp: i64,
    sequence: i64,
}

impl Snowflake {
    pub fn new(data_center_id: i64, machine_id: i64) -> Self {
        if data_center_id > MAX_DATACENTER_NUM || data_center_id < 0 {
            panic!("dataCenterId can't be greater than MAX_DATACENTER_NUM or less than 0");
        }
        if machine_id > MAX_MACHINE_NUM || machine_id < 0 {
            panic!("machineId can't be greater than MAX_MACHINE_NUM or less than 0");
        }
        Snowflake {
            data_center_id: data_center_id,
            machine_id: machine_id,
            last_stmp: 0,
            sequence: 0,
        }
    }

    fn get_next_mill(&self) -> i64 {
        let mut mill = self.get_new_stmp();
        while mill <= self.last_stmp {
            mill = self.get_new_stmp();
        }
        mill
    }

    #[inline]
    fn get_new_stmp(&self) -> i64 {
        chrono::offset::Local::now().timestamp_millis()
    }

    pub fn next_id(&mut self) -> i64 {
        let mut curr_stmp = self.get_new_stmp();
        if curr_stmp < self.last_stmp {
            panic!("Clock moved backwards.  Refusing to generate id");
        }
        if curr_stmp == self.last_stmp {
            // 相同毫秒内，序列号自增
            self.sequence = (self.sequence + 1) & MAX_SEQUENCE;
            //同一毫秒的序列数已经达到最大
            if self.sequence == 0 {
                curr_stmp = self.get_next_mill();
            }
        } else {
            // 不同毫秒内，序列号置为0
            self.sequence = 0;
        }
        self.last_stmp = curr_stmp;
        // 时间戳部分 |  #数据中心部分 |  #机器标识部分  |  #序列号部分
        (curr_stmp - START_STMP) << TIMESTMP_LEFT
            | self.data_center_id << DATACENTER_LEFT
            | self.machine_id << MACHINE_LEFT
            | self.sequence
    }
}
