extern crate cfg_if;
extern crate hsbc_parser;
extern crate wasm_bindgen;
extern crate web_sys;

mod utils;

use hsbc_parser::parser::parse;
use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
extern "C" {
    pub type Node;
}

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

// #[wasm_bindgen]
// pub fn greet() {
//     alert("Hello, bank-statement-reader!");
// }

// #[wasm_bindgen]
// pub fn get_balance(s: &str) -> String {
//     let statement = parse(s);
//     return statement.balance();
// }

#[wasm_bindgen]
pub fn parse_statement(s: &str) {
    utils::set_panic_hook();
    // console_error_panic_hook::set_once();

    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let parent_node = document
        .query_selector("bank-statement")
        .expect("should have bank-statement element")
        .unwrap();
    clean_children(&parent_node.as_ref() as &web_sys::Node);

    log!("{:?}", s);
    let statement = parse(s);

    //console::log_2(&"parsed".into(), &"parsed".into());

    let balance_text = document.create_text_node(&statement.balance());

    let p = document.create_element("p");

    // Right now the class inheritance hierarchy of the DOM isn't super
    // ergonomic, so we manually cast `val: Element` to `&Node` to call the
    // `append_child` method.
    match AsRef::<web_sys::Node>::as_ref(p.as_ref().unwrap()).append_child(balance_text.as_ref()) {
        Err(e) => console::log_1(&e),
        _ => (),
    }

    match AsRef::<web_sys::Node>::as_ref(&parent_node)
        .append_child(AsRef::<web_sys::Node>::as_ref(p.as_ref().unwrap()))
    {
        Err(e) => console::log_1(&e),
        _ => (),
    }

    let ul = create_ul(&document);
    let ref ul = ul.as_ref() as &web_sys::Node;

    let mut movements_string = String::from("");

    for mov in statement.movements() {
        let desc = &mov.description.clone();
        movements_string.push_str(&desc);
        movements_string.push('\n');

        let li = create_li(&document);
        let ref li = li.as_ref() as &web_sys::Node;

        let text_container = create_span_text(&document);
        let ref text_container = text_container.as_ref() as &web_sys::Node;

        li.append_child(text_container).unwrap();

        let span = create_span_pri_text(&document);
        let ref span = span.as_ref() as &web_sys::Node;
        text_container.append_child(span).unwrap();

        let text_mov = create_text(&document, &mov.description);
        let ref text_mov = text_mov.as_ref() as &web_sys::Node;
        span.append_child(text_mov).unwrap();

        let span = create_span_sec_text(&document);
        let ref span = span.as_ref() as &web_sys::Node;
        text_container.append_child(span).unwrap();

        let text_date = create_text(&document, &mov.date.format("%Y-%m-%d %H:%M:%S").to_string());
        let ref text_date = text_date.as_ref() as &web_sys::Node;
        span.append_child(text_date).unwrap();

        let span = create_span_meta(&document);
        let ref span = span.as_ref() as &web_sys::Node;
        li.append_child(span).unwrap();

        let amount = create_text(&document, &mov.amount());
        let ref amount = amount.as_ref() as &web_sys::Node;
        span.append_child(amount).unwrap();

        ul.append_child(li).unwrap();
    }

    // console::log_1(&movements_string.into());

    let pre = create_pre(&document);
    let ref pre = pre.as_ref() as &web_sys::Node;

    let text_mov = create_text(&document, &movements_string);
    let ref text_mov = text_mov.as_ref() as &web_sys::Node;

    pre.append_child(text_mov).unwrap();

    match AsRef::<web_sys::Node>::as_ref(&parent_node).append_child(pre) {
        Err(e) => console::log_1(&e),
        _ => (),
    }

    match AsRef::<web_sys::Node>::as_ref(&parent_node).append_child(ul) {
        Err(e) => console::log_1(&e),
        _ => (),
    }

    //console::log_2(&"Parent node".into(), &parent_node.into());
}

fn clean_children(node: &web_sys::Node) {
    while node.has_child_nodes() {
        node.remove_child(&node.first_child().unwrap()).unwrap();
    }
}

fn create_ul(document: &web_sys::Document) -> web_sys::Element {
    let element = document.create_element("ul").unwrap();
    element
        .set_attribute("class", "mdc-list mdc-list--two-line")
        .unwrap();
    element
        .set_attribute("aria-orientation", "vertical")
        .unwrap();
    element
}

fn create_li(document: &web_sys::Document) -> web_sys::Element {
    let element = document.create_element("li").unwrap();
    element.set_attribute("class", "mdc-list-item").unwrap();
    element
}

fn create_text(document: &web_sys::Document, text: &str) -> web_sys::Text {
    let element = document.create_text_node(text);
    element
}

fn create_span_text(document: &web_sys::Document) -> web_sys::Element {
    let element = document.create_element("span").unwrap();
    element
        .set_attribute("class", "mdc-list-item__text")
        .unwrap();
    element
}

fn create_span_pri_text(document: &web_sys::Document) -> web_sys::Element {
    let element = document.create_element("span").unwrap();
    element
        .set_attribute("class", "mdc-list-item__primary-text")
        .unwrap();
    element
}

fn create_span_sec_text(document: &web_sys::Document) -> web_sys::Element {
    let element = document.create_element("span").unwrap();
    element
        .set_attribute("class", "mdc-list-item__secondary-text")
        .unwrap();
    element
}

fn create_span_meta(document: &web_sys::Document) -> web_sys::Element {
    let element = document.create_element("span").unwrap();
    element
        .set_attribute("class", "mdc-list-item__meta")
        .unwrap();
    element
}

fn create_pre(document: &web_sys::Document) -> web_sys::Element {
    let element = document.create_element("pre").unwrap();
    element
}
