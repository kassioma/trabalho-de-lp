// src/components/note_editor.rs
use yew::{use_state, prelude::*};
use crate::components::background_dropdown::BackgroundDropdown;
use crate::models::note::{Note, NoteHistory};
use crate::components::font_dropdown::{FontDropdown};
use crate::components::color_dropdown::{ColorDropdown};

#[derive(Properties, PartialEq)]
pub struct NoteEditorProps {
    pub note: Option<Note>,
    pub user_id: String,
    pub on_save: Callback<Note>,
    pub on_close: Callback<()>,
}

#[function_component(NoteEditor)]
pub fn note_editor(props: &NoteEditorProps) -> Html {
    let title = use_state(|| {
        props.note.as_ref()
            .map(|n| n.title.clone())
            .unwrap_or_default()
    });
    
    let content = use_state(|| {
        props.note.as_ref()
            .map(|n| n.content.clone())
            .unwrap_or_default()
    });

    let history = use_state(|| {
        props.note.as_ref()
            .map(|n| n.history.clone())
            .unwrap_or_else(|| vec![])
    });

    let current_version_index = use_state(|| {
    props.note.as_ref()
        .map(|n| n.history.len())
        .unwrap_or(0)
    });

    let selected_font = use_state(|| {
    props.note.as_ref()
        .map(|n| n.font.clone())
        .unwrap_or_else(|| "Arial".to_string())
    });

    let on_font_select = {
        let selected_font = selected_font.clone();
        Callback::from(move |font: String| {
            selected_font.set(font);
        })
    };

    let text_color = use_state(|| {
    props.note.as_ref()
        .map(|n| n.color.clone())
        .unwrap_or_else(|| "black".to_string())
    });

    let on_color_select = {
        let text_color = text_color.clone();
        Callback::from(move |color: String| {
            text_color.set(color);
        })
    };

    let background_color = use_state(|| {
    props.note.as_ref()
        .map(|n| n.background.clone())
        .unwrap_or_else(|| "white".to_string())
    });

    let on_background_select = {
        let background_color = background_color.clone();
        Callback::from(move |color: String| {
            background_color.set(color);
        })
    };
    
    let on_title_change = {
        let title = title.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            title.set(input.value());
        })
    };
    
    let on_content_change = {
        let content = content.clone();
        Callback::from(move |e: InputEvent| {
            let textarea: web_sys::HtmlTextAreaElement = e.target_unchecked_into();
            content.set(textarea.value());
        })
    };
    
    let on_save_click = {
        let title = title.clone();
        let content = content.clone();
        let on_save = props.on_save.clone();
        let user_id = props.user_id.clone();
        let existing_note = props.note.clone();
        let color = text_color.clone();
        let font = selected_font.clone();
        let background = background_color.clone();
        
        Callback::from(move |_| {
            if title.is_empty() {
                web_sys::window()
                    .unwrap()
                    .alert_with_message("O título não pode estar vazio!")
                    .unwrap();
                return;
            }
            
            let note = if let Some(existing) = existing_note.as_ref() {
                let mut n = existing.clone();
                n.title = (*title).clone();
                n.content = (*content).clone();
                n.updated_at = js_sys::Date::now() as i64;
                let old  = NoteHistory {title: existing.title.clone(), 
                    content: existing.content.clone(), updated_at: existing.updated_at,};
                n.history.push(old);
                n.color = color.clone().to_string();
                n.background = background.clone().to_string();
                n.font = font.clone().to_string();
                n
            } else {
                Note::new(
                    (*title).clone(),
                    (*content).clone(),
                    user_id.clone(),
                    font.clone().to_string(),
                    color.clone().to_string(),
                    background.clone().to_string()
                )
            };
            
            on_save.emit(note);
        })
    };
    
    let on_close_click = {
        let on_close = props.on_close.clone();
        Callback::from(move |_| {
            on_close.emit(());
        })
    };

    let on_earlier_click = {
        let history = history.clone();
        let content = content.clone();
        let title = title.clone();

        Callback::from(move |_: MouseEvent| {
            if *current_version_index > 0 {
                let new_index = *current_version_index -1;
                if let Some(previous_note) = history.get(new_index as usize) {
                    title.set(previous_note.title.clone());
                    content.set(previous_note.content.clone());
                    current_version_index.set(new_index);
                }
            } else {
                web_sys::window()
                    .unwrap()
                    .alert_with_message("Esta é a primeira versão.")
                    .unwrap();
            }
        })
    };
    
    let is_new = props.note.is_none();
    let char_count = content.len();
    
    
    html! {
        <div class="note-editor">
            <div class="editor-header">
                <h2>{ if is_new { "Nova Nota" } else { "Editar Nota" } }</h2>
                <button onclick={&on_close_click} class="btn-close">{ "✕" }</button>
            </div>
            
            <div class="editor-body">
                <div class="form-group">
                    <input
                        type="text"
                        class="note-title-input"
                        placeholder="Título da nota"
                        value={(*title).clone()}
                        oninput={on_title_change}
                    />
                </div>
                
                <div class="form-group">
                    <textarea
                        class="note-content-input"
                        placeholder="Escreva sua nota aqui..."
                        style={format!("font-family: {}; background-color: {}; color: {}; ", *selected_font, *background_color, *text_color)}
                        value={(*content).clone()}
                        oninput={on_content_change}
                    />
                </div>
                <div class="dropdown-buttons">
                    <BackgroundDropdown selected_background={(*background_color).clone()} on_select={on_background_select} />
                    <ColorDropdown selected_color={(*text_color).clone()} on_select={on_color_select} />
                    <FontDropdown selected_font={(*selected_font).clone()} on_select={on_font_select} />
                </div>
                
                <div class="editor-footer">
                    <span class="char-count">{ format!("{} caracteres", char_count) }</span>
                    <div class="editor-actions">
                        <button onclick={&on_earlier_click} class="btn-secondary">
                            { "Versão Anterior"}
                        </button>
                        <button onclick={&on_close_click} class="btn-secondary">
                            { "Cancelar" }
                        </button>
                        <button onclick={on_save_click} class="btn-primary">
                            { "Salvar" }
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
}
