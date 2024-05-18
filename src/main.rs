mod videos;

use yew::prelude::*;
use videos::{Video, VideosList, VideoDetails};


/// # Components
/// Components are the building blocks of Yew applications. By combining components, which can be
/// made of other components, we build our application. By structuring our components for re-usability
/// and keeping them generic, we will be able to use them in multiple parts of our application without
/// having to duplicate code or logic.
/// There are two different types of components in Yew: function components and class/struct components.
#[function_component(App)]
fn app() -> Html {
    let selected_video = use_state(|| None);
    let on_video_select = {
        let selected_video = selected_video.clone();
        Callback::from(move |video: Video| {
            selected_video.set(Some(video));
        })
    };
    let details = selected_video.as_ref().map(|video| html! {
        <VideoDetails video={video.clone()} />
    });

    html! {
        <>
            <h1>{ "RustConf Explorer" }</h1>
            <div>
                <h3>{"Videos to watch"}</h3>
                <VideosList videos={Video::videos()} on_click={on_video_select.clone()} />
            </div>
            {for details}
        </>
    }
}
fn main() {
    yew::Renderer::<App>::new().render();
}
