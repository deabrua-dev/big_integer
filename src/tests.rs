#[cfg(test)]
mod test {
    use crate::big_int::BigInt;

    #[test]
    fn not_test() {
        let mut test_1 = BigInt::new();
        let mut test_2 = BigInt::new();
        test_1.set_hex(String::from("33ced2c76b26cae94e162c4c0d2c0ff7c13094b0185a3c122e732d5ba77efebc")).unwrap();
        test_2.set_hex(String::from("cc312d3894d93516b1e9d3b3f2d3f0083ecf6b4fe7a5c3edd18cd2a458810143")).unwrap();
        let result_1 = (!test_1).get_hex();
        let result_2 = (!test_2).get_hex();
        assert_eq!("cc312d3894d93516b1e9d3b3f2d3f0083ecf6b4fe7a5c3edd18cd2a458810143".to_string().to_lowercase(), result_1);
        assert_eq!("33ced2c76b26cae94e162c4c0d2c0ff7c13094b0185a3c122e732d5ba77efebc".to_string().to_lowercase(), result_2);
    }

    #[test]
    fn xor_test() {
        let mut test_1 = BigInt::new();
        let mut test_2 = BigInt::new();
        test_1.set_hex(String::from("51bf608414ad5726a3c1bec098f77b1b54ffb2787f8d528a74c1d7fde6470ea4")).unwrap();
        test_2.set_hex(String::from("403db8ad88a3932a0b7e8189aed9eeffb8121dfac05c3512fdb396dd73f6331c")).unwrap();
        let result = (test_1 ^ test_2).get_hex();
        assert_eq!("1182d8299c0ec40ca8bf3f49362e95e4ecedaf82bfd167988972412095b13db8".to_string(), result);
    }

    #[test]
    fn or_test() {
        let mut test_1 = BigInt::new();
        let mut test_2 = BigInt::new();
        test_1.set_hex(String::from("51bf608414ad5726a3c1bec098f77b1b54ffb2787f8d528a74c1d7fde6470ea4")).unwrap();
        test_2.set_hex(String::from("403db8ad88a3932a0b7e8189aed9eeffb8121dfac05c3512fdb396dd73f6331c")).unwrap();
        let result = (test_1 | test_2).get_hex();
        assert_eq!("51bff8ad9cafd72eabffbfc9befffffffcffbffaffdd779afdf3d7fdf7f73fbc".to_string(), result);
    }

    #[test]
    fn and_test() {
        let mut test_1 = BigInt::new();
        let mut test_2 = BigInt::new();
        test_1.set_hex(String::from("51bf608414ad5726a3c1bec098f77b1b54ffb2787f8d528a74c1d7fde6470ea4")).unwrap();
        test_2.set_hex(String::from("403db8ad88a3932a0b7e8189aed9eeffb8121dfac05c3512fdb396dd73f6331c")).unwrap();
        let result = (test_1 & test_2).get_hex();
        assert_eq!("403d208400a113220340808088d16a1b10121078400c1002748196dd62460204".to_string(), result);
    }

    #[test]
    fn shr_test() {
        let mut test_1 = BigInt::new();
        test_1.set_hex(String::from("51bf608414ad5726a3c1bec098f77b1b54ffb2787f8d528a74c1d7fde6470ea4")).unwrap();
        let result = (test_1 >> 16).get_hex();
        assert_eq!("51bf608414ad5726a3c1bec098f77b1b54ffb2787f8d528a74c1d7fde647".to_string(), result);
    }

    #[test]
    fn shl_test() {
        let mut test_1 = BigInt::new();
        test_1.set_hex(String::from("51bf608414ad5726a3c1bec098f77b1b54ffb2787f8d528a74c1d7fde6470ea4")).unwrap();
        let result = (test_1 << 15).get_hex();
        assert_eq!("28dfb0420a56ab9351e0df604c7bbd8daa7fd93c3fc6a9453a60ebfef32387520000".to_string(), result);
    }

    #[test]
    fn add_test() {
        let mut test_1 = BigInt::new();
        let mut test_2 = BigInt::new();
        test_1.set_hex(String::from("36f028580bb02cc8272a9a020f4200e346e276ae664e45ee80745574e2f5ab80")).unwrap();
        test_2.set_hex(String::from("70983d692f648185febe6d6fa607630ae68649f7e6fc45b94680096c06e4fadb")).unwrap();
        let result = (test_1 + test_2).get_hex();
        assert_eq!("a78865c13b14ae4e25e90771b54963ee2d68c0a64d4a8ba7c6f45ee0e9daa65b".to_string(), result);
    }

    #[test]
    fn sub_test() {
        let mut test_1 = BigInt::new();
        let mut test_2 = BigInt::new();
        test_1.set_hex(String::from("33ced2c76b26cae94e162c4c0d2c0ff7c13094b0185a3c122e732d5ba77efebc")).unwrap();
        test_2.set_hex(String::from("22e962951cb6cd2ce279ab0e2095825c141d48ef3ca9dabf253e38760b57fe03")).unwrap();
        let result = (test_1 - test_2).get_hex();
        assert_eq!("10e570324e6ffdbc6b9c813dec968d9bad134bc0dbb061530934f4e59c2700b9".to_string(), result);
    }

    #[test]
    fn modulo_test() {
        let mut test_1 = BigInt::new();
        let mut test_2 = BigInt::new();
        test_1.set_hex(String::from("33ced2c76b26cae94e162c4c0d2c0ff7c13094b0185a3c122e732d5ba77efebc")).unwrap();
        test_2.set_hex(String::from("a78865c13b14ae4e25e90771b54963ee2d68c0a64d4a8ba7c6f45ee0e9daa65b")).unwrap();
        let result = (test_1 % test_2).get_hex();
        assert_eq!("33ced2c76b26cae94e162c4c0d2c0ff7c13094b0185a3c122e732d5ba77efebc".to_string(), result);
    }

    #[test]
    fn eq_positive_test() {
        let mut test_1 = BigInt::new();
        let mut test_2 = BigInt::new();
        test_1.set_hex(String::from("33ced2c76b26cae94e162c4c0d2c0ff7c13094b0185a3c122e732d5ba77efebc")).unwrap();
        test_2.set_hex(String::from("33ced2c76b26cae94e162c4c0d2c0ff7c13094b0185a3c122e732d5ba77efebc")).unwrap();
        let result = test_1 == test_2;
        assert_eq!(true, result);
    }

    #[test]
    fn eq_negative_test() {
        let mut test_1 = BigInt::new();
        let mut test_2 = BigInt::new();
        test_1.set_hex(String::from("cc312d3894d93516b1e9d3b3f2d3f0083ecf6b4fe7a5c3edd18cd2a458810143")).unwrap();
        test_2.set_hex(String::from("cc312d3894d93516b1e9d3b3f2d3f0083ecf6b4fe7a5c3edd18cd2a458810143")).unwrap();
        let result = test_1 == test_2;
        assert_eq!(true, result);
    }

    #[test]
    fn not_eq_test() {
        let mut test_1 = BigInt::new();
        let mut test_2 = BigInt::new();
        test_1.set_hex(String::from("cc312d3894d93516b1e9d3b3f2d3f0083ecf6b4fe7a5c3edd18cd2a458810143")).unwrap();
        test_2.set_hex(String::from("33ced2c76b26cae94e162c4c0d2c0ff7c13094b0185a3c122e732d5ba77efebc")).unwrap();
        let result = test_1 != test_2;
        assert_eq!(true, result);
    }


    #[test]
    fn neg_sign_store_test() {
        let mut test_1 = BigInt::new();
        test_1.set_hex(String::from("cc312d3894d93516b1e9d3b3f2d3f0083ecf6b4fe7a5c3edd18cd2a458810143")).unwrap();
        let result = (test_1).get_hex();
        assert_eq!("cc312d3894d93516b1e9d3b3f2d3f0083ecf6b4fe7a5c3edd18cd2a458810143".to_string(), result);
    }
}