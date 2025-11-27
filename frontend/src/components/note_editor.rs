// src/components/note_editor.rs
use yew::prelude::*;
use crate::models::note::Note;

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
                n
            } else {
                Note::new(
                    (*title).clone(),
                    (*content).clone(),
                    user_id.clone(),
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
                        value={(*content).clone()}
                        oninput={on_content_change}
                    />
                </div>
                
                <div class="editor-footer">
                    <span class="char-count">{ format!("{} caracteres", char_count) }</span>
                    <div class="editor-actions">
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
