use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CheckboxProps {
    pub checked: bool,
    pub on_click: Callback<MouseEvent>,
}

#[function_component]
pub fn Checkbox(props: &CheckboxProps) -> Html {
    let CheckboxProps { checked, on_click } = props;
    html! {
        <div onclick={on_click}>
            if !checked {
                <div class="checkbox unchecked">
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        viewBox="0 0 448 512"
                        width="24"
                        height="24"
                    >
                        <path
                            fill="#e0e0e0"
                            d="M384 32C419.3 32 448 60.65 448 96V416C448 451.3 419.3 480 384 480H64C28.65 480 0 451.3 0 416V96C0 60.65 28.65 32 64 32H384zM384 80H64C55.16 80 48 87.16 48 96V416C48 424.8 55.16 432 64 432H384C392.8 432 400 424.8 400 416V96C400 87.16 392.8 80 384 80z"
                        />
                    </svg>
                </div>
            } else {
                <div class="checkbox checked">
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        viewBox="0 0 448 512"
                        width="24"
                        height="24"
                    >
                        <path
                            fill="#00ff99"
                            d="M64 32C28.65 32 0 60.65 0 96V416C0 451.3 28.65 480 64 480H384C419.3 480 448 451.3 448 416V96C448 60.65 419.3 32 384 32H64zM337 209L209 337C199.6 346.4 184.4 346.4 174.1 337L110.1 273C100.6 263.6 100.6 248.4 110.1 239C119.6 229.6 134.8 229.6 144.2 239L191 285.8L303 174C312.4 164.6 327.6 164.6 337 174C346.4 183.4 346.4 198.6 337 209z"
                        />
                    </svg>
                </div>
            }
        </div>
    }
}
