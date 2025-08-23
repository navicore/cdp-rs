# Development Workflow

## Current Status

We have a clean build! All tests pass because failing oracle tests are marked as `#[ignore]`.

Use `make test-status` to see which modules are ready:
- 🔴 **Red modules** have ignored tests (not implemented yet)
- 🟢 **Green modules** have all tests enabled (implementation complete)

## Working on a Module

When you're ready to implement a module (e.g., `cdp-housekeep`):

### 1. Enable the tests
```bash
./scripts/mark-tests.sh enable cdp-housekeep
```

### 2. Run the oracle tests to see failures
```bash
cargo test --package cdp-housekeep
```

### 3. Study the CDP source
Look at the CDP source mapping in `CDP_SOURCE_MAP.md` to find the original C code.

### 4. Fix the implementation
Implement the functionality to match CDP exactly.

### 5. Verify tests pass
```bash
cargo test --package cdp-housekeep
```

### 6. Update status
Update `MODULE_STATUS.md` to mark the module as complete.

## Build Commands

### Clean Build (CI-ready)
```bash
make test          # Runs all tests, fails on any error
make test-passing  # Runs only modules without ignored tests
```

### Development
```bash
make test-status   # Show which modules are ready
make test-oracle   # Run oracle tests (including ignored ones)
```

### Working on a specific module
```bash
# Enable tests for a module you're working on
./scripts/mark-tests.sh enable cdp-housekeep

# Work on implementation...
cargo test -p cdp-housekeep

# If you need to pause work, re-ignore the tests
./scripts/mark-tests.sh ignore cdp-housekeep
```

## Important Files

- `IMPLEMENTATION_PLAN.md` - Overall implementation strategy
- `MODULE_STATUS.md` - Current status of each module
- `CDP_SOURCE_MAP.md` - Where to find CDP source for each module
- `scripts/mark-tests.sh` - Enable/disable tests for modules

## Why This Approach?

1. **Clean CI**: `make test` always passes on main branch
2. **Clear Progress**: We know exactly which modules are done
3. **Incremental Work**: Can implement one module at a time
4. **No Skipping**: Tests either pass or are explicitly marked as not ready
5. **Easy Onboarding**: New contributors can see what needs work

## Next Steps

Start with Phase 1 from `IMPLEMENTATION_PLAN.md`:
1. Enable tests for `cdp-housekeep`
2. Fix the CDP WAV format (needs PEAK chunks)
3. Make the copy test pass
4. Move to the next module

Each successful implementation brings us closer to a complete CDP port!