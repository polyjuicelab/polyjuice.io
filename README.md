# polyjuice.io

A frontend project that supports x402 protocol and can mimic any Farcaster user through SnapRAG integration.

## 🎨 About This Project

This is a minimalist web application featuring Yves Klein's "Blue Monochrome" artwork. The site showcases our projects and philosophy with a clean, modern design.

### Features

- **Responsive Design**: Optimized for all screen sizes with golden ratio proportions
- **Interactive Navigation**: Click the artwork or scroll down to access project information
- **Smooth Animations**: Subtle rainbow background animation and smooth scrolling
- **Modern UI**: Clean typography and minimalist layout
- **Project Showcase**: Display of our open-source projects with detailed descriptions

## 🚀 Quick Start

### Prerequisites

- Rust (nightly toolchain)
- Trunk (Rust WASM build tool)

### Installation

1. Clone the repository:
```bash
git clone https://github.com/your-org/polyjuice.io.git
cd polyjuice.io
```

2. Install Trunk:
```bash
cargo install trunk
```

3. Serve the development server:
```bash
make serve-dev
```

4. Open your browser and navigate to `http://127.0.0.1:8081`

## 🛠️ Development

### Available Commands

- `make serve-dev` - Start development server on localhost:8081
- `make serve` - Start production server on localhost:8080
- `make build` - Build the project for production
- `make check` - Run cargo check
- `make clean` - Clean build artifacts

### Project Structure

```
polyjuice.io/
├── src/
│   └── main.rs          # Main Yew application
├── imgs/                # Static assets
│   ├── blue.jpg         # Yves Klein artwork
│   ├── logo.png         # polyjuice.io logo
│   ├── favicon.png      # Site favicon
│   ├── rings.png        # Rings project logo
│   └── castorix.png     # Castorix project logo
├── index.html           # HTML entry point
├── Cargo.toml          # Rust dependencies
├── Trunk.toml          # Trunk configuration
├── rust-toolchain      # Rust toolchain specification
├── Makefile            # Build automation
└── README.md          # This file
```

## 🎯 Our Projects

### Rings
**P2P network with WebRTC & WASM**

A decentralized peer-to-peer networking library built with Rust, featuring WebRTC for real-time communication and WebAssembly for cross-platform compatibility. Designed for high-performance, low-latency applications.

[GitHub Repository](https://github.com/0xBaseAI/rings)

### Castorix
**Farcaster protocol library**

A comprehensive Rust implementation of the Farcaster protocol, providing secure and efficient tools for building decentralized social applications. Features include cryptographic signatures, message validation, and network synchronization.

[GitHub Repository](https://github.com/0xBaseAI/castorix)

### SnapRAG
**Farcaster data synchronization system**

A high-performance data synchronization system designed specifically for Farcaster protocol data, optimized for RAG applications.

[GitHub Repository](https://github.com/0xBaseAI/snaprag)

### x402
**Payments protocol for the internet**

A payments protocol for the internet built on HTTP, providing a standardized way to handle payments in web applications.

[GitHub Repository](https://github.com/RyanKung/x402)

### Polyjuice
**Frontend project supporting x402**

A frontend project that supports x402 protocol and can mimic any Farcaster user through SnapRAG integration.

[GitHub Repository](https://github.com/0xBaseAI/polyjuice)

## 🎨 Design Philosophy

This website embodies our core principles:

- **Minimalism**: Clean, uncluttered design focusing on essential elements
- **Aesthetic Excellence**: Inspired by MoMA's design language and Yves Klein's artistic vision
- **Technical Precision**: Built with Rust for performance and security
- **Open Source**: All code is publicly available and auditable

## 🔧 Technology Stack

- **Frontend**: Yew (Rust WebAssembly framework)
- **Build Tool**: Trunk
- **Styling**: CSS3 with modern features (Flexbox, Grid, CSS Variables)
- **Deployment**: Static site generation

## 📝 License

This project is licensed under a proprietary license that prohibits any form of usage. All rights reserved.

## 🤝 Contributing

We welcome contributions from the community. Please ensure all code follows our standards:

- Written in Rust
- Fully documented
- Security-focused
- Performance-optimized

## 📞 Contact

- **Organization**: polyjuice.io
- **Focus**: Frontend supporting x402 protocol
- **Philosophy**: Security-obsessed, open source

---

*Built with ❤️ and Rust by the polyjuice.io team*