#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    AudioBook { title: String },
    Podcast(u32),
    Placeholder,
}

impl Media {
    fn description(&self) -> String {
        match self {
            Media::Book { title, author } => format!("Book: {} by {}", title, author),
            Media::Movie { title, director } => format!("Movie: {} by {}", title, director),
            Media::AudioBook { title } => format!("AudioBook: {}", title),
            Media::Podcast(episode_number) => format!("Podcast: {}", episode_number),
            Media::Placeholder => format!("Placeholder"),
        }
    }
}

#[derive(Debug)]
struct Catalog {
    items: Vec<Media>,
}

impl Catalog {
    fn new() -> Self {
        Catalog { items: vec![] }
    }

    fn add(&mut self, media: Media) {
        self.items.push(media);
    }

    // with custom get by index with custom error handling
    fn get_by_index(&self, index: usize) -> MaybeValue {
        // check validity of index
        if self.items.len() > index {
            MaybeValue::Value(&self.items[index])
        } else {
            MaybeValue::None
        }
    }

    fn get_by_index_with_option(&self, index: usize) -> Option<&Media> {
        if self.items.len() > index {
            Some(&self.items[index])
        } else {
            None
        }
    }
}

enum MaybeValue<'a> {
    Value(&'a Media),
    None,
}

fn print_media(media: Media) {
    print!("{:#?}", media);
}

fn main() {
    let audio_book = Media::AudioBook {
        title: String::from("The History of the English Language"),
    };

    let my_favorite_movie = Media::Movie {
        title: String::from("Inception"),
        director: String::from("Christopher Nolan"),
    };

    let my_favorite_book = Media::Book {
        title: String::from("The Rust Programming Language"),
        author: String::from("Steve Klabnik"),
    };

    let podcast = Media::Podcast(1);
    let placeholder = Media::Placeholder;

    // print!("{}", audio_book.description());
    // print!("{}", audio_book.description());
    // print!("{}", audio_book.description());

    let mut catelog = Catalog::new();
    catelog.add(audio_book);
    catelog.add(my_favorite_movie);
    catelog.add(my_favorite_book);
    catelog.add(podcast);
    catelog.add(placeholder);

    // match catelog.items.get(20) {
    //     Some(media) => println!("Item: {:#?}", media),
    //     None => println!("No media found"),
    // }

    match catelog.get_by_index(0) {
        MaybeValue::Value(media) => println!("Item: {:#?}", media),
        MaybeValue::None => println!("No media found"),
    }

    match catelog.get_by_index_with_option(10) {
        Some(media) => println!("Item: {:#?}", media),
        None => println!("No media found"),
    }
}
