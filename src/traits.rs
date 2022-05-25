use std::sync::Arc;

use dominator::Dom;

pub trait Component {
    fn render(c: Arc<Self>) -> Dom;
}
