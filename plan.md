# VantaDB Plan

## Vision

VantaDB will aim to become a fast, secure, PostgreSQL-inspired database engine built in Rust.

The long-term goal is not to copy PostgreSQL feature-for-feature on day one. The goal is to deliver the most important database capabilities first, then expand them into a broader engine with stronger security defaults and very high performance.

Target statement:

> VantaDB will become a production-ready database engine with PostgreSQL-like capabilities, optimized for speed, strong durability, and secure-by-default deployment.

## Product Direction

VantaDB should be positioned as:

- A relational and transactional database engine
- Inspired by PostgreSQL in capabilities and operational seriousness
- Built for fast reads, fast writes, and strong security defaults
- Designed for modern applications that want a self-hosted Rust-native database

Near-term target:

- A secure and fast single-node transactional engine with core SQL-style database capabilities

Long-term target:

- A production database that offers much of what developers expect from PostgreSQL, with tighter defaults around security and a strong focus on performance

## What "PostgreSQL-Inspired" Means

This roadmap assumes VantaDB should eventually support the core capabilities people expect from PostgreSQL:

- Tables and schemas
- Typed data
- Primary keys and secondary indexes
- Transactions and isolation guarantees
- Constraints
- Query planning and optimization
- Sorting, filtering, pagination, aggregation, and joins
- Roles, permissions, auditing, and secure networking
- Backup, restore, replication, and operational tooling

This does **not** mean:

- Full PostgreSQL compatibility in the short term
- Immediate support for every SQL feature
- Immediate wire-protocol compatibility
- Trying to outrun PostgreSQL in every workload before the core engine is trustworthy

## Main Priorities

VantaDB should optimize for:

1. Correctness
2. Security
3. Performance
4. Operational reliability
5. Feature growth

That order matters. A database that is fast but loses data, corrupts indexes, or recovers incorrectly is not useful in production.

## Core Principles

1. Correctness before marketing claims
2. Security by default, not by documentation
3. Performance must be benchmarked, not assumed
4. Single-node maturity before distributed maturity
5. Features should be added only when they can be tested and maintained

## Current Reality

VantaDB already has promising building blocks:

- WAL-backed storage
- MVCC concepts
- Index structures
- Transaction manager
- Query and planner modules
- gRPC server layer
- Auth, ACL, TLS, and audit-related pieces
- Early Raft/distributed modules

But it is not yet a production-grade PostgreSQL-style engine because:

- Storage is still too memory-centric
- MVCC is not fully durable across restart
- Indexes are not yet durable on-disk structures
- The query layer is not yet a full SQL-capable execution engine
- Crash recovery, corruption handling, and benchmark coverage are not yet complete
- Operational tooling is still early

## Main Strategy

Build VantaDB in this order:

1. Durable storage core
2. Secure production defaults
3. Relational foundations
4. Query execution and optimizer improvements
5. Performance engineering
6. Operational readiness
7. Distributed maturity

## Phase 1: Durable Storage Core

Objective:

- Make one node correct, durable, and recoverable under real failure conditions

Work items:

- Redesign the storage engine around a real on-disk architecture
- Choose the long-term storage layout:
  - B+Tree/page engine if OLTP and relational workloads are primary
  - LSM-based design only if write-heavy tradeoffs are acceptable
- Define stable page, WAL, and metadata formats
- Persist MVCC/version metadata
- Guarantee committed writes survive restart
- Implement deterministic crash recovery
- Add corruption detection and torn-write handling
- Add checksums for WAL, pages, and critical metadata
- Add compaction or vacuum strategy as needed by the chosen architecture

Deliverables:

- Stable WAL format
- Stable on-disk data format
- Recovery test harness
- Documented durability guarantees

Success criteria:

- No acknowledged committed write is lost after clean or unclean restart
- Recovery is deterministic and repeatable
- Corruption is detected safely rather than silently accepted

## Phase 2: Secure Production Defaults

Objective:

- Make VantaDB safer to deploy than the average default database setup

Work items:

- Make TLS-first deployment standard
- Harden auth flows and token/session validation
- Improve role and permission enforcement
- Add secure default config values
- Strengthen audit logging coverage
- Add lockout, rate limits, and abuse protection
- Improve certificate and secret handling
- Add parser and request fuzzing
- Write a threat model for server, storage, and admin surfaces

Deliverables:

- Hardened default configuration
- Security guide
- Threat model
- Audit log schema and operational guidance

Success criteria:

- Default deployments are encrypted and authenticated
- Dangerous defaults are removed or explicitly blocked
- Security-sensitive actions are traceable

## Phase 3: Relational Foundations

Objective:

- Build the core database capabilities people expect from a PostgreSQL-like system

Work items:

- Define table metadata and schemas clearly
- Support typed columns beyond raw document storage
- Add primary key enforcement
- Add unique constraints
- Add nullable and non-nullable semantics
- Add default values
- Add basic schema change paths
- Define catalog/system metadata structures
- Decide how JSON support fits alongside typed relational storage

Deliverables:

- Stable schema/catalog design
- Typed table support
- Core constraint enforcement

Success criteria:

- Applications can model relational data safely
- Constraints are enforced consistently
- Metadata survives restart and upgrade safely

## Phase 4: Query Execution and Optimizer

Objective:

- Turn the query path into a serious engine rather than a thin filter layer

Work items:

- Add a structured query model
- Expand planner capabilities
- Add table scans, index scans, and range scans
- Add projections, filters, sorting, and pagination
- Add aggregations with predictable execution behavior
- Add join support in staged form
- Add statistics and selectivity tracking
- Improve explain-plan output
- Add cost-based planning over time

Deliverables:

- Query execution model
- Better planner
- Explain output that reflects actual execution decisions

Success criteria:

- Common queries are explainable, testable, and benchmarked
- Indexed paths materially outperform full scans
- Query behavior is predictable under load

## Phase 5: Performance Engineering

Objective:

- Make VantaDB measurably fast for transactional application workloads

Work items:

- Build repeatable benchmark suites
- Benchmark inserts, updates, deletes, point reads, range reads, joins, and aggregations
- Profile CPU, allocations, I/O, serialization, and lock contention
- Reduce unnecessary allocations and copying
- Add batching and group commit where safe
- Improve buffer/cache behavior
- Optimize index maintenance and query hot paths
- Add regression detection in CI

Deliverables:

- Benchmark harness
- Baseline performance reports
- Regression alerts

Success criteria:

- Performance is measured continuously
- Regressions are detected early
- Release targets include latency and throughput goals

## Phase 6: Operational Readiness

Objective:

- Make VantaDB manageable in real application environments

Work items:

- Add backup and restore tooling
- Add consistency checking and repair flows
- Add metrics, tracing, and health endpoints
- Improve structured logging
- Add admin diagnostics
- Validate configuration strictly
- Define upgrade compatibility rules
- Add migration and maintenance tooling

Deliverables:

- Backup and restore commands
- Health and metrics documentation
- Upgrade playbook

Success criteria:

- Operators can deploy, observe, back up, restore, and upgrade safely
- Operational risks are documented and testable

## Phase 7: Distributed Maturity

Objective:

- Add distributed strength only after the single-node engine is trustworthy

Work items:

- Validate Raft behavior under load and failure
- Clarify consistency guarantees
- Add snapshot transfer and log compaction
- Improve failover behavior and diagnostics
- Benchmark replication overhead and recovery time
- Define operational expectations for cluster management

Deliverables:

- Cluster deployment model
- Consistency model documentation
- Failover and replication test coverage

Success criteria:

- Cluster behavior is predictable
- Failover and replay are verified under realistic tests

## What To Delay

These should not be top priority right now:

- Full PostgreSQL wire compatibility
- Full stored procedure support
- Every advanced SQL feature
- Large extension ecosystems
- Broad compatibility claims before correctness and security are proven

## Engineering Standards

Every major subsystem should have:

- Unit tests
- Integration tests
- Crash and recovery tests where relevant
- Benchmark coverage
- Documentation of guarantees and tradeoffs

Every performance claim should include:

- Workload definition
- Hardware details
- Dataset shape
- Read/write mix
- Throughput and latency numbers

## Initial Milestones

### Milestone A: Durable Core

Target:

- Stable WAL and recovery model
- Persistent MVCC design
- Deterministic recovery tests

### Milestone B: Secure Alpha

Target:

- Hardened TLS/auth defaults
- ACL review
- Audit log coverage
- Safer configuration defaults

### Milestone C: Relational Base

Target:

- Typed tables
- Keys and constraints
- Stable catalog metadata

### Milestone D: Query Engine

Target:

- Better planner
- Table and index scans
- Sorting, filtering, and aggregation improvements

### Milestone E: Measured Speed

Target:

- Benchmark suite
- Latency and throughput targets
- Regression checks

## Definition of Production Ready

VantaDB can be called production ready only when it has:

- Documented durability guarantees
- Verified crash recovery
- Persistent storage and indexes
- Secure default deployment
- Strong auth and permission enforcement
- Backup and restore support
- Metrics and observability
- Upgrade compatibility rules
- Repeatable benchmark evidence
- Clear operational documentation

## Summary

VantaDB should aim to become:

- Fast
- Secure
- Durable
- Relational
- Operationally reliable

The winning path is:

1. Build a trustworthy storage core
2. Make security stronger than the default experience people expect
3. Add relational foundations and query power
4. Prove performance with benchmarks
5. Expand breadth only when the foundation is solid
