# 🐕 LogDog: The Hacker's Best Friend

> A blazingly fast, terminal-native reporting engine for bug bounty hunters and penetration testers, written in Rust.

---

## 📜 PROBLEM!

Bug bounty hunters spend 50% of their time hacking and 50% writing tedious vulnerability reports. Constantly switching between the terminal, a browser, and a text editor destroys the flow state. 

**The DOG's philosopy**
**Our dawg believes that,**
1. **The Terminal is Home:** If a hacker has to leave the CLI to log a bug, we have failed.
2. **Speed Over Features:** LogDog is a lightweight, memory-safe text router.
3. **Format-Agnostic Output:** Hackers shouldn't memorize HackerOne or Bugcrowd markdown tables. LogDog compiles raw terminal chaos into platform-ready reports instantly.

---

## 🚀 Features (V1.0 Roadmap)

The initial sprint focuses entirely on the "Core 4" commands necessary to eliminate copy-pasting from the workflow.

* **`ld init`**: Generates a standardized `report.md` skeleton in the current directory.
* **`ld step`**: Appends numbered, sequential actions to the "Steps to Reproduce" section without opening an editor.
* **`ld fetch`**: Reads standard input (`stdin`) directly from other terminal tools (like `nmap` or `curl`), wraps it in Markdown code blocks, timestamps it, and appends it to the report as evidence.
* **`ld view`**: Prints the current draft of the report to the terminal with syntax highlighting for quick review.

## 🔮 The Future (Post-V1.0)
Once the core engine is flawless, we will introduce:
* **`ld export --format hackerone`**: A compiler that automatically structures the raw logs into platform-specific submission templates.
* **Secret Scrubbing**: Auto-redaction of API keys or passwords from terminal output before it hits the markdown file.
* **Custom Templates**: Support for bringing your own `.toml` configuration files for custom report structures.

---

## 🛠️ Installation (Coming Soon)
*(LogDog is currently in active development. Binaries and Cargo installation commands will be provided upon the V1.0 release.)*

## 🤝 Contributing
This tool is built by the community, for the community. If you are a hacker tired of formatting Markdown tables, drop an issue with your requested workflow feature.
