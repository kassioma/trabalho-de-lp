// src/components/dashboard.rs
use yew::prelude::*;
use yew_router::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::services::{auth::AuthService, notes::NotesService};
use crate::models::note::Note;
use crate::components::note_editor::NoteEditor;
use crate::Route;

#[function_component(Dashboard)]
pub fn dashboard() -> Html {
    let navigator = use_navigator().unwrap();
    let notes = use_state(|| Vec::<Note>::new());
    let selected_note = use_state(|| None::<Note>);
    let loading = use_state(|| true);
    let user = use_state(|| AuthService::get_current_user());
    let show_editor = use_state(|| false);
    
    // Verificar autenticação
    {
        let navigator = navigator.clone();
        let user = user.clone();
        use_effect_with((), move |_| {
            if user.is_none() {
                navigator.push(&Route::Login);
            }
            || ()
        });
    }
    
    // Carregar notas
    {
        let notes = notes.clone();
        let loading = loading.clone();
        let user = user.clone();
        
        use_effect_with((), move |_| {
            if let Some(current_user) = (*user).as_ref() {
                let user_id = current_user.uid.clone();
                let notes = notes.clone();
                let loading = loading.clone();
                
                spawn_local(async move {
                    match NotesService::get_user_notes(&user_id).await {
                        Ok(mut user_notes) => {
                            user_notes.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
                            notes.set(user_notes);
                        }
                        Err(e) => {
                            web_sys::console::error_1(&format!("Erro ao carregar notas: {}", e).into());
                        }
                    }
                    loading.set(false);
                });
            }
            || ()
        });
    }
    
    let on_logout = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            let navigator = navigator.clone();
            spawn_local(async move {
                let _ = AuthService::logout().await;
                navigator.push(&Route::Login);
            });
        })
    };
    
    let on_new_note = {
        let show_editor = show_editor.clone();
        let selected_note = selected_note.clone();
        Callback::from(move |_| {
            selected_note.set(None);
            show_editor.set(true);
        })
    };
    
    let on_select_note = {
        let selected_note = selected_note.clone();
        let show_editor = show_editor.clone();
        Callback::from(move |note: Note| {
            selected_note.set(Some(note));
            show_editor.set(true);
        })
    };
    
    let on_delete_note = {
        let notes = notes.clone();
        Callback::from(move |note_id: String| {
            let notes = notes.clone();
            spawn_local(async move {
                if let Ok(_) = NotesService::delete_note(&note_id).await {
                    notes.set(notes.iter().filter(|n| n.id.as_ref() != Some(&note_id)).cloned().collect());
                }
            });
        })
    };
    
    let on_close_editor = {
        let show_editor = show_editor.clone();
        let selected_note = selected_note.clone();
        Callback::from(move |_| {
            show_editor.set(false);
            selected_note.set(None);
        })
    };
    
    let on_save_note = {
        let notes = notes.clone();
        let show_editor = show_editor.clone();
        let selected_note = selected_note.clone();
        
        Callback::from(move |note: Note| {
            let notes = notes.clone();
            let show_editor = show_editor.clone();
            let selected_note = selected_note.clone();
            
            spawn_local(async move {
                if note.id.is_some() {
                    // Atualizar nota existente
                    if let Ok(_) = NotesService::update_note(&note).await {
                        let mut updated_notes = (*notes).clone();
                        if let Some(pos) = updated_notes.iter().position(|n| n.id == note.id) {
                            updated_notes[pos] = note;
                        }
                        updated_notes.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
                        notes.set(updated_notes);
                    }
                } else {
                    // Criar nova nota
                    if let Ok(id) = NotesService::create_note(&note).await {
                        let mut new_note = note;
                        new_note.id = Some(id);
                        let mut updated_notes = (*notes).clone();
                        updated_notes.insert(0, new_note);
                        notes.set(updated_notes);
                    }
                }
                show_editor.set(false);
                selected_note.set(None);
            });
        })
    };
    
    if user.is_none() {
        return html! {
            <div class="dashboard">
                <header class="dashboard-header">
                    <div class="header-content">
                        <h1>{ "📝 Minhas Notas" }</h1>
                    </div>
                </header>
                <main class="dashboard-main">
                    <div class="welcome-message" style="width: 100%;">
                        <h2>{ "⏳ Autenticando..." }</h2>
                        <p>{ "Redirecionando para login..." }</p>
                    </div>
                </main>
            </div>
        };
    }
    
    let user_email = user.as_ref().unwrap().email.clone();
    
    html! {
        <div class="dashboard">
            <header class="dashboard-header">
                <div class="header-content">
                    <h1>{ "📝 Minhas Notas" }</h1>
                    <div class="user-info">
                        <span>{ user_email }</span>
                        <button onclick={on_logout} class="btn-secondary">
                            { "Sair" }
                        </button>
                    </div>
                </div>
            </header>
            
            <main class="dashboard-main">
                <div class="notes-sidebar">
                    <button onclick={on_new_note} class="btn-primary btn-new-note">
                        { "+ Nova Nota" }
                    </button>
                    
                    if *loading {
                        <div class="loading">{ "Carregando notas..." }</div>
                    } else if notes.is_empty() {
                        <div class="empty-state">
                            <p>{ "Nenhuma nota ainda" }</p>
                            <p class="empty-hint">{ "Clique em 'Nova Nota' para começar" }</p>
                        </div>
                    } else {
                        <div class="notes-list">
                            { for notes.iter().map(|note| {
                                let note_clone = note.clone();
                                let note_clone2 = note.clone();
                                let on_select = on_select_note.clone();
                                let on_delete = on_delete_note.clone();
                                
                                html! {
                                    <div class="note-item">
                                        <div
                                            class="note-content"
                                            onclick={Callback::from(move |_| on_select.emit(note_clone.clone()))}
                                        >
                                            <h3>{ &note.title }</h3>
                                            <p>{ truncate(&note.content, 100) }</p>
                                            <small>{ format_date(note.updated_at) }</small>
                                        </div>
                                        <button
                                            class="btn-delete"
                                            onclick={Callback::from(move |e: MouseEvent| {
                                                e.stop_propagation();
                                                if let Some(id) = &note_clone2.id {
                                                    on_delete.emit(id.clone());
                                                }
                                            })}
                                        >
                                            { "🗑️" }
                                        </button>
                                    </div>
                                }
                            }) }
                        </div>
                    }
                </div>
                
                <div class="notes-content">
                    if *show_editor {
                        <NoteEditor
                            note={(*selected_note).clone()}
                            user_id={user.as_ref().unwrap().uid.clone()}
                            on_save={on_save_note}
                            on_close={on_close_editor}
                        />
                    } else {
                        <div class="welcome-message">
                            <h2>{ "Bem-vindo ao Notepad!" }</h2>
                            <p>{ "Selecione uma nota da lista ou crie uma nova." }</p>
                        </div>
                    }
                </div>
            </main>
        </div>
    }
}

fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len])
    }
}

fn format_date(timestamp: i64) -> String {
    let date = js_sys::Date::new(&(timestamp as f64).into());
    let day = date.get_date();
    let month = date.get_month() + 1;
    let year = date.get_full_year();
    format!("{:02}/{:02}/{}", day, month, year)
}
