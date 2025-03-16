use crate::*;

impl<'a> Default for ServerConfig<'a> {
    #[inline]
    fn default() -> Self {
        Self {
            host: DEFAULT_HOST,
            port: DEFAULT_WEB_PORT,
            log_dir: DEFAULT_LOG_DIR,
            log_size: DEFAULT_LOG_FILE_SIZE,
            interval_millis: DEFAULT_LOG_INTERVAL_MILLIS,
            inner_print: DEFAULT_INNER_PRINT,
            inner_log: DEFAULT_INNER_LOG,
            websocket_buffer_size: DEFAULT_BUFFER_SIZE,
        }
    }
}
