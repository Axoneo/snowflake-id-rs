pub struct SnowflakeState {
    pub time_since_epoch: i64,
    pub worker_id: u16,
    pub sequence: u16,
    pub epoch: i64,
    instant: std::time::Instant,
    instant_timestamp: i64,
}

impl SnowflakeState {
    pub fn new(epoch: i64, worker_id: u16) -> Result<Self> {
        if worker_id > 0x3FF {
            return Err(SnowflakeError::WorkerIdOutOfRange);
        }
        let instant = std::time::Instant::now();
        let instant_timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis() as i64)
            .map_err(|_| panic!("Failed to get instant timestamp"))?;
        if instant_timestamp < epoch {
            panic!("Epoch is in the future");
        }
        Ok(Self {
            time_since_epoch: instant_timestamp - epoch,
            instant_timestamp,
            instant,
            worker_id,
            epoch,
            sequence: 0,
        })
    }

    fn to_i64(&self) -> i64 {
        ((self.time_since_epoch << 22) | ((self.worker_id as i64) << 12) | (self.sequence as i64)) & 0x7FFFFFFFFFFFFFFF
    }

    fn get_time_since_epoch(&self) -> i64 {
        self.instant_timestamp + self.instant.elapsed().as_millis() as i64 - self.epoch
    }

    pub fn generate_id(&mut self) -> i64 {
        let current_time = self.get_time_since_epoch();
        if self.time_since_epoch == current_time {
            if self.sequence > 0xFFF {
                std::thread::sleep(std::time::Duration::from_millis(1));
                self.time_since_epoch = self.get_time_since_epoch();
                self.sequence = 0;
            }
        } else {
            self.time_since_epoch = current_time;
            self.sequence = 0;
        }
        let id = self.to_i64();

        self.sequence += 1;
        id
    }

    pub fn decompose(&self, id: i64) -> SnowflakeDecomposed {
        let timestamp = (id >> 22) + self.epoch;
        let worker_id = ((id >> 12) & 0x3FF) as u16;
        let sequence = (id & 0xFFF) as u16;

        SnowflakeDecomposed {
            timestamp,
            worker_id,
            sequence,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SnowflakeDecomposed {
    /// The timestamp component of the Snowflake ID in milliseconds since epoch.
    pub timestamp: i64,
    /// The worker ID component of the Snowflake ID.
    pub worker_id: u16,
    /// The sequence number component of the Snowflake ID.
    pub sequence: u16,
}



#[derive(Debug)]
pub enum SnowflakeError {
    /// Error when the worker_id is out of range (0-1023).
    WorkerIdOutOfRange,
}

impl std::fmt::Display for SnowflakeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SnowflakeError::WorkerIdOutOfRange => write!(f, "Worker ID is out of range (0-1023)")
        }
    }
}

impl std::error::Error for SnowflakeError {}

pub type Result<T> = std::result::Result<T, SnowflakeError>;