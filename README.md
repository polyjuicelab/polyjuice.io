# polyjuice.io

A minimalist landing page showcasing polyjuice.io projects and philosophy.

## 🎨 About

This is a simple landing page built with Rust and Yew, featuring Yves Klein's "Blue Monochrome" artwork. The site showcases our open-source projects with a clean, modern design.

## 🚀 Quick Start

### Prerequisites

- Rust (nightly toolchain)
- Trunk (Rust WASM build tool)

### Development

1. Install Trunk:
```bash
cargo install trunk
```

2. Serve the development server:
```bash
make serve-dev
```

3. Open your browser and navigate to `http://127.0.0.1:8081`

### Build

Build for production:
```bash
make build
```

## 🛠️ Available Commands

- `make serve-dev` - Start development server on localhost:8081
- `make serve` - Start production server on localhost:8080
- `make build` - Build the project for production
- `make check` - Run cargo check
- `make clean` - Clean build artifacts

## 🔧 Technology Stack

- **Frontend**: Yew (Rust WebAssembly framework)
- **Build Tool**: Trunk
- **Styling**: CSS3

## 📝 License

This project is licensed under a proprietary license that prohibits any form of usage. All rights reserved.

---

*Built with ❤️ and Rust by the polyjuice.io team*
