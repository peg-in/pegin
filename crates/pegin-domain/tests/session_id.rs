use pegin_domain::session_id::SessionId;

#[test]
fn new_v4_produces_unique_ids() {
    let a = SessionId::new_v4();
    let b = SessionId::new_v4();
    assert_ne!(a, b);
}

#[test]
fn display_is_standard_uuid_format() {
    assert_eq!(SessionId::new_v4().to_string().len(), 36);
}
