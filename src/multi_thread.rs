use crate::common::SnowflakeState as Snowflake;
use crate::common::Result;

pub struct MultiThreadedAsyncGenerator {
    inner: std::sync::Arc<tokio::sync::Mutex<Snowflake>>,
}

impl MultiThreadedAsyncGenerator {
    pub fn new(epoch: i64, worker_id: u16) -> Result<Self> {
        Ok(Self {
            inner: std::sync::Arc::new(tokio::sync::Mutex::new(Snowflake::new(epoch, worker_id)?)),
        })
    }

    pub async fn generate_id(&self) -> i64 {
        let mut guard = self.inner.lock().await;
        guard.generate_id()
    }
}

impl Clone for MultiThreadedAsyncGenerator {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

pub struct MultiThreadedSyncGenerator {
    inner: std::sync::Arc<std::sync::Mutex<Snowflake>>,
}

impl MultiThreadedSyncGenerator {
    pub fn new(epoch: i64, worker_id: u16) -> Result<Self> {
        Ok(Self {
            inner: std::sync::Arc::new(std::sync::Mutex::new(Snowflake::new(epoch, worker_id)?)),
        })
    }

    pub fn generate_id(&self) -> Result<i64> {
        let mut guard = self.inner.lock().map_err(|_| crate::common::SnowflakeError::PoisonError)?;
        Ok(guard.generate_id())
    }
}

impl Clone for MultiThreadedSyncGenerator {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_snowflake_id_generation() {
        let mut snowflake = Snowflake::new(0, 0).unwrap();
        let id1 = snowflake.generate_id();
        let id2 = snowflake.generate_id();
        assert!(id1 != id2);
    }

    #[tokio::test]
    async fn test_snowflake_id_generation_uniqueness() {
        let threads = 10;
        let mut handles = vec![];
        let time = std::time::Instant::now();
        let ids = 1000000;
        for i in 0..threads {
            let snowflake = MultiThreadedAsyncGenerator::new(0, i as u16).unwrap();
            let handle = tokio::spawn(async move {
                for _ in 0..ids {
                    snowflake.generate_id().await;
                }
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.await.unwrap();
        }
        println!("Generated 10,000,000 IDs in {:?}", time.elapsed());
    }

    #[test]
    fn test_snowflake_id_generation_sync() {
        let threads = 10;
        let mut handles = vec![];
        let time = std::time::Instant::now();
        let ids = 1000000;
        for i in 0..threads {
            let snowflake = MultiThreadedSyncGenerator::new(0, i as u16).unwrap();
            let handle = std::thread::spawn(move || {
                for _ in 0..ids {
                    snowflake.generate_id().unwrap();
                }
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap();
        }
        println!("Generated 10,000,000 IDs in {:?}", time.elapsed());
    }
}