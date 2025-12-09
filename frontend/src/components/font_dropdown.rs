use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct FontDropdownProps {
    pub selected_font: String,
    pub on_select: Callback<String>,
}

#[function_component(FontDropdown)]
pub fn font_dropdown(props: &FontDropdownProps) -> Html {
    let fonts = vec![
        "Arial", "Courier New", "Georgia", "Times New Roman",
        "Verdana", "Comic Sans MS", "Impact", "Franklin Gothic Medium",
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
        Callback::from(move |font: String| {
            on_select.emit(font);
            is_open.set(false);
        })
    };

    html! {
    <div class="font-dropdown">
        <button onclick={toggle_dropdown} class="btn-special">
            <span class="btn-icon">{ "🔤" }</span>
            <span class="btn-label">{ "Fonte de Texto" }</span>
            <span class={"btn-value"}>{ props.selected_font.clone() }</span>
        </button>
        if *is_open.clone() {
            <div class="dropdown-menu">
                { for fonts.iter().map(|font| {
                    let font_clone = font.to_string();
                    let on_select = on_select_callback.clone();
                    html! {
                        <button
                            onclick={Callback::from(move |_| on_select.emit(font_clone.clone()))}
                            class="dropdown-item"
                            style={format!("display:flex;align-items:center;gap:10px;")}
                        >
                            <span class="font-sample" style={format!("font-family: {};", font)}>{ "Aa" }</span>
                            <span>{ font }</span>
                        </button>
                    }
                }) }
            </div>
        }
    </div>
}
}
