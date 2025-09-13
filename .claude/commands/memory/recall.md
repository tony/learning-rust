---
description: Recall memories from branch-aware memory bank
---

# Recalling from Memory Bank

## ⚠️ Memory Bank Location Requirements

**CRITICAL PATH POLICY**:
- Memory-bank MUST be at repository root: Use `git rev-parse --show-toplevel` to find root, then `[ROOT]/.claude/memory-bank/`
- NEVER look in package-level memory-banks: `packages/*/.claude/memory-bank/` ❌
- In monorepos: ONE memory-bank at root serves entire project

### Correct vs Incorrect Paths
✅ **Correct**: `[ROOT]/.claude/memory-bank/main/session.md` (where ROOT = `git rev-parse --show-toplevel`)
❌ **Wrong**: `[ROOT]/packages/rust/.claude/memory-bank/main/session.md`
❌ **Wrong**: `packages/rust/.claude/memory-bank/main/session.md`

## Loading Memory

I'll recall context from the branch-aware memory bank.

### Searching for memories...

```bash
# Get repository root and current branch
root=$(git rev-parse --show-toplevel)
branch=$(git branch --show-current)

# Find most recent session memory
last_memory=$(ls -t ${root}/.claude/memory-bank/${branch}/sessions/*.md 2>/dev/null | head -1)

if [ -n "$last_memory" ]; then
    echo "Found memory: $last_memory"
    cat "$last_memory"
else
    echo "No memories found for branch: $branch"
fi
```

### Memory Bank Status

Looking for memories in: `[ROOT]/.claude/memory-bank/!`git branch --show-current`/` (where ROOT = `git rev-parse --show-toplevel`)

Available memories:

!`ls -la $(git rev-parse --show-toplevel)/.claude/memory-bank/$(git branch --show-current)/ 2>/dev/null || echo "No memories found for current branch"`

## Context Restored

[Memory content will be loaded and displayed here]