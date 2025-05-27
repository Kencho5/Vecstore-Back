use crate::prelude::*;

pub fn load_http_client() -> Client {
    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_static("api-key"),
        HeaderValue::from_static(
            "Dp5kaOKNI3gR1J6UQm16J0minp7sSML4NIAHFyqdDCO2aj9j0mO67KMbeBcMikP2",
        ),
    );

    let http_client = Client::builder().default_headers(headers).build().unwrap();

    http_client
}
