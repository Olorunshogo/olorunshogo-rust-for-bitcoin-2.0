# Rust for Bitcoin Program 2.0

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![Bitcoin](https://img.shields.io/badge/Bitcoin-F7931A?style=for-the-badge&logo=bitcoin&logoColor=white)
![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)

> A hands-on training program for building Bitcoin applications using Rust and contributing to the Bitcoin open-source ecosystem.

---

## 📁 Repository Structure

```
.
├── .github/          # GitHub workflows and CI/CD configurations
├── rfb_labs_week_1/  # Week 1 labs and exercises
│   ├── grader/       # Grading scripts and evidence checking
│   ├── src/          # Source code for labs
│   │   ├── labs/     # Individual lab implementations (lab01-lab10)
│   │   ├── error.rs  # Error handling
│   │   ├── lib.rs    # Library entry point
│   │   ├── model.rs  # Data models
│   │   └── rpc.rs    # RPC client implementation
│   ├── submissions/  # Lab submission templates
│   ├── tests/        # Lab tests and support utilities
│   ├── Cargo.toml    # Rust project configuration
│   ├── Cargo.lock    # Dependency lock file
│   ├── LABS.md       # Lab documentation
│   └── README.md     # Week 1 specific README
├── rfb_labs_week_2/  # Week 2 Bitcoin transaction modelling assignment
│   ├── src/          # Student implementation files
│   │   ├── lib.rs    # Library entry point and public exports
│   │   ├── transaction.rs # Transaction types, methods, traits, and borrowing
│   │   ├── error.rs  # Custom transaction errors
│   │   ├── utxo.rs   # UTXO model and coin selection
│   │   └── main.rs   # Payment transaction example
│   ├── tests/        # Starter integration tests
│   │   ├── transaction.rs # Transaction and validation tests
│   │   └── utxo.rs   # UTXO selection tests
│   ├── Cargo.toml    # Rust package configuration
│   ├── Cargo.lock    # Locked dependency versions
│   ├── ASSIGNMENT.md # Ordered requirements and progress checklist
│   └── README.md     # Week 2 workflow and written-answer template
├── rfb_labs_week_2_session_4/ # Session 4 lending-library assignment (pure Rust)
│   ├── src/          # Student implementation files
│   │   ├── lib.rs    # Library entry point and public exports
│   │   ├── catalogue.rs # Media kinds, loan status, items, and the LoanTerms trait
│   │   ├── member.rs # Library members
│   │   ├── library.rs # Owning collection, lookups, checkout, and return
│   │   ├── error.rs  # Custom library errors
│   │   └── main.rs   # Lending example
│   ├── tests/        # Starter integration tests
│   │   └── library.rs # Checkout, borrow limit, late fee, and search tests
│   ├── Cargo.toml    # Rust package configuration
│   ├── ASSIGNMENT.md # Ordered requirements and progress checklist
│   └── README.md     # Session 4 workflow and written-answer template
├── .gitignore        # Git ignore patterns
├── README.md         # This file
└── x.sh              # Setup/utility script
```

### Directory Descriptions

- **`.github/`** - GitHub Actions workflows for automated grading and CI/CD
- **`rfb_labs_week_1/`** - Week 1 lab exercises covering Bitcoin fundamentals and Rust basics
  - **`grader/`** - Automated grading scripts for evaluating lab submissions
  - **`src/`** - Source code including lab implementations and supporting modules
  - **`submissions/`** - Templates for participants to submit their lab evidence
  - **`tests/`** - Unit tests for each lab to verify implementation correctness
- **`rfb_labs_week_2/`** - Week 2 assignment for modelling a Bitcoin transaction in Rust
  - **`src/`** - Starter implementation, organized by transaction, error, and UTXO concerns
  - **`tests/`** - Staged integration tests that students enable as they progress
  - **`ASSIGNMENT.md`** - Requirements in implementation order
  - **`README.md`** - Student workflow, commands, and written questions
- **`rfb_labs_week_2_session_4/`** - Session 4 assignment on enums, structs, traits, ownership, borrowing, and error handling, deliberately not Bitcoin-themed so the Rust concepts stand alone
  - **`src/`** - Starter implementation, organized by catalogue, member, library, and error concerns
  - **`tests/`** - Staged integration tests that students enable as they progress
  - **`ASSIGNMENT.md`** - Requirements in implementation order
  - **`README.md`** - Student workflow, commands, and written questions

---

## 📖 Technical Resources

- 🦀 **The Rust Book** - Comprehensive Rust programming guide
- 📄 **Bitcoin Whitepaper** - Original Bitcoin protocol specification
- 🔄 **Bitcoin Improvement Proposals (BIPs)** - Protocol standards and proposals
- 🔧 **Rust Bitcoin Documentation** - Library documentation and examples
- ⛏️ **Bitcoin Core Documentation** - Core implementation reference

---

## 🌟 Open Source Contributions

The major goal of this program is to contribute meaningfully to Bitcoin open-source projects. Participants will:

- 🔄 Learn Git workflow and best practices
- 📤 Create and submit Pull Requests
- 👁️ Participate in code reviews
- 📖 Read and understand BIPs
- 💬 Join Bitcoin Core Review Club discussions
- 🤝 Collaborate with global Bitcoin developers
- 💰 Explore grant opportunities for active contributors

### Open Source Etiquette

- Follow project contribution guidelines
- Write clear commit messages
- Engage constructively in code reviews
- Respect maintainers' time and decisions

---


##  Capstone Project

All participants will complete a capstone project demonstrating their skills and knowledge gained throughout the program.

### Project Options

- **Bitcoin Wallet** - Full-featured wallet implementation
- **Lightning Tool** - Lightning Network utility or application
- **Bitcoin CLI** - Command-line interface for Bitcoin operations
- **Bitcoin Indexer** - Blockchain data indexing and analysis
- **Transaction Explorer** - Transaction visualization and analysis
- **Block Explorer** - Blockchain block exploration interface
- **Wallet Library** - Reusable wallet functionality library

### Final Deliverables

- 📦 GitHub Repository with complete source code
- 📖 Comprehensive README with documentation
- 🏗️ Architecture Diagram
- 🎥 Demo Video
- 🎤 Live Presentation during Demo Day

---


## 🛠️ Prerequisites

Before starting the program, ensure you have:

- 🦀 **Rust** installed (latest stable version)
- 📦 **Cargo** (comes with Rust)
- 🔄 **Git** version control
- 🌐 **GitHub Account** (configured with SSH keys)
- 🐳 **Docker** for containerized development
- ⚡ **Polar** for Bitcoin regtest networks
- 💻 **VS Code** (recommended IDE)

---

## 🚀 Getting Started

### Clone the Repository

```bash
git clone https://github.com/thebuidl-grid/rust-for-bitcoin-2.0.git
cd rust-for-bitcoin-2.0
```

### Build the Project

```bash
cargo build
```

### Run Tests

```bash
cargo test
```

### Setup Development Environment

See [`rfb_labs_week_1/README.md`](rfb_labs_week_1/README.md) for detailed environment setup instructions.

For the transaction modelling assignment, see
[`rfb_labs_week_2/README.md`](rfb_labs_week_2/README.md).

---

## 🤝 Contributing

We welcome contributions! Follow these steps:

1. **Fork** the repository
2. **Create** a feature branch (`git checkout -b feature/amazing-feature`)
3. **Commit** your changes (`git commit -m 'Add amazing feature'`)
4. **Push** to the branch (`git push origin feature/amazing-feature`)
5. **Open** a Pull Request

### Contribution Guidelines

- Write clear, descriptive commit messages
- Include tests for new features
- Update documentation as needed
- Follow Rust coding conventions
- Ensure all tests pass before submitting

---

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---
