use elias_fano_rust::elias_fano::EliasFano;

#[test]
/// Check that elias fano runs considering a lot of possible combinations.
fn test_errors() {
    assert!(
        EliasFano::<10>::from_iter([9, 8, 7, 6, 5, 4, 3, 2, 1].iter().cloned(), 10, 10).is_err()
    );
    //assert!(EliasFano::from_iter([].iter().cloned(), 5, 5).is_err());
    let ef = EliasFano::<10>::from_vec(&[1, 5, 8, 9]).unwrap();
    assert!(ef.select(100000).is_err());
}
