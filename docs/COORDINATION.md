# Sequential sessions across accounts

GitHub carries source, evidence and state. Chat histories, attachments, local
scratch files, credentials, host access and installed skills do not transfer.
This protocol is a human-coordinated single-writer convention, not a distributed
lock service. Never claim it detects another account's live activity automatically.

## Start

1. The user selects the active account. Fetch current remote main; inspect local
   changes before pulling/reconciling. Read AGENTS, PROJECT_STATE and latest log.
2. Compare state/log with actual source and latest relevant CI/release. If stale,
   correct the record based on evidence; preserve concurrent edits.
3. Set the state file's session owner to a nonpersonal label (e.g. Agent A/Agent B),
   current task/milestone, observed base SHA and `in progress`. Publish that claim
   in the first useful checkpoint, or an explicit coordination-only commit if a
   cross-account handover needs visibility before long work. Do not push dummy code.
4. If another session is marked active, an explicit user handover can switch the
   owner after verifying all recoverable work is published. Time elapsed alone
   does not grant permission to discard another session's work. If unsaved work
   is indicated, ask the user to have the outgoing account checkpoint it first.

## Work and checkpoint

- One writer per shared source. No simultaneous sessions editing the project.
- Work from main when authorized; no force push. If the head moves unexpectedly,
  stop publishing stale payloads, fetch the new changes and reconcile.
- Keep current progress in PROJECT_STATE, detailed diffs in Git, and handback
  evidence in HANDOFF_LOG. The roadmap holds the plan, not duplicate live status.
- Preserve useful checked milestones remotely. Never depend on an outgoing
  scratch directory surviving. If unfinished code cannot be safely pushed,
  provide an exact private patch/package and record where it is recoverable.
- Do not use this public repository for the original reference binary, private
  footage/projects, credentials, activation data or unsanitized diagnostic dumps.
- Docs-only coordination commits may use `[skip ci]`; never use it to evade
  validation for code, dependencies, build, installer or workflow changes.

## Hand back

1. Finish/check/publish the current coherent slice. Verify remote SHA, changed
   paths and relevant CI. For a binary, verify its tag/target, assets and checksums.
2. Update PROJECT_STATE: owner `none`, status `ready for next session`, observed
   source base, completed slice, remaining gaps, blockers/recovery and ONE next
   action. Never put the state commit's own future SHA inside itself.
3. Append the handback entry below to HANDOFF_LOG. Be concrete and concise; no
   private conversation transcripts or invented independent review.
4. Return the repository/head or release link, what's usable, checks/gaps and
   next action. The user can now switch accounts. No ongoing background writer.

### Handoff log entry template

```markdown
## YYYY-MM-DD — Short milestone — Actor label
- Observed base: <SHA before this work>.
- Scope/result: <user goal and resulting behavior; incomplete items explicit>.
- Changed paths: <source/config/docs groups; Git contains exact diffs>.
- Checks: <commands/environment/results and exact CI/run/release target links>.
- Host/reference evidence: <what ran, exact inputs; or not available>.
- Decisions/compatibility: <parameter IDs, migrations, dependency/version changes>.
- Gaps/recovery: <known failures, unpublished files or none>.
- Next: <one concrete task and acceptance>.
- Session: ready for next session; owner none.
```

## Returning to the first account

The returning agent follows the same start steps. Read logs since the previously
observed base, inspect Git diffs and changed file contracts, then use the latest
verified state. Do not restore old source or plans from chat memory. A successful
old CI run remains useful only for unchanged inputs and the claims it actually
proved. If the other account changed dependencies/API/build/release behavior,
read those changes before modifying dependent code.
