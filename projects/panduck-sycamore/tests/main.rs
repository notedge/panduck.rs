use sycamore::{generic_node::ssr_node::render_to_string, view};

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn renderer() {
    let node = view! {
    div(class="my-class") {
        button { "Click me" }
    }
    };
    let html = render_to_string(|| node);

    // Prints: <div class="my-class"><button>Click me</button></div>
    println!("{}", html);
}
