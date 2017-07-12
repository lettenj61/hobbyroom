#![js_native(global = "$")]

#[js_native_fn]
pub fn select<S: Selector>(selector: S) -> JQuery {
    js_native!();
}

#[js_native_fn]
pub fn select_from_context<S: Selector>(selector: S, context: Selected) -> JQuery {
    js_native!();
}

js_marker_trait! { Selector =>
    String,
    <T> Array<T>,
}

js_marker_trait! { Selected =>
    Element,
}

#[cfg(test)]
fn usage_example() {

    use super::*;

    let _sel = select("#app"); // $('#app')
    let _sel = select_from_context("div[name='goodies']", select("#wrapper"));
    // => $('div[name="goodies"]', $('#wrapper'))
}

