pub mod async_generator {
    use crate::common::SnowflakeState as Snowflake;
    use crate::common::Result;

    /// An asynchronous Snowflake ID generator using Tokio's Mutex for thread safety.
    pub struct SnowflakeGenerator {
        inner: std::sync::Arc<tokio::sync::Mutex<Snowflake>>,
    }

    impl SnowflakeGenerator {

        /// Create a new asynchronous Snowflake ID generator.
        /// 
        /// # Arguments
        /// * `epoch` - The custom epoch timestamp in milliseconds.
        /// * `worker_id` - The worker ID (0-1023).
        /// # Errors
        /// Returns `SnowflakeError::WorkerIdOutOfRange` if the worker_id is out of range.
        /// 
        /// # Panics
        /// Panics if the epoch is set in the future relative to the current system time.
        pub fn new(epoch: i64, worker_id: u16) -> Result<Self> {
            Ok(Self {
                inner: std::sync::Arc::new(tokio::sync::Mutex::new(Snowflake::new(epoch, worker_id)?)),
            })
        }

        /// Asynchronously generate a new Snowflake ID.
        pub async fn generate_id(&self) -> i64 {
            let mut guard = self.inner.lock().await;
            guard.generate_id()
        }
    }

    impl Clone for SnowflakeGenerator {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
}

pub mod sync_generator {
    use crate::common::SnowflakeState as Snowflake;
    use crate::common::Result;

    pub struct SnowflakeGenerator {
        inner: std::sync::Arc<std::sync::Mutex<Snowflake>>,
    }

    impl SnowflakeGenerator {
        /// Create a new synchronous Snowflake ID generator.
        /// 
        /// # Arguments
        /// * `epoch` - The custom epoch timestamp in milliseconds.
        /// * `worker_id` - The worker ID (0-1023).
        /// # Errors
        /// Returns `SnowflakeError::WorkerIdOutOfRange` if the worker_id is out of range.
        /// 
        /// # Panics
        /// Panics if the epoch is set in the future relative to the current system time.
        pub fn new(epoch: i64, worker_id: u16) -> Result<Self> {
            Ok(Self {
                inner: std::sync::Arc::new(std::sync::Mutex::new(Snowflake::new(epoch, worker_id)?)),
            })
        }

        /// Generate a new Snowflake ID.
        /// 
        /// # Panics
        /// Panics if the internal Mutex is poisoned.
        pub fn generate_id(&self) -> i64 {
            let mut guard = self.inner.lock();
            match guard {
                Ok(ref mut g) => g.generate_id(),
                Err(e) => {
                    panic!("Mutex poisoned: {}", e);
                },
            }
        }
    }

    impl Clone for SnowflakeGenerator {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
}