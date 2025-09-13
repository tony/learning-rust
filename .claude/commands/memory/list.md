---
description: List all memories in branch-aware memory bank
---

# Memory Bank Contents

## ⚠️ Memory Bank Location Requirements

**CRITICAL PATH POLICY**:
- Memory-bank MUST be at repository root: Use `git rev-parse --show-toplevel` to find root, then `[ROOT]/.claude/memory-bank/`
- NEVER look in package-level memory-banks: `packages/*/.claude/memory-bank/` ❌
- In monorepos: ONE memory-bank at root serves entire project

### Correct vs Incorrect Paths
✅ **Correct**: `[ROOT]/.claude/memory-bank/main/` (where ROOT = `git rev-parse --show-toplevel`)
❌ **Wrong**: `[ROOT]/packages/rust/.claude/memory-bank/main/`
❌ **Wrong**: `packages/rust/.claude/memory-bank/main/`

## Current Branch Memories

!`ls -la $(git rev-parse --show-toplevel)/.claude/memory-bank/$(git branch --show-current)/ 2>/dev/null || echo "No memories for current branch"`

## All Available Memories
!`find $(git rev-parse --show-toplevel)/.claude/memory-bank -type f -name "*.md" 2>/dev/null | head -20 || echo "No memories found"`

## Memory Bank Structure

[ROOT]/.claude/memory-bank/  (where ROOT = `git rev-parse --show-toplevel`)
├── [branch-name]/          # Branch-specific memory storage
│   ├── plans/             # Technical specifications and implementation plans
│   ├── reviews/           # Code review sessions and feedback
│   └── sessions/          # Conversation history and context
└── README.md              # Documentation