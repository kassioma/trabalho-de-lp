// src/services/auth.rs
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use js_sys::{Reflect, Function};
use crate::models::note::User;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window, js_name = auth)]
    pub static AUTH: JsValue;
}

pub struct AuthService;

impl AuthService {
    pub async fn register(email: &str, password: &str) -> Result<User, String> {
        let email_val = JsValue::from_str(email);
        let password_val = JsValue::from_str(password);
        
        let create_user_fn = Reflect::get(&AUTH, &JsValue::from_str("createUserWithEmailAndPassword"))
            .map_err(|_| "Função não encontrada")?;
        let create_user_fn: Function = create_user_fn.into();
        
        let promise = create_user_fn.call2(&AUTH, &email_val, &password_val)
            .map_err(|e| format!("Erro ao chamar função: {:?}", e))?;
        
        let result = JsFuture::from(js_sys::Promise::from(promise)).await
            .map_err(|e| format!("Erro ao registrar: {:?}", e))?;
        
        Self::parse_user_from_credential(&result)
    }
    
    pub async fn login(email: &str, password: &str) -> Result<User, String> {
        let email_val = JsValue::from_str(email);
        let password_val = JsValue::from_str(password);
        
        let sign_in_fn = Reflect::get(&AUTH, &JsValue::from_str("signInWithEmailAndPassword"))
            .map_err(|_| "Função não encontrada")?;
        let sign_in_fn: Function = sign_in_fn.into();
        
        let promise = sign_in_fn.call2(&AUTH, &email_val, &password_val)
            .map_err(|e| format!("Erro ao chamar função: {:?}", e))?;
        
        let result = JsFuture::from(js_sys::Promise::from(promise)).await
            .map_err(|e| format!("Erro ao fazer login: {:?}", e))?;
        
        Self::parse_user_from_credential(&result)
    }
    
    pub async fn logout() -> Result<(), String> {
        let sign_out_fn = Reflect::get(&AUTH, &JsValue::from_str("signOut"))
            .map_err(|_| "Função não encontrada")?;
        let sign_out_fn: Function = sign_out_fn.into();
        
        let promise = sign_out_fn.call1(&AUTH, &JsValue::null())
            .map_err(|e| format!("Erro ao chamar função: {:?}", e))?;
        
        JsFuture::from(js_sys::Promise::from(promise)).await
            .map_err(|e| format!("Erro ao sair: {:?}", e))?;
        Ok(())
    }
    
    pub fn get_current_user() -> Option<User> {
        let user = Reflect::get(&AUTH, &JsValue::from_str("currentUser"))
            .ok()?;
        
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
}
