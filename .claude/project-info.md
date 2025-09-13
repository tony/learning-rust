# RIPER Workflow Documentation

## Overview
This project implements the RIPER (Research, Innovate, Plan, Execute, Review) workflow methodology for systematic and high-quality software development.

## RIPER Phases

### 1. RESEARCH
**Purpose**: Gather comprehensive information
**Agent**: research-innovate (RESEARCH sub-mode)
**Commands**: `/riper:research`
**Restrictions**: Read-only operations

### 2. INNOVATE
**Purpose**: Creative solution exploration
**Agent**: research-innovate (INNOVATE sub-mode)
**Commands**: `/riper:innovate`
**Restrictions**: Brainstorming only, no implementation

### 3. PLAN
**Purpose**: Create detailed specifications
**Agent**: plan-execute (PLAN sub-mode)
**Commands**: `/riper:plan`
**Restrictions**: Write to memory bank only

### 4. EXECUTE
**Purpose**: Implement approved plans
**Agent**: plan-execute (EXECUTE sub-mode)
**Commands**: `/riper:execute`, `/riper:execute <step>`
**Restrictions**: Follow plan exactly

### 5. REVIEW
**Purpose**: Validate implementation
**Agent**: review
**Commands**: `/riper:review`
**Restrictions**: Verification only, no modifications

## Memory Bank Policy

### CRITICAL: Repository Root Enforcement
All memory bank operations MUST:
1. Use `git rev-parse --show-toplevel` to find repository root
2. Store at `[ROOT]/.claude/memory-bank/`
3. Never create package-level memory banks in subdirectories

### Directory Structure
```
.claude/memory-bank/
├── [branch]/
│   ├── plans/      # Technical specifications
│   ├── reviews/    # Validation reports
│   └── sessions/   # Context preservation
└── backups/        # Automatic backups
```

### File Naming Convention
`[branch]-[date]-[feature].md`

Example: `main-2025-01-13-tmux-integration.md`

## Available Commands

### RIPER Workflow Commands
- `/riper:strict` - Enable strict protocol enforcement
- `/riper:research` - Enter RESEARCH mode
- `/riper:innovate` - Enter INNOVATE mode
- `/riper:plan` - Enter PLAN mode
- `/riper:execute` - Execute approved plan
- `/riper:execute <step>` - Execute specific step
- `/riper:review` - Enter REVIEW mode
- `/riper:workflow` - Full guided workflow

### Memory Bank Commands
- `/memory:save <context>` - Save to memory bank
- `/memory:recall <topic>` - Retrieve memories
- `/memory:list` - List all memories

## Strict Mode

When `/riper:strict` is enabled:
- Every response must declare mode: `[MODE: X]`
- Out-of-mode actions are blocked
- Mode transitions require explicit commands
- Zero tolerance for violations

## Agent Architecture

### 3-Agent Consolidated Design
1. **research-innovate**: Handles RESEARCH and INNOVATE
2. **plan-execute**: Handles PLAN and EXECUTE
3. **review**: Handles REVIEW validation

This consolidation improves:
- Performance (reduced context switching)
- Coherence (related phases grouped)
- Efficiency (fewer agent invocations)

## Project-Specific Context

### Technology Stack
- Language: Rust
- Build System: Cargo
- Key Integration: tmux
- Testing: Comprehensive unit tests with mocks

### Key Patterns
- Mock executables for testing
- Command execution abstraction
- Path finding utilities
- Error handling with Result types

### Important Paths
- Source: `src/`
- Tests: Integrated in source files
- Documentation: `CLAUDE.md`, `.claude/`

## Quality Standards

### Code Requirements
- Follow Rust idioms and conventions
- Comprehensive error handling
- Unit tests for all functions
- Clear documentation comments

### RIPER Compliance
- Complete each phase before proceeding
- Document all decisions in memory bank
- Validate against specifications
- Iterate when review identifies issues

## Workflow Best Practices

1. **Always start with RESEARCH** - Never skip understanding
2. **Innovate without constraints** - Best ideas come from freedom
3. **Plan with precision** - Specifications prevent problems
4. **Execute exactly** - No deviations from plan
5. **Review ruthlessly** - Quality over speed

## Common Workflows

### Feature Development
```
/riper:workflow → RESEARCH → INNOVATE → PLAN → EXECUTE → REVIEW
```

### Bug Fix
```
/riper:research → /riper:plan → /riper:execute → /riper:review
```

### Quick Task
```
/riper:plan → /riper:execute
```

## Troubleshooting

### Mode Violations
If you see "MODE VIOLATION DETECTED":
1. Check current mode declaration
2. Use appropriate `/riper:[mode]` command
3. Ensure action matches mode capabilities

### Memory Bank Issues
If memory bank operations fail:
1. Verify using `git rev-parse --show-toplevel`
2. Check directory permissions
3. Ensure branch name is valid

### Agent Confusion
If agent seems confused about mode:
1. Restart with clear mode command
2. Check `${SUBMODE}` variable
3. Review agent definition file

## Version History
- v1.0.0: Initial RIPER implementation with 3-agent architecture