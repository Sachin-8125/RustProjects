use yew::prelude::*;
use web_sys::HtmlInputElement;
use crate::{
    services::{api::ApiService, auth::AuthService},
    types::{Todo, TodoUpdate},
};

#[function_component(TodoList)]
pub fn todo_list() -> Html {
    let todos = use_state(Vec::<Todo>::new);
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);
    let new_todo_title = use_state(String::new);
    let new_todo_description = use_state(String::new);

    let auth_service = use_memo((), |_| AuthService::new());

    let title_ref = use_node_ref();
    let description_ref = use_node_ref();

    // Load todos on component mount
    {
        let todos = todos.clone();
        let loading = loading.clone();
        let error = error.clone();

        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                match ApiService::get_todos().await {
                    Ok(fetched_todos) => {
                        todos.set(fetched_todos);
                        loading.set(false);
                    }
                    Err(err) => {
                        error.set(Some(err));
                        loading.set(false);
                    }
                }
            });
            || ()
        });
    }

    let handle_logout = {
        let auth_service = auth_service.clone();
        Callback::from(move |_| {
            auth_service.logout();
        })
    };

    let handle_create_todo = {
        let todos = todos.clone();
        let error = error.clone();
        let new_todo_title = new_todo_title.clone();
        let new_todo_description = new_todo_description.clone();
        let title_ref = title_ref.clone();
        let description_ref = description_ref.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            let title = title_ref
                .cast::<HtmlInputElement>()
                .map(|input| input.value())
                .unwrap_or_default();

            let description = description_ref
                .cast::<HtmlInputElement>()
                .map(|input| input.value())
                .filter(|s| !s.is_empty());

            if title.trim().is_empty() {
                return;
            }

            let todos = todos.clone();
            let error = error.clone();
            let new_todo_title = new_todo_title.clone();
            let new_todo_description = new_todo_description.clone();

            wasm_bindgen_futures::spawn_local(async move {
                match ApiService::create_todo(title, description).await {
                    Ok(new_todo) => {
                        let mut current_todos = (*todos).clone();
                        current_todos.insert(0, new_todo);
                        todos.set(current_todos);
                        new_todo_title.set(String::new());
                        new_todo_description.set(String::new());
                        error.set(None);
                    }
                    Err(err) => {
                        error.set(Some(err));
                    }
                }
            });
        })
    };

    let create_toggle_handler = |todo_id: String, completed: bool| {
        let todos = todos.clone();
        let error = error.clone();

        Callback::from(move |_| {
            let todos = todos.clone();
            let error = error.clone();
            let todo_id = todo_id.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let update = TodoUpdate {
                    title: None,
                    description: None,
                    completed: Some(!completed),
                };

                match ApiService::update_todo(&todo_id, update).await {
                    Ok(updated_todo) => {
                        let mut current_todos = (*todos).clone();
                        if let Some(index) = current_todos.iter().position(|t| t.id == todo_id) {
                            current_todos[index] = updated_todo;
                            todos.set(current_todos);
                        }
                        error.set(None);
                    }
                    Err(err) => {
                        error.set(Some(err));
                    }
                }
            });
        })
    };

    let create_delete_handler = |todo_id: String| {
        let todos = todos.clone();
        let error = error.clone();

        Callback::from(move |_| {
            let todos = todos.clone();
            let error = error.clone();
            let todo_id = todo_id.clone();

            wasm_bindgen_futures::spawn_local(async move {
                match ApiService::delete_todo(&todo_id).await {
                    Ok(()) => {
                        let current_todos = (*todos).clone();
                        let filtered_todos: Vec<Todo> = current_todos
                            .into_iter()
                            .filter(|t| t.id != todo_id)
                            .collect();
                        todos.set(filtered_todos);
                        error.set(None);
                    }
                    Err(err) => {
                        error.set(Some(err));
                    }
                }
            });
        })
    };

    if *loading {
        return html! {
            <div class="flex justify-center items-center min-h-96">
                <div class="text-lg">{"Loading..."}</div>
            </div>
        };
    }

    html! {
        <div class="max-w-4xl mx-auto px-4">
            <div class="flex justify-between items-center mb-8">
                <h1 class="text-2xl font-bold text-gray-900">{"My Todos"}</h1>
                <button
                    onclick={handle_logout}
                    class="bg-red-600 hover:bg-red-700 text-white font-bold py-2 px-4 rounded"
                >
                    {"Logout"}
                </button>
            </div>

            {if let Some(error_msg) = error.as_ref() {
                html! {
                    <div class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4">
                        {error_msg}
                    </div>
                }
            } else {
                html! {}
            }}

            // Create new todo form
            <div class="bg-white shadow rounded-lg p-6 mb-6">
                <h2 class="text-lg font-semibold text-gray-900 mb-4">{"Add New Todo"}</h2>
                <form onsubmit={handle_create_todo}>
                    <div class="mb-4">
                        <label for="title" class="block text-sm font-medium text-gray-700 mb-2">
                            {"Title"}
                        </label>
                        <input
                            ref={title_ref}
                            type="text"
                            id="title"
                            class="block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
                            placeholder="Enter todo title"
                            value={(*new_todo_title).clone()}
                        />
                    </div>
                    <div class="mb-4">
                        <label for="description" class="block text-sm font-medium text-gray-700 mb-2">
                            {"Description (Optional)"}
                        </label>
                        <input
                            ref={description_ref}
                            type="text"
                            id="description"
                            class="block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
                            placeholder="Enter todo description"
                            value={(*new_todo_description).clone()}
                        />
                    </div>
                    <button
                        type="submit"
                        class="bg-indigo-600 hover:bg-indigo-700 text-white font-bold py-2 px-4 rounded"
                    >
                        {"Add Todo"}
                    </button>
                </form>
            </div>

            // Todo list
            <div class="space-y-4">
                {for todos.iter().map(|todo| {
                    let toggle_handler = create_toggle_handler(todo.id.clone(), todo.completed);
                    let delete_handler = create_delete_handler(todo.id.clone());

                    html! {
                        <div key={todo.id.clone()} class={format!(
                            "bg-white shadow rounded-lg p-6 {}",
                            if todo.completed { "opacity-75" } else { "" }
                        )}>
                            <div class="flex items-center justify-between">
                                <div class="flex items-center space-x-3">
                                    <input
                                        type="checkbox"
                                        checked={todo.completed}
                                        onchange={toggle_handler}
                                        class="h-4 w-4 text-indigo-600 focus:ring-indigo-500 border-gray-300 rounded"
                                    />
                                    <div>
                                        <h3 class={format!(
                                            "text-lg font-medium {}",
                                            if todo.completed { "line-through text-gray-500" } else { "text-gray-900" }
                                        )}>
                                            {&todo.title}
                                        </h3>
                                        {if let Some(description) = &todo.description {
                                            html! {
                                                <p class={format!(
                                                    "text-sm {}",
                                                    if todo.completed { "text-gray-400" } else { "text-gray-600" }
                                                )}>
                                                    {description}
                                                </p>
                                            }
                                        } else {
                                            html! {}
                                        }}
                                        <p class="text-xs text-gray-400 mt-1">
                                            {"Created: "}{todo.created_at.format("%Y-%m-%d %H:%M").to_string()}
                                        </p>
                                    </div>
                                </div>
                                <button
                                    onclick={delete_handler}
                                    class="text-red-600 hover:text-red-800"
                                >
                                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"></path>
                                    </svg>
                                </button>
                            </div>
                        </div>
                    }
                })}

                {if todos.is_empty() {
                    html! {
                        <div class="text-center py-12">
                            <div class="text-gray-500 text-lg">{"No todos yet! Create your first todo above."}</div>
                        </div>
                    }
                } else {
                    html! {}
                }}
            </div>
        </div>
    }
}