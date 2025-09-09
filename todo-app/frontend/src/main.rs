use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod services;
mod types;

use components::{Auth, TodoList};
use services::auth::AuthService;
use types::Route;



#[function_component(App)]
fn app() -> Html{
    let auth_service = use_memo((), |_| AuthService::new());
    html! {
        <BrowserRouter>
            <div class="min-h-screen bg-gray-100">
                <header class="bg-white shadow">
                    <div class="max-w-7xl mx-auto py-6 px-4 sm:px-6 lg:px-8">
                        <h1 class="text-3xl font-bold text-gray-900"> {"Todo App"}</h1>
                    </div>
                </header>
                <main>
                    <div class="max-w-7xl mx-auto py-6 sm:px-6 lg:px-8">
                        <Switch<Route> render={move |route| {
                            let auth_service = auth_service.clone();
                            match route {
                                Route::Home => {
                                    if auth_service.is_logged_in(){
                                        html!{<TodoList/>}
                                    }else{
                                        html!{<Auth/>}
                                    }
                                }
                                Route::Login => html! {<Auth/>},
                            }
                        }}/>
                    </div>
                </main>

            </div>

        </BrowserRouter>

    }

    }



fn main(){
    yew::Renderer::<App>::new().render();
}