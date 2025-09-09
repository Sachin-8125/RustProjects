use yew::prelude::*;
use web_sys::HtmlInputElement;
use crate::services::auth::AuthService;

#[function_component(Auth)]
pub fn auth() -> Html{
    let is_login = use_state(|| true);
    let error_message = use_state(|| None::<String>);
    let loading = use_state(|| false);;
    
}