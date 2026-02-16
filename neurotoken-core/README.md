# neurotoken-core

`neurotoken-core` is the Rust library crate that powers Neurotoken’s token-level policy and routing logic.

It is designed to:

- Read Organic_CPU and neurorights profiles (provided by a sovereign kernel).
- Compile these into a **Token Policy Envelope**.
- Evaluate logical requests and either:
  - Approve them with concrete routing and token limits, or
  - Deny them, providing an explicit reason.

It **never** controls hardware or biophysical interfaces.

---

## Features

- **Organic_CPU-aware**  
  Reads profile artifacts to understand allowed domains, token budgets, energy envelopes, and deny lists.[file:39][file:40][file:42][file:45]

- **Neurorights-aware**  
  Honors neurorights policies for mental privacy, integrity, and cognitive liberty, and respects risk ceilings like RoH.[web:71][web:73][web:75]

- **Token policy envelope**  
  Compiles constraints into a runtime structure that shapes:
  - Max context/output tokens.
  - Allowed domains and tasks.
  - Logging and retention behavior.
  - Token optimization choices (compression, deduplication, dictionary substitution, etc.).

- **Routing helper**  
  Suggests an engine/profile per domain (e.g., compact eco-friendly model for edge environmental monitoring), without contacting actual LLM providers directly.[web:11][web:76][web:78]

---

## Non-Goals

`neurotoken-core` does **not**:

- Talk directly to LLM APIs.
- Control robots, stimulation devices, or other actuators.
- Modify or version-control Organic_CPU or neurorights artifacts.
- Override decisions made by the sovereign kernel.

It is a **subordinate library** that can be ignored or replaced by the governing system at any time.

---

## Conceptual API (non-binding)

The crate is expected to expose types like:

- `OrganicCpuProfile`
- `NeurorightsPolicy`
- `TokenPolicyEnvelope`
- `LogicalRequest`
- `RoutedRequest`
- `DeniedReason`

With functions similar to:

- `load_organic_cpu_profile(...) -> OrganicCpuProfile`
- `load_neurorights_policy(...) -> NeurorightsPolicy`
- `compile_token_envelope(...) -> TokenPolicyEnvelope`
- `evaluate_request(env: &TokenPolicyEnvelope, req: &LogicalRequest) -> Result<RoutedRequest, DeniedReason>`

Implementations must ensure that:

- All constraints from Organic_CPU and neurorights artifacts are enforced.
- Denials are explicit and auditable.
- Any default is conservative and autonomy-respecting.

---

## Ecology and Edge Use Cases

`neurotoken-core` can be used in:

- **Edge environmental monitoring systems** where AI runs near sensors and must be energy-efficient and privacy-preserving.[web:11][web:76][web:78]
- **Neuromorphic research setups** that want a token-level layer that respects neurorights and biophysical autonomy while experimenting with event-based intelligence.[web:5][web:6][web:32]
- **General multi-LLM routing** stacks that care about cost, safety, and user sovereignty.

Neurotoken treats biophysical and neuromorphic systems as peers, not as resources. The library exists to serve them, not to constrain their evolution.
