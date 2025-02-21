use crate::*;

impl ControllerData {
    #[inline]
    pub fn new() -> Self {
        ControllerData {
            stream: None,
            request: Request::default(),
            response: Response::default(),
            log: Log::default(),
        }
    }
}

impl ArcRwLockControllerData {
    #[inline]
    pub async fn get_read_lock(&self) -> RwLockReadControllerData {
        let controller_data: RwLockReadControllerData = self.0.read().await;
        controller_data
    }

    #[inline]
    pub async fn get_write_lock(&self) -> RwLockWriteControllerData {
        let controller_data: RwLockWriteControllerData = self.0.write().await;
        controller_data
    }

    #[inline]
    pub async fn get_clone(&self) -> ControllerData {
        let controller_data: ControllerData = self.get_read_lock().await.clone();
        controller_data
    }

    #[inline]
    pub async fn get_stream(&self) -> OptionArcRwLockStream {
        let controller_data: ControllerData = self.get_clone().await;
        controller_data.get_stream().clone()
    }

    #[inline]
    pub async fn get_request(&self) -> Request {
        let controller_data: ControllerData = self.get_clone().await;
        controller_data.get_request().clone()
    }

    #[inline]
    pub async fn get_response(&self) -> Response {
        let controller_data: ControllerData = self.get_clone().await;
        controller_data.get_response().clone()
    }

    #[inline]
    pub async fn get_log(&self) -> Log {
        let controller_data: ControllerData = self.get_clone().await;
        controller_data.get_log().clone()
    }

    #[inline]
    pub async fn get_socket_addr(&self) -> Option<String> {
        let stream_result: OptionArcRwLockStream = self.get_stream().await;
        if stream_result.is_none() {
            return None;
        }
        let socket_addr: String = stream_result
            .unwrap()
            .read()
            .await
            .peer_addr()
            .and_then(|host| Ok(host.to_string()))
            .unwrap_or_default();
        Some(socket_addr)
    }

    #[inline]
    pub async fn send_response<T: Into<ResponseBody>>(
        &self,
        status_code: usize,
        response_body: T,
    ) -> ResponseResult {
        let controller_data: RwLockWriteControllerData = self.get_write_lock().await;
        let mut response: Response = controller_data.get_response().clone();
        let body: ResponseBody = response_body.into();
        let stream_lock: ArcRwLockStream = controller_data.get_stream().clone().unwrap();
        let response_res: ResponseResult = response
            .set_body(body)
            .set_status_code(status_code)
            .send(&stream_lock)
            .await;
        response_res
    }

    #[inline]
    pub async fn send_response_once<T: Into<ResponseBody>>(
        &self,
        status_code: usize,
        response_body: T,
    ) -> ResponseResult {
        let controller_data: RwLockWriteControllerData = self.get_write_lock().await;
        let mut response: Response = controller_data.get_response().clone();
        let body: ResponseBody = response_body.into();
        let stream_lock: ArcRwLockStream = controller_data.get_stream().clone().unwrap();
        let response_res: ResponseResult = response
            .set_body(body)
            .set_status_code(status_code)
            .send(&stream_lock)
            .await;
        let _ = response.close(&stream_lock).await;
        response_res
    }
}
