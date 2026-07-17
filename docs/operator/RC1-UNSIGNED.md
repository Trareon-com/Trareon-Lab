# Soft tag `v1.0.0-rc1-unsigned`

**Purpose:** freeze an Official *candidate* without claiming Official Production.  
**Signing:** none  
**Windows lab:** still required later per `docs/WINDOWS-LAB-QUEUE.md`

Create after repo Official track is complete:

```bash
git tag -a v1.0.0-rc1-unsigned -m "Official candidate RC1 (unsigned); O1-O12 open"
git push origin v1.0.0-rc1-unsigned
```

Do **not** create annotated `v1.0.0` until `release-evidence/OFFICIAL-1.0.0/gather.sh` exits 0.
