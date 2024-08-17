mod data;

use leptos::*;
use leptos_icons::*;
use icondata as i;
use thaw::{DatePicker};
use chrono::prelude::*;
use chrono::Duration;
use crate::data::{get_hotels, Hotel, Place};

fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(|| view! { <App/> })
}

fn get_days(from_date: Option<NaiveDate>, to_date: Option<NaiveDate>) -> i64 {
    if from_date.is_none() || to_date.is_none() {
        return -1;
    }

    (to_date.unwrap() - from_date.unwrap()).num_days()
}

#[component]
fn App() -> impl IntoView {
    let hotels = get_hotels();
    let from_date = create_rw_signal(Some(Local::now().date_naive()));
    let to_date = create_rw_signal(Some((Local::now() + Duration::days(5)).date_naive()));
    let daysDelta = move || get_days(from_date.get(), to_date.get());

    view! {
        <nav class="flex flex-row justify-center my-4">
            <h3 class="font-bold text-2xl">Hotel Booking</h3>
        </nav>
        <main>
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
                .map(|hotel| view! { <HotelCard hotel=hotel daysDelta=daysDelta /> })
                .collect_view()}
            </div>
        </main>
    }
}

#[component]
fn HotelCard(hotel: Hotel, daysDelta: impl Fn() -> i64 + 'static) -> impl IntoView {
    let place = match hotel.place {
        Place::Townhouse => "Townhouse".to_owned(),
        Place::Apartment => "Apartment".to_owned(),
        Place::Other => "Place to stay".to_owned(),
    };

    let totalPrice = move || {
        let total = daysDelta() * i64::from(hotel.price);
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
            <p class="text-gray-400">{&hotel.name}</p>
            <p class="text-gray-400">{hotel.beds} beds</p>
            <span>
                <strong>${hotel.price} night</strong>
                " \u{00B7} "
                <span class="text-gray-400 underline">${totalPrice} total</span>
            </span>
        </div>
    }
}

// #[component]
// fn ProgressBar(
//     #[prop(default = 100)]
//     max: u16,
//     #[prop(into)]
//     progress: Signal<i32>
// ) -> impl IntoView
// {
//     view! {
//         <progress
//             max=max
//             value=progress
//         />
//         <br/>
//     }
// }
//
// #[component]
// fn App() -> impl IntoView {
//     let styler_class = style! { "App",
//         .red {
//             color: red;
//         }
//     };
//
//     let (count, set_count) = create_signal(0);
//     let double_count = move || count() * 2;
//
//     view! { class = styler_class,
//         <button on:click=move |_| { set_count.update(|n| *n += 1); } class="red">
//             "Click me"
//         </button>
//         // .into() converts `ReadSignal` to `Signal`
//         <ProgressBar progress=count/>
//         // use `Signal::derive()` to wrap a derived signal
//         <ProgressBar progress=Signal::derive(double_count)/>
//     }
// }