use super::media::Media;

#[derive(Debug)]
pub struct Catelog {
    items: Vec<Media>,
}

enum MaybeValue<'a> {
    Value(&'a Media),
    None,
}

impl Catelog {
    pub fn new() -> Self {
        Catelog { items: vec![] }
    }

    pub fn add(&mut self, media: Media) {
        self.items.push(media);
    }

    // with custom get by index with custom error handling
    pub fn get_by_index(&self, index: usize) -> MaybeValue {
        // check validity of index
        if self.items.len() > index {
            MaybeValue::Value(&self.items[index])
        } else {
            MaybeValue::None
        }
    }

    pub fn get_by_index_with_option(&self, index: usize) -> Option<&Media> {
        if self.items.len() > index {
            Some(&self.items[index])
        } else {
            None
        }
    }
}
