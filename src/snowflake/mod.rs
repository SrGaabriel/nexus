use std::ops::{Shl, Sub};

use chrono::{NaiveDateTime, Utc};

static EPOCH: i64 = 1514772000000;
static NODE_BITS: i64 = 10;
static STEP_BITS: i64 = 12;
static TIME_SHIFT: i64 = NODE_BITS + STEP_BITS;

pub struct SnowflakeCluster {
    node: i64,
    sequence: i64,
    last_snowflake_millis: i64,
}

impl SnowflakeCluster {
    pub fn new(id: i64) -> Self {
        SnowflakeCluster {
            node: id,
            sequence: 0,
            last_snowflake_millis: 0,
        }
    }

    pub fn create(&mut self) -> Snowflake {
        let now = Utc::now().timestamp_millis();
        let epoch = now - EPOCH;

        if &epoch == &self.last_snowflake_millis {
            self.sequence = ((self.sequence + 1) & -1) ^ (-1 << STEP_BITS)
        }
        self.last_snowflake_millis = now.clone();

        Snowflake {
            value: ((epoch << TIME_SHIFT)
                | (self.node << NODE_BITS)
                | STEP_BITS),
        }
    }
}

pub struct Snowflake {
    value: i64,
}

impl Snowflake {
    pub fn to_timestamp(&self) -> NaiveDateTime {
        let millis = EPOCH + (self.value >> TIME_SHIFT);
        NaiveDateTime::from_timestamp(millis / 1_000, 0)
    }
}

impl From<String> for Snowflake {
    fn from(value: String) -> Self {
        Snowflake {
            value: value
                .parse::<i64>()
                .expect("invalid string provided for snowflake conversion"),
        }
    }
}

impl From<i64> for Snowflake {
    fn from(value: i64) -> Self {
        Snowflake { value: value.abs() }
    }
}

impl From<NaiveDateTime> for Snowflake {
    fn from(value: NaiveDateTime) -> Self {
        Snowflake {
            value: value
                .timestamp_millis()
                .sub(EPOCH)
                .shl(TIME_SHIFT),
        }
    }
}

