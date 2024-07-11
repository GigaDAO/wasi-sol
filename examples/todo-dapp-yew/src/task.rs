use crate::checkbox::Checkbox;
use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Task {
    pub name: String,
    pub done: bool,
}

#[derive(Properties, PartialEq)]
pub struct TaskProps {
    pub task: Task,
    pub on_toggle: Callback<bool>,
    pub on_trash: Callback<MouseEvent>,
    pub on_rename: Callback<String>,
}

#[function_component(TaskComponent)]
pub fn task_component(props: &TaskProps) -> Html {
    let TaskProps {
        task,
        on_toggle,
        on_trash,
        on_rename,
    } = props;

    let edit_mode = use_state(|| false);
    let task = task.clone();

    let on_name_click = {
        let edit_mode = edit_mode.clone();
        Callback::from(move |_| edit_mode.set(!*edit_mode))
    };

    let on_rename_input = {
        let on_rename = on_rename.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            on_rename.emit(input.value());
        })
    };

    let on_submit = {
        let edit_mode = edit_mode.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            edit_mode.set(false);
        })
    };

    html! {
        <div class={classes!("task", if task.done { "done" } else { "" })}>
            <Checkbox checked={task.done} on_click={on_toggle.reform(move |_| !task.done)} />
            if !*edit_mode {
                <div class="task-name" onclick={on_name_click}>
                    <span>{ &task.name }</span>
                </div>
            } else {
                <form onsubmit={on_submit}>
                    <input
                        type="text"
                        required=true
                        value={task.name.clone()}
                        oninput={on_rename_input}
                    />
                </form>
            }
            <button class="trash" onclick={on_trash}>
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    viewBox="0 0 448 512"
                    width="24"
                    height="24"
                >
                    <path
                        fill="#ff0099"
                        d="M135.2 17.7L128 32H32C14.33 32 0 46.33 0 64S14.33 96 32 96H416C433.7 96 448 81.67 448 64S433.7 32 416 32H320L312.8 17.7C307.4 6.813 296.3 0 284.2 0H163.8C151.7 0 140.6 6.813 135.2 17.7zM32 128L53.2 467C54.84 493.3 76.53 512 102.8 512H345.2C371.5 512 393.2 493.3 394.8 467L416 128H32z"
                    />
                </svg>
            </button>
        </div>
    }
}
