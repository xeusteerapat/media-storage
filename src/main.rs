mod content;
use content::catelog::Catelog;
use content::media::Media;

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

    let mut catelog = Catelog::new();
    catelog.add(audio_book);
    catelog.add(my_favorite_movie);
    catelog.add(my_favorite_book);
    catelog.add(podcast);
    catelog.add(placeholder);

    // match catelog.items.get(20) {
    //     Some(media) => println!("Item: {:#?}", media),
    //     None => println!("No media found"),
    // }

    // match catelog.get_by_index(0) {
    //     MaybeValue::Value(media) => println!("Item: {:#?}", media),
    //     MaybeValue::None => println!("No media found"),
    // }

    // match catelog.get_by_index_with_option(10) {
    //     Some(media) => println!("Item: {:#?}", media),
    //     None => println!("No media found"),
    // }

    // using unwrap
    let media = catelog.get_by_index_with_option(0);
    let media_placeholder = Media::Placeholder;
    println!("Item: {:#?}", media.unwrap_or(&media_placeholder));
}
