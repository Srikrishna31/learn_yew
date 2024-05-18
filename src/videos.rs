use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub(crate) struct Video {
    pub id: usize,
    pub title: String,
    pub speaker: String,
    pub url: String,
}

impl Video {
    pub(crate) fn videos() -> Vec<Video> {
        vec![
            Video {
                id: 1,
                title: "Building and breaking things with Rust".to_string(),
                speaker: "John Doe".to_string(),
                url: "https://www.youtu.be/PasFVLr8t4E".to_string(),
            },
            Video {
                id: 2,
                title: "The development process".to_string(),
                speaker: "Jane Smith".to_string(),
                url: "https://www.youtu.be/PasFVLr8t4E".to_string(),
            },
            Video {
                id: 3,
                title: "The Web 7.0".to_string(),
                speaker: "Matt Miller".to_string(),
                url: "https://www.youtu.be/PasFVLr8t4E".to_string(),
            },
            Video {
                id: 4,
                title: "Mouseless development".to_string(),
                speaker: "Tom Jerry".to_string(),
                url: "https://www.youtu.be/PasFVLr8t4E".to_string(),
            },
        ]
    }
}

#[derive(Properties, PartialEq)]
pub struct VideosListProps {
    pub videos: Vec<Video>,
    pub on_click: Callback<Video>,
}

/// A function component takes only one argument which defines its "props"(short for "properties").
/// Props are used to pass data down from a parent component to a child component. In this case,
/// the `VideosListProps` struct that defines the props.
#[function_component(VideosList)]
pub fn videos_list(VideosListProps {videos, on_click}: &VideosListProps) -> Html {
    let on_click = on_click.clone();

    videos.iter().map(|video| {
        let on_video_select = {
            let on_click = on_click.clone();
            let video = video.clone();
            Callback::from(move |_| on_click.emit(video.clone()))
        };

        html!{
            <p key={video.id} onclick={on_video_select}>
                {format!("{}: {}", video.speaker, video.title)}</p>
        }
    }).collect()
}


#[derive(Properties, PartialEq)]
pub struct VideosDetailsProps {
    pub video: Video,
}

#[function_component(VideoDetails)]
pub fn videos_details(VideosDetailsProps {video}: &VideosDetailsProps) -> Html {
    html! {
        <div>
            <h3>{video.title.clone()}</h3>
            <img src="https://via.placeholder.com/640x360.png?text=Video+Player+Placeholder" alt="video thumbnail" />
        </div>
    }
}