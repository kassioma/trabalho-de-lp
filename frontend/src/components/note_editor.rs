// src/components/note_editor.rs
use yew::{use_state, prelude::*, AttrValue, Html};
use pulldown_cmark::{Parser, Options, html};
use ammonia::clean;
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

    // Keep a copy of the saved/current note values so "Versão Seguinte" can restore them
    let saved_title = use_state(|| {
        props.note.as_ref()
            .map(|n| n.title.clone())
            .unwrap_or_default()
    });

    let saved_content = use_state(|| {
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

    // font size (px)
    let font_size = use_state(|| {
        props.note.as_ref()
            .and_then(|n| n.font_size)
            .unwrap_or(16u8)
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
    
    // reference to textarea for selection manipulation
    let textarea_ref = NodeRef::default();
    
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
        let font_size = font_size.clone();
        let saved_title_state = saved_title.clone();
        let saved_content_state = saved_content.clone();

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
                n.font_size = Some(*font_size);
                n
            } else {
                Note::new(
                    (*title).clone(),
                    (*content).clone(),
                    user_id.clone(),
                    font.clone().to_string(),
                    color.clone().to_string(),
                    background.clone().to_string()
                    , Some(*font_size)
                )
            };
            
            on_save.emit(note);
            // update saved copies so history navigation can restore the latest
            saved_title_state.set((*title).clone());
            saved_content_state.set((*content).clone());
        })
    };
    
    let on_close_click = {
        let on_close = props.on_close.clone();
        Callback::from(move |_| {
            on_close.emit(());
        })
    };

    // Preview mode
    let preview = use_state(|| false);
    let on_toggle_preview = {
        let preview = preview.clone();
        Callback::from(move |_| {
            preview.set(!*preview);
        })
    };

    // Full Markdown renderer using pulldown-cmark + ammonia for sanitization
    fn markdown_to_html(src: &str) -> String {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_FOOTNOTES);
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TASKLISTS);

        let parser = Parser::new_ext(src, options);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);

        // sanitize generated HTML
        clean(&html_output)
    }

    let on_earlier_click = {
        let history = history.clone();
        let content = content.clone();
        let title = title.clone();
        let current_version_index = current_version_index.clone();

        Callback::from(move |_: MouseEvent| {
            if *current_version_index > 0 {
                let new_index = *current_version_index -1;
                if let Some(previous_note) = history.get(new_index as usize) {
                    title.set(previous_note.title.clone());
                    content.set(previous_note.content.clone());
                    current_version_index.set(new_index);
                }
            }
        })
    };

    let on_later_click = {
        let history = history.clone();
        let content = content.clone();
        let title = title.clone();
        let current_version_index = current_version_index.clone();
        let saved_title = saved_title.clone();
        let saved_content = saved_content.clone();

        Callback::from(move |_: MouseEvent| {
            let max_index = history.len();
            // current_version_index refers to a position in history (0..=len)
            if *current_version_index < max_index {
                let new_index = *current_version_index + 1;
                if new_index == max_index {
                    // restore saved (latest) values
                    title.set((*saved_title).clone());
                    content.set((*saved_content).clone());
                    current_version_index.set(new_index);
                } else if let Some(next_note) = history.get(new_index as usize) {
                    title.set(next_note.title.clone());
                    content.set(next_note.content.clone());
                    current_version_index.set(new_index);
                } else {
                    web_sys::window()
                        .unwrap()
                        .alert_with_message("Esta é a versão mais recente.")
                        .unwrap();
                }
            } else {
                // no-op when already at latest (button will be disabled)
            }
        })
    };
    
    let is_new = props.note.is_none();
    let char_count = content.len();
    let preview_html = markdown_to_html(&*content);
    
    // formatting toolbar handlers
    // helper: convert UTF-16 index (JS selectionStart/End) to Rust byte index
    let utf16_to_byte_index = |s: &str, target: usize| -> usize {
        let mut utf16_count: usize = 0;
        for (byte_idx, ch) in s.char_indices() {
            let ch_u32 = ch as u32;
            let ch_utf16_len = if ch_u32 >= 0x10000 { 2 } else { 1 };
            if utf16_count + ch_utf16_len > target {
                return byte_idx;
            }
            utf16_count += ch_utf16_len;
        }
        s.len()
    };

    // Create reusable actions for bold/italic with toggle support
    let do_bold = {
        let content = content.clone();
        let textarea_ref = textarea_ref.clone();
        let utf16_to_byte_index = utf16_to_byte_index.clone();
        Callback::from(move |_: ()| {
            if let Some(elem) = textarea_ref.cast::<web_sys::HtmlTextAreaElement>() {
                let val = elem.value();
                let start = elem.selection_start().unwrap_or(Some(0)).unwrap_or(0) as u32;
                let end = elem.selection_end().unwrap_or(Some(0)).unwrap_or(0) as u32;

                if start < end {
                    let s_byte = utf16_to_byte_index(&val, start as usize);
                    let e_byte = utf16_to_byte_index(&val, end as usize);
                    let before = &val[..s_byte];
                    let selected = &val[s_byte..e_byte];
                    let after = &val[e_byte..];

                    // toggle: if already wrapped with ** on both sides, remove them
                    if before.ends_with("**") && after.starts_with("**") {
                        let new_before = &before[..before.len()-2];
                        let new_after = &after[2..];
                        let new = format!("{}{}{}", new_before, selected, new_after);
                        elem.set_value(&new);
                        content.set(new);
                        let new_start = start - 2;
                        let new_end = end - 2;
                        let _ = elem.set_selection_range(new_start, new_end);
                    } else {
                        let new = format!("{}**{}**{}", before, selected, after);
                        elem.set_value(&new);
                        content.set(new);
                        let new_start = start + 2;
                        let new_end = end + 2;
                        let _ = elem.set_selection_range(new_start, new_end);
                    }
                } else {
                    // no selection: insert pair and place caret between
                    let s_byte = utf16_to_byte_index(&val, start as usize);
                    let before = &val[..s_byte];
                    let after = &val[s_byte..];
                    let new = format!("{}****{}", before, after);
                    elem.set_value(&new);
                    content.set(new);
                    let caret = start + 2;
                    let _ = elem.set_selection_range(caret, caret);
                }
                let _ = elem.focus();
            }
        })
    };

    let do_italic = {
        let content = content.clone();
        let textarea_ref = textarea_ref.clone();
        let utf16_to_byte_index = utf16_to_byte_index.clone();
        Callback::from(move |_: ()| {
            if let Some(elem) = textarea_ref.cast::<web_sys::HtmlTextAreaElement>() {
                let val = elem.value();
                let start = elem.selection_start().unwrap_or(Some(0)).unwrap_or(0) as u32;
                let end = elem.selection_end().unwrap_or(Some(0)).unwrap_or(0) as u32;

                if start < end {
                    let s_byte = utf16_to_byte_index(&val, start as usize);
                    let e_byte = utf16_to_byte_index(&val, end as usize);
                    let before = &val[..s_byte];
                    let selected = &val[s_byte..e_byte];
                    let after = &val[e_byte..];

                    // toggle: if already wrapped with * on both sides, remove them
                    if before.ends_with("*") && after.starts_with("*") {
                        let new_before = &before[..before.len()-1];
                        let new_after = &after[1..];
                        let new = format!("{}{}{}", new_before, selected, new_after);
                        elem.set_value(&new);
                        content.set(new);
                        let new_start = start - 1;
                        let new_end = end - 1;
                        let _ = elem.set_selection_range(new_start, new_end);
                    } else {
                        let new = format!("{}*{}*{}", before, selected, after);
                        elem.set_value(&new);
                        content.set(new);
                        let new_start = start + 1;
                        let new_end = end + 1;
                        let _ = elem.set_selection_range(new_start, new_end);
                    }
                } else {
                    let s_byte = utf16_to_byte_index(&val, start as usize);
                    let before = &val[..s_byte];
                    let after = &val[s_byte..];
                    let new = format!("{}**{}", before, after);
                    elem.set_value(&new);
                    content.set(new);
                    let caret = start + 1;
                    let _ = elem.set_selection_range(caret, caret);
                }
                let _ = elem.focus();
            }
        })
    };

    let on_increase_font = {
        let font_size = font_size.clone();
        Callback::from(move |_| {
            let current = *font_size;
            if current < 72u8 {
                font_size.set(current + 2);
            }
        })
    };

    let on_decrease_font = {
        let font_size = font_size.clone();
        Callback::from(move |_| {
            let current = *font_size;
            if current > 8u8 {
                font_size.set(current.saturating_sub(2));
            }
        })
    };
    
    
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
                    if *preview {
                        <div class="note-preview" style={format!("font-family: {}; background-color: {}; color: {}; padding: 12px; border-radius: 4px;", *selected_font, *background_color, *text_color)}>
                            { Html::from_html_unchecked(AttrValue::from(preview_html.clone())) }
                        </div>
                    } else {
                        <div>
                            <div class="format-toolbar">
                                <button class="format-btn" onmousedown={Callback::from(|e: MouseEvent| e.prevent_default())} onclick={Callback::from({ let do_bold = do_bold.clone(); move |_: MouseEvent| { do_bold.emit(()) } })} title="Negrito">{"B"}</button>
                                <button class="format-btn" onmousedown={Callback::from(|e: MouseEvent| e.prevent_default())} onclick={Callback::from({ let do_italic = do_italic.clone(); move |_: MouseEvent| { do_italic.emit(()) } })} title="Itálico">{"I"}</button>
                                <div class="font-size-controls">
                                    <button class="format-btn" onclick={on_decrease_font.clone()} title="Diminuir fonte">{"-"}</button>
                                    <span class="font-size-label">{ format!("{}px", *font_size) }</span>
                                    <button class="format-btn" onclick={on_increase_font.clone()} title="Aumentar fonte">{"+"}</button>
                                </div>
                            </div>
                            <textarea
                                ref={textarea_ref.clone()}
                                class="note-content-input"
                                placeholder="Escreva sua nota aqui..."
                                style={format!("font-family: {}; background-color: {}; color: {}; font-size: {}px;", *selected_font, *background_color, *text_color, *font_size)}
                                value={(*content).clone()}
                                oninput={on_content_change}
                                onkeydown={Callback::from({ let do_bold = do_bold.clone(); let do_italic = do_italic.clone(); move |e: KeyboardEvent| {
                                    if e.ctrl_key() || e.meta_key() {
                                        let k = e.key();
                                        if k.eq_ignore_ascii_case("b") {
                                            e.prevent_default();
                                            do_bold.emit(());
                                        } else if k.eq_ignore_ascii_case("i") {
                                            e.prevent_default();
                                            do_italic.emit(());
                                        }
                                    }
                                } })}
                            />
                        </div>
                    }
                </div>
                <div class="dropdown-buttons">
                    <BackgroundDropdown selected_background={(*background_color).clone()} on_select={on_background_select} />
                    <ColorDropdown selected_color={(*text_color).clone()} on_select={on_color_select} />
                    <FontDropdown selected_font={(*selected_font).clone()} on_select={on_font_select} />
                </div>
                
                <div class="editor-footer">
                    <span class="char-count">{ format!("{} caracteres", char_count) }</span>
                    <div class="editor-actions">
                        <button onclick={on_toggle_preview.clone()} class="btn-secondary">
                            { if *preview { "Editar" } else { "Visualizar" } }
                        </button>
                        <button onclick={&on_earlier_click} class="btn-secondary" disabled={ *current_version_index == 0 }>{ "Versão Anterior" }</button>
                        <button onclick={&on_later_click} class="btn-secondary" disabled={ *current_version_index >= history.len() }>{ "Versão Seguinte" }</button>
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
