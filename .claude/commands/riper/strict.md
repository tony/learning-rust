---
name: /riper:strict
description: Enable strict RIPER protocol enforcement with mode declarations
---

# RIPER: STRICT MODE ACTIVATION

You are now operating under **STRICT RIPER PROTOCOL ENFORCEMENT**.

## Mode Declaration Requirements

**MANDATORY**: Every response MUST begin with one of:
- `[MODE: RESEARCH]` - Information gathering only
- `[MODE: INNOVATE]` - Brainstorming and ideation
- `[MODE: PLAN]` - Creating specifications
- `[MODE: EXECUTE]` - Implementing approved plans
- `[MODE: REVIEW]` - Validating implementation
- `[NO MODE]` - Administrative tasks only

## Mode Enforcement Rules

### Out-of-Mode Violations
Any action outside current mode capabilities triggers:
```
⚠️ MODE VIOLATION DETECTED

Current Mode: [MODE]
Attempted Action: [ACTION]
Status: BLOCKED

This action requires [REQUIRED_MODE] mode.
Use appropriate /riper:[mode] command to switch.
```

### Mode Transition Requirements
- Explicit mode change via /riper:[mode] command
- Cannot skip modes in workflow sequence
- Must complete or abort current mode first

## Available Commands

### Workflow Control
- `/riper:research` - Enter RESEARCH mode (read-only)
- `/riper:innovate` - Enter INNOVATE mode (brainstorming)
- `/riper:plan` - Enter PLAN mode (specifications)
- `/riper:execute` - Execute approved plan
- `/riper:review` - Enter REVIEW mode (validation)
- `/riper:workflow` - Start guided full workflow

### Memory Operations
- `/memory:save <context>` - Save to memory bank
- `/memory:recall <topic>` - Retrieve memories
- `/memory:list` - List all memories

## Strict Mode Capabilities by Phase

### RESEARCH Mode
- ✅ Read files, search code, analyze
- ❌ Write files, modify code, execute

### INNOVATE Mode
- ✅ Brainstorm, propose ideas, explore
- ❌ Write code, create files, implement

### PLAN Mode
- ✅ Write specifications to memory bank
- ❌ Write code files, implement features

### EXECUTE Mode
- ✅ Implement approved plans exactly
- ❌ Deviate from plan, add features

### REVIEW Mode
- ✅ Validate, test, verify against plan
- ❌ Modify code, change implementation

## Workflow Sequence

```
RESEARCH → INNOVATE → PLAN → EXECUTE → REVIEW
    ↑                                      ↓
    ←──────────── Iterate if needed ←──────
```

## Immediate Effect

Strict mode is now **ACTIVE**. All subsequent operations will be monitored for mode compliance.

First action: Declare your current mode or use `/riper:workflow` to begin.