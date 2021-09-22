use proxy_wasm::traits::*;
use proxy_wasm::types::*;

#[no_mangle]
pub fn _start() {
    proxy_wasm::set_http_context(|_, _| -> Box<dyn HttpContext> { Box::new(HttpHeadersRoot) });
}

struct HttpHeadersRoot;
impl Context for HttpHeadersRoot {}


impl HttpContext for HttpHeadersRoot {
    fn on_http_response_headers(&mut self, _: usize) -> Action {
        self.set_http_response_header("BTC", Some("ETH"));
        Action::Continue
    }
}