# Frozen Modules Registry

This file tracks modules that have been validated against CDP and are now frozen.
These modules MUST NOT be modified without explicit approval and re-validation.

## Frozen Modules

| Module | Frozen Date | CDP Programs Validated | Test Coverage |
|--------|-------------|----------------------|---------------|
| cdp-core | TBD | N/A (foundational) | 100% |

## Pending Validation

| Module | Status | CDP Programs | Notes |
|--------|--------|--------------|-------|
| cdp-pvoc | In Development | pvoc | Phase vocoder implementation |
| cdp-spectral | Planned | blur, focus, combine | Spectral processors |

## Validation Process

1. Implement in `cdp-sandbox/`
2. Run oracle tests against CDP binaries
3. Achieve >99.99% spectral correlation
4. Move to appropriate frozen module
5. Update this registry
6. Lock module with `#![forbid(unsafe_code)]`

## How to Request Changes to Frozen Modules

1. Open an issue describing the needed change
2. Provide justification for modification
3. Run full oracle validation suite
4. Get approval from 2 maintainers
5. Update this registry with change log