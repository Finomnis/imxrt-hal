use super::{ral, LpspiReadPart};

impl<const N: u8> LpspiReadPart<'_, N> {
    fn fifo_read_data_available(&self) -> bool {
        ral::read_reg!(
            ral::lpspi,
            self.data.lpspi.instance(),
            RSR,
            RXEMPTY == RXEMPTY_0
        )
    }

    async fn wait_for_read_watermark(&self, watermark: u32) {
        self.data
            .lpspi
            .wait_for_rx_watermark(watermark)
            .await
            .unwrap();
    }

    async fn wait_for_read_data_available(&mut self, at_most: usize) {
        if !self.fifo_read_data_available() {
            let mut watermark = self.rx_fifo_size / 2;
            if let Ok(at_most) = u32::try_from(at_most) {
                watermark = watermark.min(at_most);
            }
            self.wait_for_read_watermark(watermark).await;
        }
    }
}
