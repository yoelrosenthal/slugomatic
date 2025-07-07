# Changelog
## [v0.2.2] - 2025-07-07

### 🐞 Bug Fixes
- Fixed `--title` so that title case is applied before slugification for slugs

## [v0.2.1] - 2025-07-07

### 📝 Documentation
- Small formatting changes in documentation and examples for improved clarity.

## [v0.2.0] - 2025-07-07


### ⚠️ Breaking Change
- **Default Case Preservation:**  
  By default, slugified output now preserves the input's capitalization. *Lowercase* is **no longer the default**.  
  - You must explicitly use `--lowercase` if you want lowercase slugs.
  - This affects git workflows and scripts expecting lowercase by default.

### ✨ New Features
- **Case Conversion Options:**
  - `--lowercase`: Force the output to all lowercase.
  - `--uppercase`: Force the output to all uppercase.
  - `--title`: Convert output to Title Case (each word capitalized).
  - These case options are mutually exclusive.
- **Improved Unslugify:**  
  Easily convert slugs back to readable text and apply any case transformation.
- **Example Enhancements:**  
  Updated CLI and README examples for clarity, showing explicit case usage.

### 🚨 Migration Notes
- If you relied on old releases auto-lowercasing output, **add `--lowercase` to your command(s)** to maintain the previous behavior.

---

For detailed usage and more examples, see the README or run `slug --help`.

[v0.2.0]: https://github.com/yoelrosenthal/slugomatic/releases/tag/v0.2.0
[v0.2.1]: https://github.com/yoelrosenthal/slugomatic/releases/tag/v0.2.1
[v0.2.2]: https://github.com/yoelrosenthal/slugomatic/releases/tag/v0.2.2
