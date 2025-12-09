use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ColorDropdownProps {
    pub selected_color: String,
    pub on_select: Callback<String>,
}

#[function_component(ColorDropdown)]
pub fn color_dropdown(props: &ColorDropdownProps) -> Html {
    let colors = vec![
        ("black", "Preto"),
        ("gray", "Cinza"),
        ("white", "Branco"),
        ("blue", "Azul"),
        ("red", "Vermelho"),
        ("green", "Verde"),
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
        Callback::from(move |color: String| {
            on_select.emit(color);
            is_open.set(false);
        })
    };

    html! {
    <div class="color-dropdown">
        <button onclick={toggle_dropdown} class="btn-special">
            <span class="btn-icon">{ "🎨" }</span>
            <span class="btn-label">{ "Cor" }</span>
            <span class={"btn-value"}>{ props.selected_color.clone() }</span>
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
