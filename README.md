# Webas 🚀  

**A Rust-powered project for building a blog-style website using Markdown and Twig templates.**  
This is a personal project created while learning Rust, focused on simplicity and flexibility.

## ⚡ Proof of Concept  

Run the project with:  

```bash
cargo run -- --template tests/template --web ./tests/web
```

📁 Template Folder Structure
Your template directory should follow this structure:

```
./tests/template/  
  ├── assets/   # This entire directory is copied to the --web location  
  ├── pages/    # Contains Twig template files  
  ├── posts/    # Contains Markdown files
```

🔧 TODO

- [ ] Implement a configuration system
- [ ] Abstract configuration handling for better modularity
