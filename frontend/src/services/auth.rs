// src/services/auth.rs
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use js_sys::{Reflect, Function};
use crate::models::note::User;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window, js_name = auth)]
    #[wasm_bindgen(thread_local_v2)]
    pub static AUTH: JsValue;
}

pub struct AuthService;

impl AuthService {
    pub async fn register(email: &str, password: &str) -> Result<User, String> {
        let email_val = JsValue::from_str(email);
        let password_val = JsValue::from_str(password);
        
        let promise_res = AUTH.with(|auth| {
            Reflect::get(auth, &JsValue::from_str("createUserWithEmailAndPassword")).and_then(|f| {
                let func: Function = f.into();
                func.call2(auth, &email_val, &password_val)
            })
        });

        let promise = promise_res.map_err(|e| format!("Erro ao chamar função: {:?}", e))?;
        
        let result = JsFuture::from(js_sys::Promise::from(promise)).await
            .map_err(|e| Self::map_auth_error(&e, "Erro ao registrar"))?;
        
        Self::parse_user_from_credential(&result)
    }
    
    pub async fn login(email: &str, password: &str) -> Result<User, String> {
        let email_val = JsValue::from_str(email);
        let password_val = JsValue::from_str(password);
        
        let promise_res = AUTH.with(|auth| {
            Reflect::get(auth, &JsValue::from_str("signInWithEmailAndPassword")).and_then(|f| {
                let func: Function = f.into();
                func.call2(auth, &email_val, &password_val)
            })
        });

        let promise = promise_res.map_err(|e| format!("Erro ao chamar função: {:?}", e))?;
        
        let result = JsFuture::from(js_sys::Promise::from(promise)).await
            .map_err(|e| Self::map_auth_error(&e, "Erro ao fazer login"))?;
        
        Self::parse_user_from_credential(&result)
    }
    
    pub async fn logout() -> Result<(), String> {
        let promise_res = AUTH.with(|auth| {
            Reflect::get(auth, &JsValue::from_str("signOut")).and_then(|f| {
                let func: Function = f.into();
                func.call1(auth, &JsValue::null())
            })
        });

        let promise = promise_res.map_err(|e| format!("Erro ao chamar função: {:?}", e))?;
        
        JsFuture::from(js_sys::Promise::from(promise)).await
            .map_err(|e| Self::map_auth_error(&e, "Erro ao sair"))?;
        Ok(())
    }
    
    pub fn get_current_user() -> Option<User> {
        let user = AUTH.with(|auth| Reflect::get(auth, &JsValue::from_str("currentUser"))).ok()?;
        
        if user.is_null() || user.is_undefined() {
            return None;
        }
        
        let uid = Reflect::get(&user, &JsValue::from_str("uid")).ok()?;
        let email = Reflect::get(&user, &JsValue::from_str("email")).ok()?;
        
        Some(User {
            uid: uid.as_string()?,
            email: email.as_string()?,
        })
    }
    
    fn parse_user_from_credential(cred: &JsValue) -> Result<User, String> {
        let user_obj = js_sys::Reflect::get(cred, &"user".into())
            .map_err(|_| "Não foi possível obter dados do usuário")?;
        
        let uid = js_sys::Reflect::get(&user_obj, &"uid".into())
            .map_err(|_| "UID não encontrado")?
            .as_string()
            .ok_or("UID inválido")?;
        
        let email = js_sys::Reflect::get(&user_obj, &"email".into())
            .map_err(|_| "Email não encontrado")?
            .as_string()
            .ok_or("Email inválido")?;
        
        Ok(User { uid, email })
    }

    fn map_auth_error(err: &JsValue, prefix: &str) -> String {
        // Try to extract Firebase-style `code` and `message` fields from the error JsValue
        let code = js_sys::Reflect::get(err, &JsValue::from_str("code")).ok()
            .and_then(|v| v.as_string());
        let message = js_sys::Reflect::get(err, &JsValue::from_str("message")).ok()
            .and_then(|v| v.as_string());

        if let Some(code) = code {
            // Map common Firebase auth codes to friendly Portuguese messages
            let friendly = match code.as_str() {
                "auth/wrong-password" => "Senha incorreta. Verifique e tente novamente.",
                "auth/user-not-found" => "Usuário não encontrado. Verifique o email cadastrado.",
                "auth/invalid-email" => "Formato de email inválido.",
                "auth/email-already-in-use" => "Este email já está em uso.",
                "auth/weak-password" => "Senha muito fraca. Use pelo menos 6 caracteres.",
                "auth/invalid-credential" => "Credenciais incorretas, tente novamente!",
                _ => message.as_deref().unwrap_or(&code),
            };
            format!("{}: {}", prefix, friendly)
        } else if let Some(msg) = message {
            format!("{}: {}", prefix, msg)
        } else {
            format!("{}: erro desconhecido", prefix)
        }
    }
}
