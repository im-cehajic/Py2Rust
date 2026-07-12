# Contributing to Py2Rust

## Code Style

- Follow Rust conventions (rustfmt)
- Run `cargo fmt` before committing
- Run `cargo clippy` for linting

## Adding Features

1. **Create an issue** describing the feature
2. **Fork and create a branch**: `git checkout -b feature/my-feature`
3. **Write tests** for the new feature
4. **Implement the feature** following the architecture
5. **Run tests**: `cargo test`
6. **Submit a PR** with description of changes

## Architecture

### Adding Python Language Support

1. **Parser** (`src/parser/ast_visitor.rs`)
   - Add handling in `visit_expr()` or `visit_stmt()`
   - Convert Python AST nodes to IR

2. **IR** (`src/ir/mod.rs`)
   - Add new variant to `Expr` or `Statement`
   - Update serialization derive

3. **Generator** (`src/generator/*.rs`)
   - Add code generation for new IR node
   - Update `ExpressionGenerator` or `StatementGenerator`

4. **Tests** (`tests/integration_test.rs`)
   - Add test case for the feature

### Example: Adding `elif` Support

1. **Parser**: Handle `elif` in Python AST conversion
2. **IR**: Add `ElseIf { conditions, bodies }` to Statement
3. **Generator**: Generate Rust `else if` blocks
4. **Test**: Add test for `elif` conversion

## Testing Guidelines

- Test both success and error cases
- Use `#[test]` and include docstrings
- Run tests with `cargo test -- --nocapture`
- Update CHANGELOG.md with significant changes

## Known Limitations

See CHANGELOG.md for:
- Features in progress
- Known limitations
- Future roadmap

## Getting Help

- Check existing issues
- Read the architecture docs
- Look at similar implementations
- Ask in pull request discussions

## Release Process

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Run full test suite: `./build.sh`
4. Create git tag: `git tag v0.x.x`
5. Push to repository

## Code Review Checklist

- [ ] Code follows Rust style guide
- [ ] All tests pass
- [ ] New code has tests
- [ ] Documentation updated
- [ ] No breaking changes without discussion
- [ ] CHANGELOG updated
