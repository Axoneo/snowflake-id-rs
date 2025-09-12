pub struct SnowflakeState {
    pub time_since_epoch: i64,
    pub worker_id: u16,
    pub sequence: u16,
    pub epoch: i64,
    instant: std::time::Instant,
    instant_timestamp: i64,
}

impl SnowflakeState {
    /// Create a new SnowflakeState with the given worker_id.
    /// # Arguments
    /// * `worker_id` - A unique identifier for the worker (0-1023).
    /// # Examples
    /// ```
    /// let mut snowflake = SnowflakeState::new(1);
    /// ```
    /// # Errors
    /// This function will return an error if the system time is before the UNIX_EPOCH.
    /// It will also return an error if the worker_id is out of range (0-1023).
    /// 
    /// # Note
    /// This function uses std::time::Instant and std::time::SystemTime to get the current time.
    /// The worker_id is stored in the upper bits of the generated ID.
    /// The sequence number is incremented for each ID generated in the same millisecond.
    /// If more than 4096 IDs are requested in the same millisecond, the function will block until the next millisecond.
    /// The generated ID is a 64-bit integer that is unique across all workers and time.
    /// The ID is composed of a timestamp, worker_id, and sequence number.
    /// The timestamp is the number of milliseconds since the custom epoch (2025-01-01T00:00:00.000Z).
    /// The worker_id is a unique identifier for the worker (0-1023).
    /// The sequence number is a counter that is incremented for each ID generated in the same millisecond (0-4095).
    pub fn new(epoch: i64, worker_id: u16) -> Result<Self> {
        if worker_id > 0x3FF {
            return Err(SnowflakeError::WorkerIdOutOfRange);
        }
        let instant = std::time::Instant::now();
        let instant_timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis() as i64)
            .map_err(|_| SnowflakeError::TimeBeforeUnixEpoch)?;
        if instant_timestamp < epoch {
            return Err(SnowflakeError::EpochInFuture);
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

    /// Generate a new Snowflake ID.
    /// # Examples
    /// ```
    /// let mut snowflake = SnowflakeState::new(1);
    /// let id = snowflake.generate_id();
    /// ```
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
}

#[derive(Debug)]
pub enum SnowflakeError {
    /// Error when the system time is before the UNIX_EPOCH.
    TimeBeforeUnixEpoch,
    /// Error when the worker_id is out of range (0-1023).
    WorkerIdOutOfRange,
    /// Error when the epoch is in the future.
    EpochInFuture,
    /// Poison error for Mutex.
    PoisonError,
}

impl std::fmt::Display for SnowflakeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SnowflakeError::TimeBeforeUnixEpoch => write!(f, "System time is before UNIX_EPOCH"),
            SnowflakeError::WorkerIdOutOfRange => write!(f, "Worker ID is out of range (0-1023)"),
            SnowflakeError::EpochInFuture => write!(f, "Epoch is in the future"),
            SnowflakeError::PoisonError => write!(f, "Mutex is poisoned"),
        }
    }
}

impl std::error::Error for SnowflakeError {}

pub type Result<T> = std::result::Result<T, SnowflakeError>;