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
            .map_err(|e| format!("Erro ao registrar: {:?}", e))?;
        
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
            .map_err(|e| format!("Erro ao fazer login: {:?}", e))?;
        
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
            .map_err(|e| format!("Erro ao sair: {:?}", e))?;
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
}
