#[derive(Debug)]
pub enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    AudioBook { title: String },
    Podcast(u32),
    Placeholder,
}

impl Media {
    pub fn description(&self) -> String {
        match self {
            Media::Book { title, author } => format!("Book: {} by {}", title, author),
            Media::Movie { title, director } => format!("Movie: {} by {}", title, director),
            Media::AudioBook { title } => format!("AudioBook: {}", title),
            Media::Podcast(episode_number) => format!("Podcast: {}", episode_number),
            Media::Placeholder => format!("Placeholder"),
        }
    }
}
