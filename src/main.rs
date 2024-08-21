mod data;
mod home_page;
mod hotel_page;

use crate::home_page::HomePage;
use crate::hotel_page::HotelPage;
use leptos::*;
use leptos_router::*;

fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <nav class="flex flex-row justify-center my-4">
                <h3 class="font-bold text-2xl">Hotel Booking</h3>
            </nav>
            <main>
                <Routes>
                    <Route path="/" view=HomePage />
                    <Route path="/hotel/:id" view=HotelPage />
                </Routes>
            </main>
        </Router>
    }
}
