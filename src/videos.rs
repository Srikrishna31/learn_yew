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