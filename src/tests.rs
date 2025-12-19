use crate::single_thread::sync_generator::SnowflakeGenerator as STSG;
use crate::multi_thread::sync_generator::SnowflakeGenerator as MTSG;
use crate::multi_thread::async_generator::SnowflakeGenerator as MTAG;

#[test]
fn test_single_thread_snowflake_id_generation() {
    let generator = STSG::new(0, 1).unwrap();
    let id1 = generator.generate_id();
    let id2 = generator.generate_id();
    let decomposed1 = generator.decompose(id1);
    let decomposed2 = generator.decompose(id2);
    println!("ID1: {}, ID2: {}", id1, id2);
    println!("Decomposed1: {:?}, Decomposed2: {:?}", decomposed1, decomposed2);
    assert!(id2 > id1);
}

#[tokio::test]
async fn test_multi_thread_async_snowflake_id_generation() {
    let generator = MTAG::new(0, 1).unwrap();

    let handle1 = {
        let gen_clone = generator.clone();
        tokio::spawn(async move {
            gen_clone.generate_id().await
        })
    };

    let handle2 = {
        let gen_clone = generator.clone();
        tokio::spawn(async move {
            gen_clone.generate_id().await
        })
    };
    let id1 = handle1.await.unwrap();
    let id2 = handle2.await.unwrap();
    let decomposed1 = generator.decompose(id1).await;
    let decomposed2 = generator.decompose(id2).await;

    println!("ID1: {}, ID2: {}", id1, id2);
    println!("Decomposed1: {:?}, Decomposed2: {:?}", decomposed1, decomposed2);
    assert!(id2 != id1);
}

#[test]
fn test_multi_thread_sync_snowflake_id_generation() {
    use std::thread;
    let generator = MTSG::new(0, 1).unwrap();

    let handle1 = {
        let gen_clone = generator.clone();
        thread::spawn(move || {
            gen_clone.generate_id()
        })
    };

    let handle2 = {
        let gen_clone = generator.clone();
        thread::spawn(move || {
            gen_clone.generate_id()
        })
    };
    let id1 = handle1.join().unwrap();
    let id2 = handle2.join().unwrap();
    let decomposed1 = generator.decompose(id1);
    let decomposed2 = generator.decompose(id2);

    println!("ID1: {}, ID2: {}", id1, id2);
    println!("Decomposed1: {:?}, Decomposed2: {:?}", decomposed1, decomposed2);

    assert!(id2 != id1);
}

#[test]
fn bench_single_thread_snowflake_id_generation() {
    let generator = STSG::new(0, 1).unwrap();
    let ids = 1_000_000;
    let time = std::time::Instant::now();
    for _ in 0..ids {
        generator.generate_id();
    }
    let elapsed = time.elapsed();
    println!("Generated {} IDs in {:?}", ids, elapsed);
    println!("({:.2} IDs/ms)", ids as f64 / elapsed.as_millis() as f64);
}

#[tokio::test]
async fn bench_multi_thread_async_snowflake_id_generation() {
    use std::thread;
    let generator = MTAG::new(0, 1).unwrap();
    let ids_per_thread = 100_000;
    let threads = 10;
    let time = std::time::Instant::now();
    let mut handles = vec![];
    for _ in 0..threads {
        let gen_clone = generator.clone();
        let handle = thread::spawn(async move || {
            for _ in 0..ids_per_thread {
                gen_clone.generate_id().await;
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap().await;
    }
    let elapsed = time.elapsed();
    println!("Generated {} IDs in {:?}", ids_per_thread * threads, elapsed);
    println!("({:.2} IDs/ms)", (ids_per_thread * threads) as f64 / elapsed.as_millis() as f64);
}

#[test]
fn bench_multi_thread_sync_snowflake_id_generation() {
    use std::thread;
    let generator = MTSG::new(0, 1).unwrap();
    let ids_per_thread = 100_000;
    let threads = 10;
    let time = std::time::Instant::now();
    let mut handles = vec![];
    for _ in 0..threads {
        let gen_clone = generator.clone();
        let handle = thread::spawn(move || {
            for _ in 0..ids_per_thread {
                gen_clone.generate_id();
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    let elapsed = time.elapsed();
    println!("Generated {} IDs in {:?}", ids_per_thread * threads, elapsed);
    println!("({:.2} IDs/ms)", (ids_per_thread * threads) as f64 / elapsed.as_millis() as f64);
}