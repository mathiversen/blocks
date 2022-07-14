mod app;

use app::App;

pub fn main() {
    console_error_panic_hook::set_once();
    dominator::append_dom(&dominator::body(), App::render(App::new()));
}
