// use super::*;
//
// static ROM_CASTLEBOY: &'static [u8] = include_bytes!("test-roms/CastleBoy/CastleBoy.hex");
//
// #[test]
// fn test_serialize() {
//     let mut core = Core::new();
//     let mut buf = vec![0; core.serialize_len().unwrap()];
//     core.serialize(&mut buf).unwrap();
//
//     let mut core2 = Core::new();
//     core2.deserialize(&buf).unwrap();
//
//     assert_eq!(core, core2);
// }
