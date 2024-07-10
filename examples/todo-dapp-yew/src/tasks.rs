use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TaskFormProps {
    pub on_add: Callback<String>,
}

#[function_component]
pub fn TaskForm(props: &TaskFormProps) -> Html {
    let TaskFormProps { on_add } = props;
    let task_name = use_state(String::new);

    let on_input = {
        let task_name = task_name.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            task_name.set(input.value());
        })
    };

    let on_submit = {
        let on_add = on_add.clone();
        let task_name = task_name.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            on_add.emit((*task_name).clone());
            task_name.set(String::new());
        })
    };

    html! {
        <form onsubmit={on_submit}>
            <button>
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    viewBox="0 0 448 512"
                    width="24"
                    height="24"
                >
                    <path
                        fill="#00ff99"
                        d="M416 208H272V64C272 46.33 257.7 32 240 32H208C190.3 32 176 46.33 176 64V208H32C14.33 208 0 222.3 0 240V272C0 289.7 14.33 304 32 304H176V448C176 465.7 190.3 480 208 480H240C257.7 480 272 465.7 272 448V304H416C433.7 304 448 289.7 448 272V240C448 222.3 433.7 208 416 208z"
                    />
                </svg>
            </button>
            <input
                type="text"
                required=true
                value={(*task_name).clone()}
                oninput={on_input}
                placeholder="Your next task..."
            />
        </form>
    }
}
