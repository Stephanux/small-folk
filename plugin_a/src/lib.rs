use tide::{Request, Response};
use core::State;
struct PluginA;

impl core::Plugin for PluginA {
    fn callback1(&self) {
        println!("PluginA::callback1")
    }

    fn callback2(&self, i: i32) -> i32 {
        println!("PluginA::callback2");
        i + 1
    }

    fn execute(&self, _req: &Request<State>) -> Result<Response, tide::Error> {
        // NOTE: This is a synchronous stub. You must adapt async code to sync or use a runtime.
        // Here, we just return a dummy response for trait compatibility.
        let mut res = Response::new(200);
        res.set_content_type("text/html");
        res.set_body("<html><body>From execute de plugin_a</body></html>");
        println!("PluginA::execute called with request: {:?}", _req);
        Ok(res)
    }
}

#[no_mangle]
pub fn plugin_entry(registrar: &mut dyn core::PluginRegistrar) {
    registrar.register_plugin(Box::new(PluginA));
}
