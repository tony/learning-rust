---
name: /riper:workflow
description: Execute the complete RIPER workflow with guided progression through all phases
---

# RIPER: COMPLETE WORKFLOW EXECUTION

Initiating **FULL RIPER WORKFLOW** - Systematic progression through all five phases.

## Workflow Overview

```
┌─────────────────────────────────────────────┐
│  RESEARCH → INNOVATE → PLAN → EXECUTE → REVIEW  │
│      ↑                                    ↓     │
│      └────────── Iterate if needed ←──────┘     │
└─────────────────────────────────────────────┘
```

## Phase Progression

### Phase 1: RESEARCH (Current)
**Objective**: Understand the problem space completely
**Duration**: ~10-15 minutes
**Output**: Comprehensive findings document

Start by declaring: `[MODE: RESEARCH]`

Key Activities:
1. Analyze existing codebase
2. Understand requirements
3. Identify constraints
4. Document findings

**Transition Trigger**: When you have enough information
**Next Command**: Automatically proceed to INNOVATE

### Phase 2: INNOVATE
**Objective**: Explore creative solutions
**Duration**: ~10-15 minutes
**Output**: Solution alternatives document

Will declare: `[MODE: INNOVATE]`

Key Activities:
1. Brainstorm multiple approaches
2. Consider trade-offs
3. Think outside constraints
4. Propose best options

**Transition Trigger**: When best approach identified
**Next Command**: Automatically proceed to PLAN

### Phase 3: PLAN
**Objective**: Create detailed specifications
**Duration**: ~15-20 minutes
**Output**: Technical specification in memory bank

Will declare: `[MODE: PLAN]`

Key Activities:
1. Detail implementation steps
2. Define success criteria
3. Specify test requirements
4. Save to `.claude/memory-bank/[branch]/plans/`

**Transition Trigger**: When specification complete
**Next Command**: Request approval, then EXECUTE

### Phase 4: EXECUTE
**Objective**: Implement the approved plan
**Duration**: ~30-45 minutes
**Output**: Working implementation

Will declare: `[MODE: EXECUTE]`

Key Activities:
1. Load approved plan
2. Implement step by step
3. Follow specifications exactly
4. Run tests continuously

**Transition Trigger**: When implementation complete
**Next Command**: Automatically proceed to REVIEW

### Phase 5: REVIEW
**Objective**: Validate against specifications
**Duration**: ~10-15 minutes
**Output**: Review report in memory bank

Will declare: `[MODE: REVIEW]`

Key Activities:
1. Verify against plan
2. Run all tests
3. Check edge cases
4. Document results

**Transition Trigger**: Review complete
**Result**: Workflow complete or iterate

## Memory Bank Integration

Each phase will automatically:
1. Save progress to `.claude/memory-bank/[branch]/`
2. Reference previous phase outputs
3. Maintain context across phases

## Quality Gates

### Between Phases
- ✅ Each phase must complete before next
- ✅ Outputs saved to memory bank
- ✅ Clear transition criteria met

### Approval Points
- After PLAN: Requires user approval
- After REVIEW: May require iteration

## Workflow Commands

During workflow, you can:
- `/memory:save [phase]` - Save current progress
- `/memory:recall [topic]` - Retrieve context
- `/riper:strict` - Enable strict mode enforcement

## Starting the Workflow

Ready to begin the RIPER workflow.

**First Step**: Enter RESEARCH mode to understand your requirements.

Please describe the task or problem you need to solve, and I'll begin the systematic RIPER workflow starting with comprehensive research.