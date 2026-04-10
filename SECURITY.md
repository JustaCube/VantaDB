# VantaDB Security Guide

## Current Phase 2 Baseline

This repository now includes an initial Phase 2 security hardening pass focused on safer defaults and traceability:

- TLS remains enabled by default
- JWT signing keys are persisted instead of regenerated on each restart
- Bearer token parsing is strict
- auth lockout and rate limiting remain active
- security-sensitive admin actions are written to the audit log
- configuration values are validated at startup

## Deployment Guidance

- Keep TLS enabled in all shared or remote environments
- Treat `--no-tls` as local-development-only
- Restrict network access to the gRPC port even when TLS is enabled
- Rotate issued client certificates when users or machines change
- Review audit logs regularly for `authenticate`, `create_user`, `delete_user`, `set_password`, `set_acl`, `issue_cert`, `revoke_cert`, `create_backup`, and `restore_backup`
- Use strong passwords and avoid reducing the minimum password length below 12

## Security Notes

- JWT revocation is process-local today; a restart clears the in-memory revoked-token set
- mTLS is enforced when TLS is enabled on the server
- Root and admin operations should be treated as privileged change events and monitored through the audit log

## Recommended Next Steps

- Persist revoked token IDs with expiry-aware cleanup
- Add request fuzzing for auth and query surfaces
- Add certificate rotation workflows
- Add stronger secret management options for production deployments
- Expand audit coverage to include more read-path administrative operations
