use leptos::*;
use leptos_router::*;
use leptos_icons::*;
use icondata as i;
use thaw::{DatePicker};
use chrono::prelude::*;
use chrono::{Duration, NaiveDate};
use crate::data::{get_hotels, Hotel, Place};

fn get_days(from_date: Option<NaiveDate>, to_date: Option<NaiveDate>) -> i64 {
    if from_date.is_none() || to_date.is_none() {
        return -1;
    }

    (to_date.unwrap() - from_date.unwrap()).num_days()
}

#[component]
pub(crate) fn HomePage() -> impl IntoView {
    let hotels = get_hotels();
    let from_date = create_rw_signal(Some(Local::now().date_naive()));
    let to_date = create_rw_signal(Some((Local::now() + Duration::days(5)).date_naive()));
    let days_delta = move || get_days(from_date.get(), to_date.get());

    view! {
        <div class="flex flex-row justify-center gap-4">
            <div>
                From: <DatePicker value={from_date} />
            </div>
            <div>
                To: <DatePicker value={to_date} />
            </div>
        </div>
        <div class="grid grid-cols-2 gap-4 mx-4 my-4">
        {hotels.into_iter()
            .map(|hotel| view! { <HotelCard hotel=hotel days_delta=days_delta /> })
            .collect_view()}
        </div>
    }
}

#[component]
fn HotelCard(hotel: Hotel, days_delta: impl Fn() -> i64 + 'static) -> impl IntoView {
    let place = match hotel.place {
        Place::Townhouse => "Townhouse".to_owned(),
        Place::Apartment => "Apartment".to_owned(),
        Place::Other => "Place to stay".to_owned(),
    };

    let totalPrice = move || {
        let total = days_delta() * i64::from(hotel.price);
        if total >= 0 {
            total.to_string()
        } else {
            "--".to_owned()
        }
    };

    view! {
        <div class="flex flex-col">
            <img class="object-cover w-full h-full rounded-xl" src={hotel.image_url} alt={format!("image of {}", hotel.name)} />
            <div class="flex flex-row justify-between mt-2">
                <strong>{place} in {&hotel.city}</strong>
                <div class="flex flex-row items-center">
                    <Icon icon=i::AiStarFilled />{hotel.rating}
                    {format!("  ({})", hotel.review_count)}
                </div>
            </div>
            <A href={format!("hotel/{}", &hotel.id)}><p class="text-gray-400">{&hotel.name}</p></A>
            <p class="text-gray-400">{hotel.beds} beds</p>
            <span>
                <strong>${hotel.price} night</strong>
                " \u{00B7} "
                <span class="text-gray-400 underline">${totalPrice} total</span>
            </span>
        </div>
    }
}