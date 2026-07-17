//! Merge timeline events from multiple sources (Day 36).

use lab_artifacts::TimelineEvent;

/// Merge and sort timeline events by occurred_at_utc then kind.
pub fn merge_timeline(batches: &[Vec<TimelineEvent>]) -> Vec<TimelineEvent> {
    let mut out = Vec::new();
    for b in batches {
        out.extend(b.iter().cloned());
    }
    out.sort_by(|a, b| {
        a.occurred_at_utc
            .cmp(&b.occurred_at_utc)
            .then(a.kind.cmp(&b.kind))
            .then(a.summary.cmp(&b.summary))
    });
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_sorts() {
        let a = vec![TimelineEvent {
            occurred_at_utc: "2026-01-02T00:00:00Z".into(),
            kind: "b".into(),
            summary: "later".into(),
            provenance_ref: "1".into(),
        }];
        let b = vec![TimelineEvent {
            occurred_at_utc: "2026-01-01T00:00:00Z".into(),
            kind: "a".into(),
            summary: "earlier".into(),
            provenance_ref: "2".into(),
        }];
        let m = merge_timeline(&[a, b]);
        assert_eq!(m[0].summary, "earlier");
        assert_eq!(m[1].summary, "later");
    }
}
