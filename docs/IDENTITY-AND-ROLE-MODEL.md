# Identity and Role Model

## Roles

| Role | Privileges |
|---|---|
| examiner | create/open case, run authorized methods, draft findings |
| reviewer | technical/admin review; cannot self-approve own findings when policy requires separation |
| administrator | local install config, pack trust policy, user provisioning |
| validation operator | run validation corpora, seal validation records |
| observer | read-only case view |

## Actor identity

OS identity is an input, not the sole stable actor identifier. Lab actor records store: actor_uuid, display name, role bindings, authentication method, competency/method authorizations, and revocation state.

Privileged operations record actor, role, authentication method, and approval references when dual control is required.
