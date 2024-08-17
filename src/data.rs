
pub(crate) fn get_hotels() -> Vec<Hotel> {
    vec![
        Hotel {
            image_url: String::from("https://a0.muscache.com/im/pictures/49cce853-d7f4-4571-9808-c2bc8c806728.jpg?im_w=960"),
            name: String::from("Tokyo Kids Castle"),
            beds: 7,
            rating: 5.0,
            review_count: 109,
            city: String::from("Nakano City"),
            price: 645,
            place: Place::Other,
        },
        Hotel {
            image_url: String::from("https://a0.muscache.com/im/pictures/7ab6a8ba-9745-4d31-8d9e-44ffc8ef3eae.jpg?im_w=960"),
            name: String::from("Kyomachiya Building Rental Inn"),
            city: String::from("Kyoto"),
            price: 144,
            beds: 8,
            rating: 4.97,
            review_count: 145,
            place: Place::Townhouse,
        }
    ]
}

pub(crate) struct Hotel {
    pub name: String,
    pub city: String,
    pub price: i32,
    pub image_url: String,
    pub beds: i32,
    pub rating: f32,
    pub review_count: i32,
    pub place: Place,
}

pub(crate) enum Place {
    Townhouse,
    Apartment,
    Other
}