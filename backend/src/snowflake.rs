use std::sync::atomic::{AtomicU16, AtomicU64, Ordering};

static SEQUENCE: AtomicU16 = AtomicU16::new(0);
static TIME: AtomicU64 = AtomicU64::new(0);

pub struct Snowflake {
    pub snowflake: u64,
}

impl Snowflake {
    pub fn new() -> Self {
        let now = time::OffsetDateTime::now_utc();
        if now.unix_timestamp().unsigned_abs() != TIME.load(Ordering::Relaxed) {
            TIME.store(now.unix_timestamp().unsigned_abs(), Ordering::Relaxed);
            SEQUENCE.store(0, Ordering::Relaxed);
        }
        let time = TIME.load(Ordering::Relaxed);
        let sequence = SEQUENCE.load(Ordering::Relaxed);

        let snowflake = (time << 16) + sequence as u64;
        SEQUENCE.store(sequence + 1, Ordering::Relaxed);

        Self { snowflake }
    }
}

impl From<u64> for Snowflake {
    fn from(value: u64) -> Self {
        Self { snowflake: value }
    }
}
