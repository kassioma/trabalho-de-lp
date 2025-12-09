use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct BackgroundDropdownProps {
    pub selected_background: String,
    pub on_select: Callback<String>,
}

#[function_component(BackgroundDropdown)]
pub fn background_dropdown(props: &BackgroundDropdownProps) -> Html {
    let colors = vec![
        ("white", "Branco"),
        ("lightgray", "Cinza Claro"),
        ("black", "Preto"),
        ("lightblue", "Azul Claro"),
        ("lightcoral", "Vermelho Claro"),
        ("lightgreen", "Verde Claro"),
    ];

    let is_open = use_state(|| false);
    
    let toggle_dropdown = {
        let is_open = is_open.clone();
        Callback::from(move |_| {
            is_open.set(!*is_open);
        })
    };

    let on_select_callback = {
        let on_select = props.on_select.clone();
        let is_open = is_open.clone();
        Callback::from(move |background: String| {
            on_select.emit(background);
            is_open.set(false);
        })
    };

    html! {
    <div class="background-dropdown">
        <button onclick={toggle_dropdown} class="btn-special">
            <span class="btn-icon">{ "🖌️" }</span>
            <span class="btn-label">{ "Fundo" }</span>
            <span class={"btn-value"}>{ props.selected_background.clone() }</span>
        </button>
        if *is_open.clone() {
            <div class="dropdown-menu">
                { for colors.iter().map(|&(color_code, color_name)| {
                        let on_select = on_select_callback.clone();
                        html! {
                            <button
                                onclick={Callback::from(move |_| on_select.emit(color_code.to_string()))}
                                class="dropdown-item"
                                style={format!("display:flex;align-items:center;gap:8px;")}
                            >
                                <span class="swatch" style={format!("background:{};", color_code)}></span>
                                <span>{ color_name }</span>
                            </button>
                        }
                }) }
            </div>
        }
    </div>
}
}
