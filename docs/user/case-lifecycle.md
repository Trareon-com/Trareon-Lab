# Case lifecycle

Exact states: CREATED, OPEN, READ_ONLY, RECOVERY_REQUIRED, CLOSED, ARCHIVED.

One exclusive process lock per case. Stale dead-PID locks enter RECOVERY_REQUIRED.
