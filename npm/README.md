# np - The Universal Package Manager Proxy 🚀

<div align="center">

[![npm version](https://img.shields.io/npm/v/@kitlib/np.svg?style=for-the-badge&logo=npm)](https://www.npmjs.com/package/@kitlib/np)
[![Downloads](https://img.shields.io/npm/dt/@kitlib/np.svg?style=for-the-badge&logo=npm)](https://www.npmjs.com/package/@kitlib/np)
[![License](https://img.shields.io/badge/license-MIT-blue.svg?style=for-the-badge)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)

</div>

> **Stop memorizing commands. Start building.**

`np` is a blazingly fast, intelligent package manager proxy that automatically detects and forwards commands to the right package manager (npm, yarn, or pnpm) in your project. 

**Write once, run anywhere.** No more context switching between package managers!

---

## 📑 Table of Contents

- [Highlights](#-highlights)
- [The Problem](#-the-problem)
- [The Solution](#-the-solution)
- [Why np?](#-why-np)
- [Who Should Use np?](#-who-should-use-np)
- [Features](#-features)
- [Installation](#-installation)
- [Quick Start](#-quick-start)
- [Usage](#-usage)
- [Real-World Examples](#-real-world-examples)
- [How It Works](#️-how-it-works)
- [Performance](#-performance)
- [Testing](#-testing)
- [FAQ](#-faq)
- [Contributing](#-contributing)
- [Support & Community](#-support--community)
- [License](#-license)

---

## ✨ Highlights

```bash
# One command for all package managers
$ np install        # Works with npm, yarn, or pnpm
$ np run dev        # Automatically uses the right PM
$ np add axios      # No more mental overhead
```

**Key Benefits:**
- 🚀 **10x faster** than switching contexts
- 🎯 **100% compatible** with existing projects
- ⚡ **Near-zero overhead** thanks to Rust
- 🔧 **No configuration** needed

> 💡 **Try it now:** `npx @kitlib/np --version` (no installation required!)

---

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

## 💪 Why np?

| Without np | With np |
|------------|---------|
| Remember which PM each project uses | Use `np` everywhere |
| Check lock files before running commands | Automatic detection |
| Context-switch between npm/yarn/pnpm syntax | Universal syntax |
| Risk using the wrong package manager | Always uses the right one |
| Configure CI/CD for different PMs | Single `np` command |

**One tool. Zero friction. Maximum productivity.**

## 👥 Who Should Use np?

`np` is perfect for:

- **🏢 Teams** working on multiple projects with different package managers
- **🌐 Open source contributors** who frequently switch between repositories
- **📚 Polyglot developers** who can't remember which PM each project uses
- **🚀 DevOps engineers** looking to simplify CI/CD pipelines
- **🎓 Learners** who want to focus on coding, not tooling
- **💼 Agencies** managing client projects with varying tech stacks

**Not sure if you need it?** If you've ever typed `npm install` in a yarn project (or vice versa), `np` is for you!

## 🎯 Features

- **🧠 Smart Detection**: Automatically reads `packageManager` field in `package.json` or scans lock files
- **⚡ Zero Config**: Works out of the box, no setup required
- **🔄 Full Compatibility**: All npm/yarn/pnpm commands work exactly as expected
- **💪 Blazingly Fast**: Built with Rust for near-zero overhead
- **🎨 Interactive Mode**: Prompts you when multiple package managers are detected
- **💾 Persistent Choices**: Optionally save your selection to `package.json` automatically
- **🌍 Universal**: Works in any Node.js project, regardless of package manager
- **🔧 Battle-Tested**: Comprehensive test suite with 34+ tests
- **📦 Lightweight**: Minimal footprint, maximum performance

## 📦 Installation

### Using npm (Recommended)

The easiest way to get started is via npm:

```bash
# Global install
npm i -g @kitlib/np

# Or run without installing
npx @kitlib/np --version
```

**Why npm install?**
- ✅ Works on all platforms (Windows, macOS, Linux)
- ✅ No need to install Rust toolchain
- ✅ Automatic updates with `npm update -g @kitlib/np`
- ✅ Pre-compiled binaries for optimal performance

After installing globally, just use `np` as a drop-in replacement:

```bash
np install
np run dev
```

### Using Cargo

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

### Platform Support

`np` works on all major platforms:
- ✅ **Linux** (x64, arm64)
- ✅ **macOS** (Intel, Apple Silicon)
- ✅ **Windows** (x64)

## 🚀 Quick Start

**1. Install globally:**

```bash
npm i -g @kitlib/np
```

**2. Use in any project:**

```bash
# Instead of npm/yarn/pnpm install
np install

# Instead of npm/yarn/pnpm run dev
np run dev

# Instead of npm/yarn/pnpm add axios
np add axios
```

**That's it!** `np` automatically detects which package manager your project uses and runs the appropriate command. No configuration needed.

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

### Common Command Translations

Here's how `np` translates to different package managers:

| np Command | npm Equivalent | yarn Equivalent | pnpm Equivalent |
|------------|---------------|-----------------|-----------------|
| `np install` | `npm install` | `yarn install` | `pnpm install` |
| `np add axios` | `npm install axios` | `yarn add axios` | `pnpm add axios` |
| `np add -D jest` | `npm install -D jest` | `yarn add -D jest` | `pnpm add -D jest` |
| `np remove lodash` | `npm uninstall lodash` | `yarn remove lodash` | `pnpm remove lodash` |
| `np run dev` | `npm run dev` | `yarn run dev` | `pnpm run dev` |
| `np test` | `npm test` | `yarn test` | `pnpm test` |
| `np run build` | `npm run build` | `yarn build` | `pnpm run build` |

**The beauty of `np`:** You don't need to remember these differences anymore! Just use `np` and it works everywhere.

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

## ⚡ Performance

`np` is built with Rust for maximum performance. Here's what that means for you:

| Operation | Overhead | Notes |
|-----------|----------|-------|
| Package detection | < 5ms | Reads lock files or package.json |
| Command forwarding | < 1ms | Direct process spawn, no intermediary layers |
| Total overhead | < 10ms | Negligible compared to actual PM operations |

**Real-world impact:** Running `np install` vs `npm install` adds less than 10ms to the total execution time. You won't notice the difference, but you'll love the convenience!

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

## ❓ FAQ

<details>
<summary><strong>Is np slower than using npm/yarn/pnpm directly?</strong></summary>

No! `np` is built with Rust and adds near-zero overhead. The detection happens in milliseconds, and then `np` directly spawns the actual package manager with all arguments passed through.
</details>

<details>
<summary><strong>Will np work with my existing scripts?</strong></summary>

Yes! You can use `np` as a drop-in replacement. All commands, flags, and arguments are forwarded exactly as you write them. Your scripts in `package.json` will continue to work without any changes.
</details>

<details>
<summary><strong>What if my project has multiple lock files?</strong></summary>

`np` will prompt you to choose which package manager to use and optionally save your choice to `package.json`. This prevents conflicts in the future.
</details>

<details>
<summary><strong>Can I use np in CI/CD pipelines?</strong></summary>

Absolutely! `np` is perfect for CI/CD environments. Just make sure your `package.json` has the `packageManager` field set, or ensure only one lock file exists in your project.
</details>

<details>
<summary><strong>Does np support bun or other package managers?</strong></summary>

Currently, `np` supports npm, yarn, and pnpm. Support for other package managers like bun may be added in future versions.
</details>

<details>
<summary><strong>How do I update np to the latest version?</strong></summary>

Simply run `npm update -g @kitlib/np` to get the latest version. You can check your current version with `np --version`.
</details>

<details>
<summary><strong>Can I use np in a monorepo?</strong></summary>

Yes! `np` works perfectly in monorepos. Each package can have its own package manager, and `np` will detect the correct one based on the directory you're in.
</details>

<details>
<summary><strong>What if np can't find my package manager?</strong></summary>

Make sure you have the package manager installed globally. `np` detects which PM to use, but you still need to have npm/yarn/pnpm installed on your system.
</details>

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
- Inspired by the need for a universal package manager interface

## 💬 Support & Community

Need help or want to discuss `np`?

- 🐛 **Bug Reports**: [Open an issue](https://github.com/jordyfontoura/np/issues/new?labels=bug)
- 💡 **Feature Requests**: [Suggest a feature](https://github.com/jordyfontoura/np/issues/new?labels=enhancement)
- 💬 **Discussions**: [Join the conversation](https://github.com/jordyfontoura/np/discussions)
- 📧 **Contact**: Reach out via [Discord](https://discord.com/users/jordyfontoura) or [LinkedIn](https://www.linkedin.com/in/jordy-fontoura/)

## ⭐ Show Your Support

If `np` saves you time and mental overhead, please consider:

- ⭐ **Star the repository** to show your appreciation
- 🐛 **Report bugs** and suggest features to help improve `np`
- 📢 **Share with colleagues** who struggle with multiple package managers
- 💬 **Leave a review** on npm to help others discover `np`
- 🤝 **Contribute** to make `np` even better

**Every star motivates us to keep improving!** ⭐

---

<div align="center">

### 🚀 Ready to Simplify Your Workflow?

```bash
npm i -g @kitlib/np
```

**Stop thinking about package managers. Start building amazing things.**

---

**Made with ❤️ and Rust**

[Report Bug](https://github.com/jordyfontoura/np/issues) · [Request Feature](https://github.com/jordyfontoura/np/issues) · [View on npm](https://www.npmjs.com/package/@kitlib/np)

<br/>

<a href="https://discord.com/users/jordyfontoura" target="_blank" rel="noreferrer">
  <img alt="Discord" src="https://img.shields.io/badge/Discord-5865F2?style=for-the-badge&logo=discord&logoColor=white" />
  </a>
  <a href="https://www.linkedin.com/in/jordy-fontoura/" target="_blank" rel="noreferrer">
  <img alt="LinkedIn" src="https://img.shields.io/badge/LinkedIn-0A66C2?style=for-the-badge&logo=linkedin&logoColor=white" />
  </a>

<br/><br/>

*np - One command to rule them all* 💍

</div>

