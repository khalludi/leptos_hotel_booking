use chrono::{Duration, Local};
use leptos::*;
use leptos_router::*;
use crate::data::{get_hotel, Hotel, Place};

#[derive(Params, PartialEq)]
struct HotelParams {
    id: Option<usize>,
}

#[component]
pub(crate) fn HotelPage() -> impl IntoView {
    let params = use_params::<HotelParams>();

    let id = move || {
        params.with(|params| {
            params.as_ref()
                .map(|params| params.id)
                .unwrap_or_default()
        })
    };
    let hotel = move || {
        get_hotel(id().unwrap())
    };

    view! {
        <Show
            when=move || hotel().is_some()
            fallback=|| view! { "Hotel not found" }
        >
            <HotelComponent hotel=Signal::derive(move || hotel().unwrap()) />
        </Show>
    }
}

#[component]
pub(crate) fn HotelComponent(
    #[prop(into)]
    hotel: Signal<Hotel>
) -> impl IntoView {
    let h = hotel.get();
    let place = match &h.place {
        Place::Townhouse => "townhouse".to_owned(),
        Place::Apartment => "apartment".to_owned(),
        Place::Other => "place".to_owned(),
    };

    view! {
        <div class="m-10 flex flex-col gap-3">
            <h1 class="text-2xl font-bold">{&h.name}</h1>
            <img class="object-cover w-full h-full rounded-xl"
                src={&h.image_url}
                alt={format!("image of {}", &h.name)} />
            <div class="grid grid-cols-5 gap-4">
                <div class="col-span-3">
                    <h2 class="font-bold text-xl">Entire {place} in {&h.city}, Japan</h2>
                    <p>
                        {h.guests}" guests \u{00B7} "
                        {h.bedrooms}" bedrooms \u{00B7} "
                        {h.beds}" beds \u{00B7} "
                        {h.baths}" baths"</p>
                </div>
                <div class="col-span-2">
                    <HotelForm hotel=h />
                </div>
            </div>
        </div>
    }
}

#[component]
pub(crate) fn HotelForm(
    hotel: Hotel
) -> impl IntoView {
    view! {
        <div class="p-3 m-3 border-2 rounded-xl">
            <p class="text-xl"><strong>${hotel.price}</strong> night</p>
            <CustomDatePicker />
        </div>
    }
}

#[component]
pub(crate) fn CustomDatePicker() -> impl IntoView {
    let from_date = create_rw_signal(Local::now().date_naive());
    let to_date = create_rw_signal((Local::now() + Duration::days(5)).date_naive());
    let (show_date_picker, set_show_date_picker) = create_signal(false);

    view! {
        <div class="relative border-2">
            <div>
                <input prop:value={move || from_date.get().to_string()} on:click={move |_| set_show_date_picker(true)} />
            </div>
            <Show when={show_date_picker}>
                <div class="absolute right-0 bg-white">
                    DatePickerPopOut
                </div>
            </Show>
        </div>
    }
}