// src/components/login.rs
use yew::prelude::*;
use yew_router::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::services::auth::AuthService;
use crate::Route;

#[function_component(Login)]
pub fn login() -> Html {
    let navigator = use_navigator().unwrap();
    let email = use_state(|| String::new());
    let password = use_state(|| String::new());
    let error = use_state(|| None::<String>);
    let loading = use_state(|| false);
    
    let on_email_change = {
        let email = email.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            email.set(input.value());
        })
    };
    
    let on_password_change = {
        let password = password.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            password.set(input.value());
        })
    };
    
    let on_submit = {
        let email = email.clone();
        let password = password.clone();
        let error = error.clone();
        let loading = loading.clone();
        let navigator = navigator.clone();
        
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            
            let email_val = (*email).clone();
            let password_val = (*password).clone();
            let error = error.clone();
            let loading = loading.clone();
            let navigator = navigator.clone();
            
            loading.set(true);
            
            spawn_local(async move {
                match AuthService::login(&email_val, &password_val).await {
                    Ok(_) => {
                        navigator.push(&Route::Dashboard);
                    }
                    Err(e) => {
                        error.set(Some(e));
                        loading.set(false);
                    }
                }
            });
        })
    };
    
    let go_to_register = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            navigator.push(&Route::Register);
        })
    };
    
    html! {
        <div class="auth-container">
            <div class="auth-card">
                <h1 class="auth-title">{ "📝 Notepad Multiusuário" }</h1>
                <p class="auth-subtitle">{ "Faça login para acessar suas notas" }</p>
                
                <form onsubmit={on_submit}>
                    <div class="form-group">
                        <label for="email">{ "Email" }</label>
                        <input
                            type="email"
                            id="email"
                            value={(*email).clone()}
                            oninput={on_email_change}
                            placeholder="seu@email.com"
                            required={true}
                            disabled={*loading}
                        />
                    </div>
                    
                    <div class="form-group">
                        <label for="password">{ "Senha" }</label>
                        <input
                            type="password"
                            id="password"
                            value={(*password).clone()}
                            oninput={on_password_change}
                            placeholder="••••••••"
                            required={true}
                            disabled={*loading}
                        />
                    </div>
                    
                    if let Some(err) = (*error).as_ref() {
                        <div class="error-message">
                            { err }
                        </div>
                    }
                    
                    <button type="submit" class="btn-primary" disabled={*loading}>
                        { if *loading { "Entrando..." } else { "Entrar" } }
                    </button>
                </form>
                
                <div class="auth-footer">
                    <p>{ "Não tem uma conta? " }</p>
                    <button onclick={go_to_register} class="btn-link">
                        { "Cadastre-se" }
                    </button>
                </div>
            </div>
        </div>
    }
}
