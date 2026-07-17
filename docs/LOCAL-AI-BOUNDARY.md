# Local AI Boundary

**ADR-007:** ACCEPTED  
**Priority:** P0 optional (ADR-014)

## Network

Ollama and LM Studio adapters connect only to explicitly configured **loopback** endpoints. Non-loopback endpoints are rejected.

## Data minimization

Send derived/minimized content by default; never raw evidence bytes by default. Operators may authorize specific derived excerpts with audit.

## Output requirements

Every AI output records model identity, endpoint type, prompt-template version, source citations, parameters, timestamp, acceptance state, and disclaimer.

## Authority limits

AI may suggest. AI may **not**: establish evidence integrity; alter provenance; automatically close findings; issue an examiner conclusion; approve reports; access shell, Internet, arbitrary paths, or evidence writes.
