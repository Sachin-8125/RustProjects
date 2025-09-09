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
            

        </BrowserRouter>

    }

}



fn main(){
    yew::Renderer::<App>::new().render();
}