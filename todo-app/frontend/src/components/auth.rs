use yew::prelude::*;
use web_sys::HtmlInputElement;
use crate::services::auth::AuthService;

#[function_component(Auth)]
pub fn auth() -> Html {
    let is_login = use_state(|| true);
    let error_message = use_state(|| None::<String>);
    let loading = use_state(|| false);

    let username_ref = use_node_ref();
    let email_ref = use_node_ref();
    let password_ref = use_node_ref();

    let auth_service = use_memo((), |_| AuthService::new());

    let toggle_mode = {
        let is_login = is_login.clone();
        let error_message = error_message.clone();
        Callback::from(move |_| {
            is_login.set(!*is_login);
            error_message.set(None);
        })
    };

    let handle_submit = {
        let is_login = is_login.clone();
        let error_message = error_message.clone();
        let loading = loading.clone();
        let auth_service = auth_service.clone();
        let username_ref = username_ref.clone();
        let email_ref = email_ref.clone();
        let password_ref = password_ref.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            
            let is_login = *is_login;
            let error_message = error_message.clone();
            let loading = loading.clone();
            let auth_service = auth_service.clone();

            let username = if !is_login {
                username_ref
                    .cast::<HtmlInputElement>()
                    .map(|input| input.value())
                    .unwrap_or_default()
            } else {
                String::new()
            };

            let email = email_ref
                .cast::<HtmlInputElement>()
                .map(|input| input.value())
                .unwrap_or_default();

            let password = password_ref
                .cast::<HtmlInputElement>()
                .map(|input| input.value())
                .unwrap_or_default();

            if email.is_empty() || password.is_empty() || (!is_login && username.is_empty()) {
                error_message.set(Some("Please fill in all fields".to_string()));
                return;
            }

            loading.set(true);
            error_message.set(None);

            wasm_bindgen_futures::spawn_local(async move {
                let result = if is_login {
                    auth_service.login(email, password).await
                } else {
                    auth_service.register(username, email, password).await
                };

                match result {
                    Ok(()) => {
                        web_sys::window()
                            .unwrap()
                            .location()
                            .reload()
                            .unwrap();
                    }
                    Err(err) => {
                        error_message.set(Some(err));
                        loading.set(false);
                    }
                }
            });
        })
    };

    html! {
        <div class="min-h-screen flex items-center justify-center bg-gray-50 py-12 px-4 sm:px-6 lg:px-8">
            <div class="max-w-md w-full space-y-8">
                <div>
                    <h2 class="mt-6 text-center text-3xl font-extrabold text-gray-900">
                        {if *is_login { "Sign in to your account" } else { "Create your account" }}
                    </h2>
                </div>
                <form class="mt-8 space-y-6" onsubmit={handle_submit}>
                    <div class="rounded-md shadow-sm -space-y-px">
                        {if !*is_login {
                            html! {
                                <div>
                                    <label for="username" class="sr-only">{"Username"}</label>
                                    <input
                                        ref={username_ref}
                                        id="username"
                                        name="username"
                                        type="text"
                                        required={true}
                                        class="appearance-none rounded-none relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-t-md focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 focus:z-10 sm:text-sm"
                                        placeholder="Username"
                                    />
                                </div>
                            }
                        } else {
                            html! {}
                        }}
                        <div>
                            <label for="email" class="sr-only">{"Email address"}</label>
                            <input
                                ref={email_ref}
                                id="email"
                                name="email"
                                type="email"
                                required={true}
                                class={format!("appearance-none rounded-none relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 {} focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 focus:z-10 sm:text-sm",
                                    if *is_login { "rounded-t-md" } else { "" }
                                )}
                                placeholder="Email address"
                            />
                        </div>
                        <div>
                            <label for="password" class="sr-only">{"Password"}</label>
                            <input
                                ref={password_ref}
                                id="password"
                                name="password"
                                type="password"
                                required={true}
                                class="appearance-none rounded-none relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-b-md focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 focus:z-10 sm:text-sm"
                                placeholder="Password"
                            />
                        </div>
                    </div>

                    {if let Some(error) = error_message.as_ref() {
                        html! {
                            <div class="text-red-600 text-sm text-center">
                                {error}
                            </div>
                        }
                    } else {
                        html! {}
                    }}

                    <div>
                        <button
                            type="submit"
                            disabled={*loading}
                            class="group relative w-full flex justify-center py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 disabled:opacity-50"
                        >
                            {if *loading {
                                "Loading..."
                            } else if *is_login {
                                "Sign In"
                            } else {
                                "Sign Up"
                            }}
                        </button>
                    </div>

                    <div class="text-center">
                        <button
                            type="button"
                            onclick={toggle_mode}
                            class="text-indigo-600 hover:text-indigo-500"
                        >
                            {if *is_login {
                                "Don't have an account? Sign up"
                            } else {
                                "Already have an account? Sign in"
                            }}
                        </button>
                    </div>
                </form>
            </div>
        </div>
    }
}