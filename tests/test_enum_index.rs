use derive_custom_enum_traits::DeriveIndex;
use custom_enum_traits::EnumIndex;

#[derive(Debug, DeriveIndex, PartialEq, Eq)]
pub enum Example {
    One,
    Two,
    Three,
    Four
}

mod tests {
    use super::*;

    #[test]
    fn test_example_enum_to_index() {
        assert_eq!(Example::One.to_index(), 0);
        assert_eq!(Example::Two.to_index(), 1);
        assert_eq!(Example::Three.to_index(), 2);
    }

    #[test]
    fn test_example_enum_from_index() {
        assert_eq!(Example::from_index(0), Some(Example::One));
        assert_eq!(Example::from_index(1), Some(Example::Two));
        assert_eq!(Example::from_index(2), Some(Example::Three));
        assert_eq!(Example::from_index(3), Some(Example::Four));
        assert_eq!(Example::from_index(4), None);
    }
}