# np - The Universal Package Manager Proxy 🚀

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg?style=for-the-badge)](LICENSE)

> **Stop memorizing commands. Start building.**

`np` is a blazingly fast, intelligent package manager proxy that automatically detects and forwards commands to the right package manager (npm, yarn, or pnpm) in your project. Write once, run anywhere.

## 🤔 The Problem

Ever switched between projects and typed the wrong command?

```bash
# In a pnpm project
$ npm install axios
# ❌ Wrong package manager!

# In a yarn project  
$ pnpm add lodash
# ❌ Wrong again!
```

Managing multiple projects with different package managers is **painful**. You need to:
- ✋ Remember which package manager each project uses
- ✋ Check for lock files before running commands
- ✋ Context-switch between npm, yarn, and pnpm commands

## ✨ The Solution

**`np` does the thinking for you.**

```bash
# Works everywhere, automatically
$ np install axios
$ np add lodash
$ np run dev
$ np --version
```

One command to rule them all. `np` automatically:
- 🔍 Detects the package manager from `package.json` or lock files
- ⚡ Forwards all commands and arguments seamlessly
- 💾 Optionally saves your choice for future runs
- 🚀 Runs with near-zero overhead (written in Rust)

## 🎯 Features

- **🧠 Smart Detection**: Reads `packageManager` field in `package.json` or scans lock files
- **⚡ Zero Config**: Works out of the box, no setup required
- **🔄 Full Compatibility**: All npm/yarn/pnpm commands work exactly as expected
- **💪 Blazingly Fast**: Built with Rust for maximum performance
- **🎨 Interactive Mode**: Prompts you when multiple package managers are detected
- **💾 Persistent Choices**: Save your selection to `package.json` automatically
- **🔧 Battle-Tested**: Comprehensive test suite with 34+ tests

## 📦 Installation

### Using Cargo (Recommended)

```bash
cargo install np-cli
```

### From Source

```bash
git clone https://github.com/jordyfontoura/np.git
cd np
cargo build --release
sudo mv target/release/np /usr/local/bin/
```

### Pre-built Binaries

Download the latest binary from [Releases](https://github.com/jordyfontoura/np/releases)

## 🚀 Quick Start

Once installed, just replace your package manager command with `np`:

```bash
# Instead of:
npm install
yarn install  
pnpm install

# Just use:
np install
```

That's it! `np` will figure out the rest.

## 💡 Usage

### Basic Commands

All your favorite commands work exactly the same:

```bash
# Installing dependencies
np install
np add axios
np add -D typescript

# Running scripts
np run dev
np run build
np test

# Getting info
np --version
np list
```

### First-Time Setup in a Project

When `np` can't determine the package manager, it will ask you:

```bash
$ np install

🤔 Could not determine the package manager.
? Which package manager would you like to use? 
  > npm
    yarn
    pnpm
    
? Would you like to save this choice in the 'packageManager' field in package.json? (Y/n)
```

Select your preferred package manager, and optionally save it to avoid future prompts.

### Package Manager Detection Priority

`np` follows this detection order:

1. **`packageManager` field** in `package.json` (highest priority)
   ```json
   {
     "packageManager": "pnpm@9.9.0"
   }
   ```

2. **Lock files** in the project directory:
   - `package-lock.json` → npm
   - `yarn.lock` → yarn
   - `pnpm-lock.yaml` → pnpm

3. **Interactive prompt** (when multiple or none are detected)

## 🎯 Real-World Examples

### Monorepo with Mixed Package Managers

```bash
cd frontend/          # Uses yarn
np install           # ✅ Runs: yarn install

cd ../backend/       # Uses pnpm
np install          # ✅ Runs: pnpm install

cd ../legacy/        # Uses npm
np install          # ✅ Runs: npm install
```

### Contributing to Open Source

```bash
# Clone any project
git clone https://github.com/some/project.git
cd project

# Just use np - it adapts automatically
np install
np run test
np run build
```

### CI/CD Integration

```yaml
# .github/workflows/ci.yml
- name: Install dependencies
  run: np install
  
- name: Run tests  
  run: np test
  
- name: Build
  run: np run build
```

## 🏗️ How It Works

1. **Detection Phase**: 
   - Scans `package.json` for `packageManager` field
   - Falls back to detecting lock files (`package-lock.json`, `yarn.lock`, `pnpm-lock.yaml`)
   - Prompts user if detection is ambiguous

2. **Execution Phase**:
   - Spawns the detected package manager with all original arguments
   - Inherits stdin/stdout/stderr for seamless integration
   - Preserves exit codes and signals

3. **Persistence** (Optional):
   - Saves user choice to `package.json` → `packageManager` field
   - Ensures consistent behavior across team members

## 🧪 Testing

`np` comes with a comprehensive test suite:

```bash
# Run all tests
cargo test

# Run specific test suites
cargo test --test package_detector_tests
cargo test --test script_handler_tests
cargo test --test command_executor_tests
```

**Test Coverage**:
- ✅ 10 package detection tests
- ✅ 15 package.json read/write tests
- ✅ 9 command execution tests

## 🤝 Contributing

Contributions are welcome! Here's how to get started:

1. Fork the repository
2. Create your feature branch: `git checkout -b feature/amazing-feature`
3. Make your changes and add tests
4. Run the test suite: `cargo test`
5. Commit your changes: `git commit -m 'Add amazing feature'`
6. Push to the branch: `git push origin feature/amazing-feature`
7. Open a Pull Request

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with the amazing Rust ecosystem
- Thanks to all contributors and users!

## ⭐ Show Your Support

If `np` saves you time and mental overhead, please consider:
- ⭐ Starring the repository
- 🐛 Reporting bugs and suggesting features
- 📢 Sharing with your team and friends

---

<div align="center">

**Made with ❤️ and Rust**

[Report Bug](https://github.com/jordyfontoura/np/issues) · [Request Feature](https://github.com/jordyfontoura/np/issues)

<br/>

<a href="https://discord.com/users/jordyfontoura" target="_blank" rel="noreferrer">
  <img alt="Discord" src="https://img.shields.io/badge/Discord-5865F2?style=for-the-badge&logo=discord&logoColor=white" />
  </a>
  <a href="https://www.linkedin.com/in/jordy-fontoura/" target="_blank" rel="noreferrer">
  <img alt="LinkedIn" src="https://img.shields.io/badge/LinkedIn-0A66C2?style=for-the-badge&logo=linkedin&logoColor=white" />
  </a>

</div>

