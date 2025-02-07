# Webas 🚀  

**A Rust-powered project for crafting a dynamic blog-style website with Markdown and Twig templates.**  
Built as a personal learning project to explore Rust's capabilities, with an emphasis on minimalism and flexibility.

## ⚡ Proof of Concept (POC)  

To run the project, execute the following:  

```bash
cargo run -- --template tests/template --web ./tests/web
```

📁 Template Folder Structure
Your template directory must follow this structure:

```
./tests/template/  
  ├── assets/   # The entire directory is copied to the --web location  
  ├── pages/    # Contains Twig template files  
  ├── posts/    # Contains Markdown files
```

📑 Post (Markdown) Metadata Example:
Each Markdown post should include metadata at the top for proper rendering:

```
---
template_base: "parts/base.twig"  # Base Twig template for the post layout
title: "bash_oneliners.md"        # Title of the post
slug: "bash_oneliners.html"       # URL slug and output file name
date: ""                          # Static date for now
intro: ""                         # Brief intro to the post
---
Markdown content...               # Main content, rendered into the Twig template
```

🧱 Twig Template Blocks
Use Twig blocks to inject content from your Markdown files into the templates:

```
{% block title %} # Injects the "title" metadata from the Markdown  
{% block date %}  # Injects the "date" metadata from the Markdown  
{% block intro %} # Injects the "intro" metadata from the Markdown  
{% block main %}  # Renders the Markdown content as HTML
```

🔧 TODO

- Remove all hard-coded values and make the system more configurable
- Build initial POC template for a personal notes website
- Refactor the code to eliminate the dreaded unwrap calls



