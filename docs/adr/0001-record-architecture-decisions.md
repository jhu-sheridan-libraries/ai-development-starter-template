# 1. Record architecture decisions

* Status: accepted
* Date: 2026-08-10

## Context and Problem Statement

We need to record the architectural and technical decisions made on this
project so that future contributors (including future versions of
ourselves) understand not just *what* was built, but *why* — especially
for decisions that involved tradeoffs, rejected alternatives, or
constraints that aren't obvious from reading the code alone.

## Decision Drivers

* Decisions and their rationale should be discoverable without relying on
  chat history, tribal knowledge, or the original author's memory.
* The format should be lightweight enough to actually get used.

## Considered Options

* No formal record — rely on commit messages and code comments.
* A wiki or external document.
* Architecture Decision Records (ADRs) committed alongside the code, in
  MADR format.

## Decision Outcome

Chosen option: "Architecture Decision Records committed alongside the
code, in MADR format", because it keeps decisions version-controlled,
co-located with the code they affect, and reviewable through the same pull
request process as everything else.

## Consequences

* Every significant architectural or technical decision going forward
  should get its own numbered ADR in this directory.
* ADRs are not edited after acceptance; superseding decisions get a new
  ADR that references the one it replaces.
