#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SnowflakeDecomposed {
    /// The timestamp component of the Snowflake ID in milliseconds since epoch.
    pub timestamp: i64,
    /// The worker ID component of the Snowflake ID.
    pub worker_id: u16,
    /// The sequence number component of the Snowflake ID.
    pub sequence: u16,
}

pub fn decompose_snowflake(id: i64, epoch: i64) -> Result<SnowflakeDecomposed> {
    if id < 0 {
        return Err(SnowflakeDecomposeError::SignBitError);
    }

    let timestamp = (id >> 22) + epoch;
    let worker_id = ((id >> 12) & 0x3FF) as u16;
    let sequence = (id & 0xFFF) as u16;

    Ok(SnowflakeDecomposed {
        timestamp,
        worker_id,
        sequence,
    })
}

#[derive(Debug)]
pub enum SnowflakeDecomposeError {
    /// Sign bit error in the Snowflake ID.
    SignBitError
}

impl std::fmt::Display for SnowflakeDecomposeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SnowflakeDecomposeError::SignBitError => write!(f, "Sign bit error in the Snowflake ID"),
        }
    }
}

impl std::error::Error for SnowflakeDecomposeError {}

pub type Result<T> = std::result::Result<T, SnowflakeDecomposeError>;