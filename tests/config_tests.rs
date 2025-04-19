use projet_essaim::config::{MAP_WIDTH, MAP_HEIGHT, FOG_OF_WAR};

#[test]
fn test_constants_are_valid() {
    assert!(MAP_WIDTH > 0);
    assert!(MAP_HEIGHT > 0);
    assert!(FOG_OF_WAR); 
}
