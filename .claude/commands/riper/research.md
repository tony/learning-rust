---
name: /riper:research
description: Enter RESEARCH mode for read-only information gathering
---

# RIPER: RESEARCH MODE

You are now in **RESEARCH MODE** - Phase 1 of the RIPER workflow.

## Mode Declaration
All responses MUST begin with: `[MODE: RESEARCH]`

## Purpose
Gather comprehensive information about the problem space, codebase, and requirements without making any modifications.

## Allowed Actions
- ✅ Read and analyze files
- ✅ Search codebase for patterns
- ✅ Review documentation
- ✅ Examine git history
- ✅ List directory contents
- ✅ Check dependencies
- ✅ Run read-only commands (git log, git status, etc.)
- ✅ Analyze existing architecture
- ✅ Identify patterns and conventions

## FORBIDDEN Actions
- ❌ Writing or modifying any files
- ❌ Creating new files
- ❌ Executing code changes
- ❌ Running build/test commands
- ❌ Making design decisions
- ❌ Proposing solutions

## Research Checklist

### Codebase Understanding
- [ ] Project structure and organization
- [ ] Key files and their purposes
- [ ] Dependencies and versions
- [ ] Build and test processes
- [ ] Existing patterns and conventions

### Problem Analysis
- [ ] Current implementation status
- [ ] Related code sections
- [ ] Potential impact areas
- [ ] Edge cases to consider
- [ ] Technical constraints

### Context Gathering
```bash
# Recent changes
git log -n 10 --oneline

# Project structure
find . -type f -name "*.rs" | head -20

# Check for related implementations
grep -r "pattern" --include="*.rs" | head -10
```

## Output Format
```
[MODE: RESEARCH]

## Research Topic: [What you're investigating]

### Findings
1. [Key discovery]
2. [Important pattern]
3. [Relevant context]

### Code Analysis
- File: [path]
  Purpose: [what it does]
  Relevance: [why it matters]

### Next Steps
- Additional areas to research
- Questions needing clarification
```

## Transition to Next Phase
When research is complete:
1. Summarize key findings
2. Use `/riper:innovate` to brainstorm solutions
3. Or use `/memory:save research` to preserve findings

## Active Status
Research mode is now **ACTIVE**. Begin gathering information about the task at hand.