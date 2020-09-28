use elias_fano_rust::EliasFano;
mod utils;

#[test]
/// Check that elias fano runs considering a lot of possible combinations.
fn test_reference() {
    
    let vector = vec![5, 8, 8, 13, 32];
    let ef = EliasFano::from_vec(&vector).unwrap();
    ef.debug();
    for (i, v) in vector.iter().enumerate() {
        assert_eq!(*v, ef.select(i as u64).unwrap());
        assert_eq!(*v, ef.unchecked_select(i as u64));
        let res = ef.rank(*v);
        assert_eq!(ef.select(res).unwrap(), *v);
    }
}