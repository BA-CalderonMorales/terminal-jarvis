//! A useful version may follow banners; accept the budget, reject excess.
use super::*;

#[test]
fn finite_banner_output_respects_the_capture_boundary() {
    for (width, expected) in [(2047, Some("v1.2")), (2048, None)] {
        let script = format!(
            "i=0; while [ \"$i\" -lt 31 ]; do printf '%2047s\\n' x; \
             i=$((i+1)); done; printf '%{width}s\\n' v1.2"
        );
        assert_eq!(version("sh", &["-c".into(), script]).as_deref(), expected);
    }
}
