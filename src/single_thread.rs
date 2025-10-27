pub mod sync_generator {
    use crate::common::SnowflakeState as Snowflake;
    use crate::common::Result;

    pub struct SnowflakeGenerator {
        inner: std::rc::Rc<std::cell::RefCell<Snowflake>>,
    }

    impl SnowflakeGenerator {
        pub fn new(epoch: i64, worker_id: u16) -> Result<Self> {
            Ok(Self {
                inner: std::rc::Rc::new(std::cell::RefCell::new(Snowflake::new(epoch, worker_id)?)),
            })
        }

        pub fn generate_id(&self) -> Result<i64> {
            let mut guard = self.inner.borrow_mut();
            Ok(guard.generate_id())
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::SnowflakeState as Snowflake;
    #[test]
    fn test_snowflake_id_generation() {
        let mut snowflake = Snowflake::new(0, 1).unwrap();
        let id1 = snowflake.generate_id();
        let id2 = snowflake.generate_id();
        assert!(id2 > id1);
    }

    #[test]
    fn bench_snowflake_id_generation() {
        let snowflake = sync_generator::SnowflakeGenerator::new(0, 1).unwrap();
        let ids = 1_000_000;
        let time = std::time::Instant::now();
        for _ in 0..ids {
            snowflake.generate_id().unwrap();
        }
        let elapsed = time.elapsed();
        println!(
            "Generated {} IDs in {:?} ({:.2} IDs/sec)",
            ids,
            elapsed,
            ids as f64 / elapsed.as_secs_f64()
        );
    }
}