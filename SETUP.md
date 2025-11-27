# 🚀 Guia Completo de Instalação e Configuração

Este documento descreve todos os passos necessários para configurar e rodar o projeto **Notepad Multiusuário** em sua máquina.

---

## 📋 Pré-requisitos

Antes de começar, certifique-se de que você tem os seguintes programas instalados:

### 1. **Git** (para clonar o repositório)
- **macOS**: Já incluído no Xcode Command Line Tools
  ```bash
  xcode-select --install
  ```
- **Windows**: Baixe em https://git-scm.com/download/win
- **Linux**: 
  ```bash
  sudo apt-get install git  # Ubuntu/Debian
  sudo yum install git      # Fedora/RHEL
  ```

### 2. **Rust** (linguagem de programação para o frontend)
- Instale em https://rustup.rs/
- **macOS/Linux**:
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  source $HOME/.cargo/env
  ```
- **Windows**: Baixe o instalador em https://rustup.rs/

Verifique a instalação:
```bash
rustc --version
cargo --version
```

### 3. **Trunk** (ferramenta de build para Yew/WASM)
- Instale via cargo:
  ```bash
  cargo install trunk
  ```

Verifique a instalação:
```bash
trunk --version
```

### 4. **Node.js e npm** (opcional, para backend futuro)
- Baixe em https://nodejs.org/ (versão LTS recomendada)

Verifique a instalação:
```bash
node --version
npm --version
```

### 5. **wasm-pack** (gerador de bindings para WebAssembly)
- Instale via cargo:
  ```bash
  cargo install wasm-pack
  ```

---

## 📁 Estrutura do Projeto

```
trabalho-de-lp-main/
├── frontend/                 # Aplicação Yew/WASM
│   ├── src/
│   │   ├── main.rs          # Entrypoint da aplicação
│   │   ├── components/      # Componentes Yew (Login, Register, Dashboard, etc.)
│   │   ├── models/          # Modelos de dados (Note, User)
│   │   └── services/        # Serviços (Auth, Notes)
│   ├── Cargo.toml           # Dependências do Rust
│   ├── Trunk.toml           # Configuração do Trunk
│   ├── index.html           # HTML de entrada
│   └── styles.css           # Estilos CSS
├── README.md                # Este arquivo
└── SETUP.md                 # Guia de instalação (este arquivo)
```

---

## 🔧 Passos de Instalação

### Passo 1: Clonar o Repositório

```bash
git clone https://github.com/SEU_USUARIO/trabalho-de-lp-main.git
cd trabalho-de-lp-main
```

### Passo 2: Navegar para o Diretório do Frontend

```bash
cd frontend
```

### Passo 3: Instalar Dependências do Rust

As dependências são definidas em `frontend/Cargo.toml`. Cargo as instalará automaticamente ao compilar:

```bash
cargo build
```

Isso pode levar alguns minutos na primeira execução (compilação completa).

### Passo 4: Rodar o Servidor de Desenvolvimento (Trunk)

```bash
trunk serve
```

O Trunk irá:
- Compilar o código Rust para WebAssembly
- Iniciar um servidor local em `http://127.0.0.1:8080`
- Abrir automaticamente o navegador (se a flag `--open` estiver habilitada em `Trunk.toml`)

### Passo 5: Acessar a Aplicação

- **URL local**: http://localhost:8080
- A aplicação abrirá automaticamente no navegador
- Você verá a página de login

---

## 🔐 Configuração do Firebase

A aplicação usa **Firebase** para autenticação e armazenamento de dados. A configuração está no arquivo `frontend/index.html`:

```html
<script>
  const firebaseConfig = {
    apiKey: "AIzaSyB6QfIESuk84fCcqyvO94JorIQFbrVshIk",
    authDomain: "notepad-rust.firebaseapp.com",
    projectId: "notepad-rust",
    storageBucket: "notepad-rust.firebasestorage.app",
    messagingSenderId: "49619333427",
    appId: "1:49619333427:web:60938a9bdd941921770e46",
    measurementId: "G-FQQWJ6N7EZ"
  };
</script>
```

**Nota**: A chave Firebase já está configurada. Se quiser usar seu próprio projeto Firebase:
1. Crie uma conta em https://firebase.google.com/
2. Crie um novo projeto
3. Copie suas credenciais e substitua no `frontend/index.html`

---

## 📦 Dependências Principais

### Frontend (Rust/Yew)
- **yew**: Framework para UI em WebAssembly
- **yew-router**: Roteamento client-side
- **wasm-bindgen**: Bindings JavaScript ↔ Rust
- **serde**: Serialização/desserialização de dados
- **gloo-net**: Cliente HTTP para fazer requisições
- **js-sys**: Acesso a APIs do JavaScript

Veja `frontend/Cargo.toml` para a lista completa.

---

## 🚀 Compilação para Produção

Para gerar uma versão otimizada para produção:

```bash
cd frontend
trunk build --release
```

Os arquivos compilados estarão em `frontend/dist/`.

---

## ❌ Troubleshooting

### Erro: "Unable to find any Trunk configuration"
- **Solução**: Execute `trunk serve` **dentro do diretório `frontend/`**, não da raiz
  ```bash
  cd frontend
  trunk serve
  ```

### Erro: "error taking the canonical path to the watch ignore path"
- **Solução**: Certifique-se de que o diretório `frontend/target` existe (ou deixe o Trunk criá-lo)
  ```bash
  mkdir -p frontend/target
  ```

### Compilação lenta na primeira vez
- **Normal**: A primeira compilação pode levar 3-5 minutos (compila todas as dependências)
- Execuções subsequentes serão muito mais rápidas

### Porta 8080 já está em uso
- **Solução**: Mude a porta em `frontend/Trunk.toml`:
  ```toml
  [serve]
  address = "127.0.0.1"
  port = 8081  # Altere para outra porta
  ```

### Problemas de permissão no macOS/Linux
- **Solução**: Ajuste permissões da pasta do projeto
  ```bash
  chmod -R u+w .
  ```

---

## 🔄 Workflow de Desenvolvimento

1. **Navegar para o frontend**:
   ```bash
   cd frontend
   ```

2. **Rodar o servidor de desenvolvimento**:
   ```bash
   trunk serve
   ```

3. **Editar código** (os arquivos em `src/` se recompilam automaticamente):
   - Modifique qualquer arquivo em `src/components/`, `src/services/`, etc.
   - O Trunk detectará mudanças e recompilará automaticamente
   - Atualize o navegador (ou use o hot reload se habilitado)

4. **Ver logs**:
   - Abra a aba "Console" das Developer Tools do navegador (F12 ou Cmd+Option+I)
   - Erros de Rust/WASM e logs aparecem ali

---

## 🆘 Suporte e Documentação

- **Rust**: https://doc.rust-lang.org/
- **Yew**: https://yew.rs/
- **Trunk**: https://trunkrs.io/
- **Firebase**: https://firebase.google.com/docs
- **WebAssembly**: https://webassembly.org/

---

## ✅ Verificação Final

Após seguir os passos acima, você deve ter:

- ✅ Git, Rust, Trunk e Node.js instalados
- ✅ Projeto clonado localmente
- ✅ Servidor de desenvolvimento rodando em `http://localhost:8080`
- ✅ Aplicação Yew carregando no navegador
- ✅ Página de login visível

Se tudo estiver funcionando, parabéns! 🎉 Você está pronto para desenvolver.

---

**Última atualização**: 26 de novembro de 2025
