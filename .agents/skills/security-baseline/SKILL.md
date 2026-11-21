---
name: security-baseline
description: Use when code handles user input, authentication, authorization, secrets, files, network calls, databases, payments or personal data. A short non-negotiable security checklist applied during design, build and review, mapped to common OWASP risks.
---

# Security Baseline

## Always
- **Secrets**: never in code, logs, URLs, error messages, tests or git history. Load from env/secret manager. `.env` is gitignored; commit `.env.example` with placeholders. Run `secrets_scan.sh`.
- **Validate at the boundary**: parse every external input (body, query, headers, files, webhooks, env) with a schema; reject unknown/oversized/malformed. Trust nothing past the parser except typed values.
- **AuthN/AuthZ on every route**: deny by default. Check the caller may access THIS resource (object-level / tenant check), not just that they are logged in.
- **Injection**: parameterized queries only; no string-built SQL/shell/regex/paths. Use `execFile` with arg arrays, never `exec` with interpolation.
- **Output encoding**: escape by context (HTML, URL, JSON, shell). No `dangerouslySetInnerHTML`/`innerHTML` with untrusted data.
- **Least privilege**: DB roles, cloud IAM, API tokens, service accounts get only what they need.
- **Dependencies**: pinned, audited, minimal. Check new packages for maintenance, typosquatting, install scripts.
- **Errors/logging**: no stack traces or internals to clients; structured logs; redact tokens/PII; log security events (login fail, permission denied).

## When relevant
- **Sessions/tokens**: httpOnly + Secure + SameSite cookies; short-lived tokens; rotate/revoke; constant-time compare; hash passwords with argon2/bcrypt.
- **CORS/CSRF**: explicit origin allowlist (never `*` with credentials); CSRF protection for cookie auth.
- **Rate limits & abuse**: login, signup, OTP, expensive endpoints, webhooks. Size/time limits on requests and uploads.
- **SSRF**: never fetch user-supplied URLs without allowlist and private-IP blocking.
- **File uploads**: type by content not extension, size cap, random names, store outside web root, scan if feasible.
- **Webhooks**: verify signatures and timestamps; idempotency keys.
- **PII/privacy**: collect the minimum, encrypt at rest and in transit, define retention and deletion.
- **AI/LLM features**: treat model output and retrieved text as untrusted input; no model-driven shell/SQL without allowlists; never put secrets in prompts; log prompts without PII.

## Review questions
Who can call this? What can they send? What is the worst thing it does with the worst input?
What leaks if it fails? Is there a negative test for each answer? If unsure, ask the human; never "assume it's fine".
