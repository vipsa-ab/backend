# Skill Registry

**Delegator use only.** Any agent that launches sub-agents reads this registry to resolve compact rules, then injects them directly into sub-agent prompts. Sub-agents do NOT read this registry or individual SKILL.md files.

See `_shared/skill-resolver.md` for the full resolution protocol.

## User Skills

| Trigger | Skill | Path |
|---------|-------|------|
| Creating a pull request, opening a PR, preparing changes for review | branch-pr | /home/carfdev/.config/opencode/skills/branch-pr/SKILL.md |
| Creating a GitHub issue, reporting a bug, or requesting a feature | issue-creation | /home/carfdev/.config/opencode/skills/issue-creation/SKILL.md |
| Creating a new skill, adding agent instructions, or documenting patterns for AI | skill-creator | /home/carfdev/.config/opencode/skills/skill-creator/SKILL.md |
| "judgment day", "judgment-day", "review adversarial", "dual review" | judgment-day | /home/carfdev/.config/opencode/skills/judgment-day/SKILL.md |

## Compact Rules

Pre-digested rules per skill. Delegators copy matching blocks into sub-agent prompts as `## Project Standards (auto-resolved)`.

### branch-pr
- Every PR MUST link an approved issue — no exceptions
- Every PR MUST have exactly one `type:*` label
- Automated checks must pass before merge is possible
- Blank PRs without issue linkage will be blocked by GitHub Actions
- Branch naming: `type/description` — lowercase, no spaces (regex: `^(feat|fix|chore|docs|style|refactor|perf|test|build|ci|revert)\/[a-z0-9._-]+$`)
- PR body must include: `Closes #N` (valid keywords: Closes, Fixes, Resolves), exactly one type label, summary bullets, changes table, test plan
- Conventional commits: `type(scope): description` — types: build, chore, ci, docs, feat, fix, perf, refactor, revert, style, test
- Never add "Co-Authored-By" trailers

### issue-creation
- Blank issues are disabled — MUST use template (bug report or feature request)
- Every issue gets `status:needs-review` automatically on creation
- A maintainer MUST add `status:approved` before any PR can be opened
- Questions go to Discussions, not issues
- Bug report template: `.github/ISSUE_TEMPLATE/bug_report.yml`
- Feature request template: `.github/ISSUE_TEMPLATE/feature_request.yml`
- Label system: `status:approved` (maintainer), `priority:high/medium/low`, `bug`, `enhancement`

### skill-creator
- Create skill when: pattern repeated, project conventions differ from best practices, complex workflows need step-by-step, decision trees help AI
- Don't create when: documentation exists, pattern trivial, one-off task
- Structure: `skills/{skill-name}/SKILL.md` (required), `assets/` (templates, schemas), `references/` (local docs)
- Frontmatter required: name, description (with trigger), license (Apache-2.0), metadata.author (gentleman-programming), metadata.version
- After creating, add to AGENTS.md table

### judgment-day
- Launch TWO sub-agents via `delegate` (async, parallel) — they work independently with no cross-contamination
- Judges MUST classify warnings as WARNING (real) or WARNING (theoretical)
- WARNING (real): normal user can trigger — fix required
- WARNING (theoretical): requires contrived scenario — report but do NOT block
- Verdict synthesis: Confirmed (both find) = high confidence; Suspect A/B (one finds) = needs triage; Contradiction = manual decision
- Fix Agent: separate delegation, fix only confirmed issues
- After 2 fix iterations, ASK user if they want to continue before escalating
- NEVER push/commit after fixes until re-judgment completes
- Skill Resolution field in responses: injected/fallback-registry/fallback-path/none

## Project Conventions

| File | Path | Notes |
|------|------|-------|
| AGENTS.md | /home/carfdev/Code/projects/vipsa/backend/vipsa-backend/AGENTS.md | Index — references files below |

Read the convention files listed above for project-specific patterns and rules. All referenced paths have been extracted — no need to read index files to discover more.