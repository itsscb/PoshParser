
# Contributing to posh-parser

Thank you for considering contributing to `posh-parser`! We welcome contributions from the community and are excited to see what you can bring to the project. This document outlines the process for contributing to the project.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [How to Contribute](#how-to-contribute)
  - [Reporting Bugs](#reporting-bugs)
  - [Suggesting Enhancements](#suggesting-enhancements)
  - [Submitting Pull Requests](#submitting-pull-requests)
- [Development Setup](#development-setup)
- [Style Guide](#style-guide)
- [License](#license)

## Code of Conduct

By participating in this project, you agree to abide by the [Code of Conduct](CODE_OF_CONDUCT.md). Please read it to understand the expectations for all contributors.

## How to Contribute

### Reporting Bugs

If you find a bug in the project, please open an issue on GitHub. Include as much detail as possible to help us understand and reproduce the issue.

- **Title**: A clear and descriptive title.
- **Description**: A detailed description of the issue.
- **Steps to Reproduce**: A step-by-step guide to reproduce the issue.
- **Expected Behavior**: What you expected to happen.
- **Actual Behavior**: What actually happened.
- **Environment**: Information about your environment (e.g., OS, Rust version).

### Suggesting Enhancements

If you have an idea for a new feature or an improvement to an existing feature, please open an issue on GitHub. Provide as much detail as possible to help us understand your suggestion.

- **Title**: A clear and descriptive title.
- **Description**: A detailed description of the enhancement.
- **Use Case**: Explain why this enhancement would be useful.
- **Implementation Ideas**: If you have any ideas on how to implement the enhancement, include them here.

### Submitting Pull Requests

If you would like to contribute code to the project, please follow these steps:

1. **Fork the Repository**: Click the "Fork" button at the top of the repository page to create a copy of the repository in your GitHub account.
2. **Clone the Repository**: Clone your forked repository to your local machine.
    ```sh
    git clone https://github.com/your-username/posh-parser.git
    cd posh-parser
    ```
3. **Create a Branch**: Create a new branch for your changes.
    ```sh
    git checkout -b feature/your-feature-name
    ```
4. **Make Changes**: Make your changes to the codebase.
5. **Commit Changes**: Commit your changes with a descriptive commit message.
    ```sh
    git commit -m "Add feature: your-feature-name"
    ```
6. **Push Changes**: Push your changes to your forked repository.
    ```sh
    git push origin feature/your-feature-name
    ```
7. **Open a Pull Request**: Go to the original repository and open a pull request. Provide a clear and descriptive title and description for your pull request.

## Development Setup

To set up your development environment, follow these steps:

1. **Install Rust**: Make sure you have Rust installed. You can download it from [rust-lang.org](https://www.rust-lang.org/).
2. **Clone the Repository**: Clone the repository to your local machine.
    ```sh
    git clone https://github.com/your-username/posh-parser.git
    cd posh-parser
    ```
3. **Build the Project**: Build the project to ensure everything is set up correctly.
    ```sh
    cargo build
    ```
4. **Run Tests**: Run the tests to make sure everything is working.
    ```sh
    cargo test
    ```

## Style Guide

Please follow the Rust style guidelines and best practices. We recommend using `rustfmt` to format your code and `clippy` to lint your code.

- **Formatting**: Use `rustfmt` to format your code.
    ```sh
    cargo fmt
    ```
- **Linting**: Use `clippy` to lint your code.
    ```sh
    cargo clippy
    ```

## License

By contributing to `posh-parser`, you agree that your contributions will be licensed under the license this project is licensed by.

---

Thank you for contributing to `posh-parser`! We appreciate your time and effort in improving the project. If you have any questions or need further assistance, feel free to open an issue or contact the maintainers.
