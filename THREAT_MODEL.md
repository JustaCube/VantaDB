# VantaDB Threat Model

## Scope

This first-pass threat model covers the current single-node server, authentication layer, TLS/certificate handling, audit logging, and backup/admin RPC surfaces.

## Primary Assets

- user credentials and password hashes
- JWT signing secret
- issued client certificates and revocation state
- application data in storage and backups
- audit trail integrity

## Trust Boundaries

- unauthenticated network client to gRPC server
- authenticated application client to database/admin APIs
- local filesystem holding storage, keys, and backups
- privileged operator actions such as user management, certificate issuance, and restore

## Main Threats

- credential stuffing and repeated login attempts
- token misuse from malformed or weak auth parsing
- silent loss of session validity after restart due to ephemeral signing keys
- unauthorized admin changes without traceability
- insecure runtime configuration values weakening auth or availability controls
- backup or restore misuse by privileged users

## Current Mitigations

- Argon2 password hashing
- lockout tracking and auth rate limiting
- TLS by default with mTLS when enabled
- persistent JWT signing secret
- strict Bearer token parsing
- audit logging for core authentication and privileged admin actions
- startup config validation for insecure or invalid settings

## Known Gaps

- revoked JWT IDs are not yet persisted across restart
- no dedicated secret manager integration yet
- no formal fuzzing harness landed yet
- certificate lifecycle controls are still basic
- distributed threat surfaces are not fully modeled yet

## Follow-Up Work

- persist token revocation metadata
- add auth/parser/request fuzz tests
- document key rotation and recovery procedures
- add tamper-evident audit export or checksum chaining
- extend the model for cluster membership, replication, and inter-node auth
