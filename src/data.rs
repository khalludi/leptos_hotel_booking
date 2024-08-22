
pub(crate) fn get_hotel(id: usize) -> Option<Hotel> {
    get_hotels().iter().find(|hotel| hotel.id == id).cloned()
}

pub(crate) fn get_hotels() -> Vec<Hotel> {
    vec![
        Hotel {
            id: 12,
            image_url: String::from("https://a0.muscache.com/im/pictures/49cce853-d7f4-4571-9808-c2bc8c806728.jpg?im_w=960"),
            name: String::from("Tokyo Kids Castle"),
            guests: 14,
            bedrooms: 4,
            baths: 1.5,
            beds: 7,
            rating: 5.0,
            review_count: 109,
            city: String::from("Nakano City"),
            price: 339,
            place: Place::Other,
        },
        Hotel {
            id: 16,
            image_url: String::from("https://a0.muscache.com/im/pictures/7ab6a8ba-9745-4d31-8d9e-44ffc8ef3eae.jpg?im_w=960"),
            name: String::from("Kyomachiya Building Rental Inn"),
            city: String::from("Kyoto"),
            price: 144,
            guests: 8,
            bedrooms: 2,
            baths: 1.0,
            beds: 8,
            rating: 4.97,
            review_count: 145,
            place: Place::Townhouse,
        }
    ]
}

#[derive(Clone)]
pub(crate) struct Hotel {
    pub id: usize,
    pub name: String,
    pub city: String,
    pub price: i32,
    pub image_url: String,
    pub guests: i32,
    pub bedrooms: i32,
    pub baths: f32,
    pub beds: i32,
    pub rating: f32,
    pub review_count: i32,
    pub place: Place,
}

#[derive(Clone)]
pub(crate) enum Place {
    Townhouse,
    Apartment,
    Other
}