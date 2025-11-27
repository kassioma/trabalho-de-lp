# Frontend — Aplicação Yew/WASM

Este diretório contém a aplicação frontend do **Notepad Multiusuário**, um bloco de notas moderno compilado para **WebAssembly**.

---

## 🚀 Quick Start

```bash
# Dentro do diretório frontend
cd frontend

# Rodar servidor de desenvolvimento
trunk serve

# Acessar em http://localhost:8080
```

---

## 📂 Estrutura

```
src/
├── main.rs                  # Entrypoint da aplicação Yew
├── components/
│   ├── mod.rs              # Módulo de componentes
│   ├── login.rs            # Tela de login
│   ├── register.rs         # Tela de registro
│   ├── dashboard.rs        # Dashboard principal
│   └── note_editor.rs      # Editor de notas
├── models/
│   ├── mod.rs
│   └── note.rs             # Estruturas: Note, User
└── services/
    ├── mod.rs
    ├── auth.rs             # Serviço de autenticação (Firebase)
    └── notes.rs            # Serviço de notas (Firestore)

Cargo.toml                  # Dependências Rust
Trunk.toml                  # Configuração do Trunk
index.html                  # HTML de entrada (com Firebase SDK)
styles.css                  # Estilos CSS
```

---

## 🔧 Configuração

### Variáveis de Ambiente

Não há variáveis de ambiente obrigatórias neste projeto. As credenciais do Firebase estão hardcoded em `index.html` (consideradas seguras para aplicações frontend públicas).

Se quiser usar suas próprias credenciais Firebase:

1. Crie um projeto em https://console.firebase.google.com/
2. Copie suas credenciais (Web)
3. Substitua os valores em `frontend/index.html`:
   ```html
   const firebaseConfig = {
     apiKey: "YOUR_API_KEY",
     authDomain: "YOUR_AUTH_DOMAIN",
     projectId: "YOUR_PROJECT_ID",
     storageBucket: "YOUR_STORAGE_BUCKET",
     messagingSenderId: "YOUR_MESSAGING_SENDER_ID",
     appId: "YOUR_APP_ID",
     measurementId: "YOUR_MEASUREMENT_ID"
   };
   ```

---

## 📦 Dependências

As dependências principais estão em `Cargo.toml`:

- **yew** (0.21) — Framework para WebAssembly
- **yew-router** (0.18) — Roteamento client-side
- **wasm-bindgen** (0.2) — Bindings Rust ↔ JavaScript
- **wasm-bindgen-futures** (0.4) — Suporte para Promises
- **web-sys** (0.3) — APIs do navegador
- **js-sys** (0.3) — Tipos JavaScript
- **serde** (1.0) — Serialização de dados
- **serde_json** (1.0) — JSON parsing
- **gloo-net** (0.5) — Cliente HTTP
- **gloo-storage** (0.3) — Acesso ao localStorage

---

## 🔨 Comandos

### Desenvolvimento

```bash
# Rodar servidor com hot reload
trunk serve

# Servir em porta específica
trunk serve --address 127.0.0.1 --port 3000

# Sem abrir navegador automaticamente
trunk serve --open false
```

### Build

```bash
# Build de desenvolvimento
trunk build

# Build otimizado para produção
trunk build --release

# Arquivos compilados em ./dist/
```

### Limpeza

```bash
# Remover artefatos compilados
trunk clean

# Remover apenas WASM
cargo clean --release
```

---

## 🎨 Componentes

### Login (`components/login.rs`)
- Entrada de email e senha
- Validação básica
- Integração com Firebase Auth
- Redirecionamento para Dashboard ou Register

### Register (`components/register.rs`)
- Registro de novo usuário
- Confirmação de senha
- Validações (min 6 caracteres)
- Integração com Firebase Auth

### Dashboard (`components/dashboard.rs`)
- Lista de notas do usuário
- Botão para nova nota
- Seleção e edição de notas
- Botão de logout

### NoteEditor (`components/note_editor.rs`)
- Editor inline de título e conteúdo
- Contador de caracteres
- Buttons de Salvar/Cancelar
- Suporta criação e edição

---

## 📡 Serviços

### AuthService (`services/auth.rs`)
Gerencia autenticação com Firebase:
- `register(email, password)` — Registra novo usuário
- `login(email, password)` — Faz login
- `logout()` — Desconecta usuário
- `get_current_user()` — Retorna usuário autenticado

### NotesService (`services/notes.rs`)
Gerencia notas em Firestore:
- `create_note(note)` — Cria nova nota
- `get_user_notes(user_id)` — Carrega notas do usuário
- `update_note(note)` — Atualiza nota existente
- `delete_note(note_id)` — Deleta nota

---

## 🐛 Troubleshooting

### "Unable to find any Trunk configuration"
```bash
# Certifique-se de estar dentro de frontend/
cd frontend
trunk serve
```

### Porta já está em uso
```bash
# Use outra porta
trunk serve --port 3000
```

### Compilação lenta
- Primeira compilação é normal (3-5 min)
- Garanta 4GB+ de RAM disponível
- Use `trunk build --release` para build final otimizado

### Erros de Firebase
- Verifique as credenciais em `index.html`
- Certifique-se de que Firebase está inicializado antes de usar
- Veja console do navegador (F12) para logs detalhados

---

## 🚀 Deploy

Para fazer deploy da aplicação:

1. **Build para produção**:
   ```bash
   trunk build --release
   ```

2. **Faça upload dos arquivos em `dist/`** para seu host:
   - Netlify (recomendado para SPAs)
   - Vercel
   - GitHub Pages
   - AWS S3 + CloudFront
   - Qualquer servidor web estático

3. **Exemplo com Netlify CLI**:
   ```bash
   npm install -g netlify-cli
   netlify deploy --prod --dir dist/
   ```

---

## 📚 Recursos

- [Documentação Yew](https://yew.rs/)
- [Trunk Docs](https://trunkrs.io/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [WebAssembly MDN](https://developer.mozilla.org/en-US/docs/WebAssembly/)
- [Firebase Web SDK](https://firebase.google.com/docs/web)

---

**Última atualização**: 26 de novembro de 2025
