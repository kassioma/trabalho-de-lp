# 📝 Notepad Multiusuário

Um aplicativo moderno e seguro de bloco de notas multiusuário construído com **Rust**, **Yew** (framework WebAssembly) e **Firebase**.

---

## ✨ Características

- 🔐 **Autenticação segura** com Firebase Authentication
- 📝 **Notas pessoais** armazenadas em Firestore
- 🔄 **Sincronização em tempo real** entre dispositivos
- 🎨 **Interface moderna e responsiva**
- ⚡ **Rápido** — compilado para WebAssembly
- 🌍 **Multiplataforma** — funciona em qualquer navegador moderno

---

## 🚀 Quick Start

### Pré-requisitos
- Git
- Rust (via rustup)
- Trunk (para build do Yew)

### Instalação

1. **Clone o repositório**:
   ```bash
   git clone https://github.com/SEU_USUARIO/trabalho-de-lp-main.git
   cd trabalho-de-lp-main
   ```

2. **Navegue para o frontend**:
   ```bash
   cd frontend
   ```

3. **Rode o servidor de desenvolvimento**:
   ```bash
   trunk serve
   ```

4. **Abra no navegador**:
   - A aplicação abrirá automaticamente em `http://localhost:8080`
   - Crie uma conta ou faça login com suas credenciais

---

## 📋 Estrutura do Projeto

```
trabalho-de-lp-main/
├── frontend/                 # Aplicação Yew/WASM
│   ├── src/
│   │   ├── main.rs          # Entrypoint
│   │   ├── components/      # Componentes (Login, Register, Dashboard, etc.)
│   │   ├── models/          # Estruturas de dados (Note, User)
│   │   └── services/        # Serviços (Auth, Notes)
│   ├── Cargo.toml           # Dependências Rust
│   ├── Trunk.toml           # Configuração Trunk
│   ├── index.html           # Arquivo HTML de entrada
│   └── styles.css           # Estilos CSS
├── backend/                  # Backend (espaço reservado)
│   └── README.md
├── README.md                # Este arquivo
├── SETUP.md                 # Guia detalhado de instalação
└── .gitignore              # Arquivo de exclusão Git
```

---

## 🛠️ Guia de Instalação Completo

Para instruções detalhadas sobre como configurar o projeto em sua máquina (incluindo instalação de dependências, troubleshooting e workflow de desenvolvimento), consulte o arquivo **[SETUP.md](./SETUP.md)**.

---

## 🔐 Autenticação e Dados

A aplicação usa **Firebase** para:
- **Authentication**: Registro e login de usuários
- **Firestore**: Armazenamento de notas (persistência em nuvem)

As credenciais do Firebase já estão configuradas em `frontend/index.html`. Você pode usar uma conta de teste ou criar a sua própria.

---

## 📦 Stack Tecnológico

| Tecnologia | Versão | Propósito |
|-----------|--------|----------|
| Rust | Latest | Linguagem de programação |
| Yew | 0.21 | Framework para WebAssembly/UI |
| Trunk | 0.21+ | Build tool para Yew |
| Firebase | 10.7 | Backend/Autenticação |
| CSS3 | Latest | Estilização |

---

## 🚀 Como Contribuir

1. Crie um fork do projeto
2. Crie uma branch para sua feature (`git checkout -b feature/AmazingFeature`)
3. Commit suas mudanças (`git commit -m 'Add some AmazingFeature'`)
4. Push para a branch (`git push origin feature/AmazingFeature`)
5. Abra um Pull Request

---

## 🐛 Reportar Bugs

Se encontrar um bug, abra uma issue no GitHub descrevendo:
- O comportamento esperado vs o atual
- Passos para reproduzir
- Seu sistema operacional e versão do navegador

---

## 📄 Licença

Este projeto está sob a licença MIT. Veja o arquivo LICENSE para detalhes.

---

## 👤 Autores

Erick William Marques Costa
Luca Valderramos Cirino
Lucas Silva Carneiro
Kássio Medeiros Alves

---

## 🔗 Links Úteis

- [Documentação do Rust](https://doc.rust-lang.org/)
- [Documentação do Yew](https://yew.rs/)
- [Documentação do Trunk](https://trunkrs.io/)
- [Firebase Console](https://console.firebase.google.com/)
- [MDN Web Docs](https://developer.mozilla.org/pt-BR/)

---

**Última atualização**: 26 de novembro de 2025
