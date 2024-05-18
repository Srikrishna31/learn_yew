mod videos;

use yew::prelude::*;
use videos::{Video, VideosList};


/// # Components
/// Components are the building blocks of Yew applications. By combining components, which can be
/// made of other components, we build our application. By structuring our components for re-usability
/// and keeping them generic, we will be able to use them in multiple parts of our application without
/// having to duplicate code or logic.
/// There are two different types of components in Yew: function components and class/struct components.
#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <h1>{ "RustConf Explorer" }</h1>
            <div>
                <h3>{"Videos to watch"}</h3>
                <VideosList videos={Video::videos()} />
            </div>
            <div>
                <h3>{"John Doe: Building and breaking things"}</h3>
                <img src="https://via.placeholder.com/640x360.png?text=Video+Player+Placeholder" alt="video thumbnail" />
            </div>
        </>
    }
}
fn main() {
    yew::Renderer::<App>::new().render();
}
