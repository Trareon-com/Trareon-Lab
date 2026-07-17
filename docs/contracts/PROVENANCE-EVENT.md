# Provenance Event Contract

**Schema version:** provenance-event-1

Provenance records evidence-transforming operations and derived-object lineage.

Required envelope fields on every event: `schema_version`, `event_uuid`, `case_uuid`, `actor_id`, `process_id`, `timestamp_utc`, `timestamp_utc_offset_minutes`, `monotonic_seq`, `previous_event_digest`, `payload_digest`, `software_build_identity`.

Payload includes: operation type, input evidence UUIDs, output derived UUIDs, method/tool versions, parameters digest, coverage impact.
