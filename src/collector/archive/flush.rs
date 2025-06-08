use crate::collector::archive::kline_buffer::FlushableBuffer;
use crate::collector::archive::sink::{ClickhouseSink, KlineSink, MysqlSink};

pub async fn flush_all<B: FlushableBuffer>(buffer: &B) -> Result<(), anyhow::Error> {
    // Forward -> MySQL
    if buffer.should_flush_forward().await {
        let data = buffer.drain_forward().await;
        MysqlSink.write(data).await?;
    }

    // Backward -> ClickHouse
    if buffer.should_flush_backward().await {
        let data = buffer.drain_backward().await;
        ClickhouseSink.write(data).await?;
    }

    Ok(())
}
