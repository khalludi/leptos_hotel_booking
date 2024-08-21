use leptos::*;
use leptos_router::*;
use crate::data::get_hotel;

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
        println!("Called hotel");
        get_hotel(id().unwrap()).unwrap()
    };

    view! {
        <p>Hotel page with id: {id}</p>
        <p>{move || hotel().name}</p>
        <p>{move || hotel().beds}</p>
    }
}