# Codebase Agent Guidelines

## Build/Lint/Test Commands

### General Commands
- `cargo build` - Compile the project
- `cargo check` - Check code without building
- `cargo test` - Run all tests
- `cargo test --workspace` - Run tests in workspace
- `cargo fmt` - Format code using rustfmt
- `cargo clippy` - Run linter checks

### Running Single Tests
- `cargo test <test_name>` - Run specific test by name
- `cargo test -- --exact <test_name>` - Run exact match test
- `cargo test -- --test-threads=1` - Run tests serially 
- `cargo test --lib` - Run only library tests
- `cargo test --tests` - Run only integration tests
- `cargo nextest run` - Run tests with nextest (if available)

### Build and Release
- `cargo build --release` - Build optimized release version
- `cargo install` - Install the package locally

## Code Style Guidelines

### Imports and Formatting
- Use standard Rust import syntax with `use` statements
- Follow Cargo conventions for module organization
- Format code using rustfmt (configuration in .rustfmt.toml or similar)
- Maintain consistent indentation (typically 4 spaces)

### Types and Naming Conventions
- Use camelCase for variables and functions
- Use PascalCase for structs, enums, and traits
- Use SNAKE_CASE for constants
- Prefer descriptive names over abbreviations
- Use `Result<T, E>` for error handling patterns

### Error Handling
- Use Rust's built-in error handling with `Result` and `Option`
- Implement custom error types when appropriate using `thiserror` or `anyhow`
- Log errors appropriately without panicking unless absolutely necessary
- Don't ignore `Result` or `Option` values without explicit reasoning

### Code Structure
- Keep functions small and focused (single responsibility principle)
- Use modules to organize code logically
- Break complex logic into smaller, testable components
- Follow Rust idioms for structuring code
- Use proper documentation comments with /// syntax

### Testing Practices
- Write unit tests in the same file using `#[cfg(test)]`
- Write integration tests in tests/ directory
- Test edge cases and error conditions
- Mock external dependencies appropriately
- Maintain high test coverage where reasonable

## Cursor/Copilot Rules

No specific Cursor or Copilot rules defined. Agents should follow general Rust best practices for code quality, maintainability, and idiomatic usage of the language.

## Available Skills and Guidelines

This repository contains several specialized skills that provide additional guidance:

### Domain-Specific Skills
- `m01-ownership`: Ownership patterns in Rust
- `m06-error-handling`: Error handling techniques 
- `m07-concurrency`: Concurrency patterns
- `rust-best-practices`: General best practices for Rust development

These skills should be consulted when implementing code that requires deep knowledge of specific Rust concepts or patterns.

### Agent Types Available
The following agent types are available in this environment:
- architect.md: For architectural decisions and planning
- debugger.md: For debugging assistance 
- devops.md: For deployment and operations tasks
- docs.md: For documentation generation
- implementer.md: For implementing features
- performance.md: For performance optimization
- planner.md: For project planning
- refactorer.md: For code refactoring
- reviewer.md: For code review assistance
- security.md: For security considerations
- skill-creator.md: For creating new skills
- tester.md: For testing tasks

Agents should utilize these specialized agents when their specific expertise is required for the task at hand.