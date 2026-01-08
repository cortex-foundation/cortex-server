// //자동 생성된 모듈 로드
// pub mod packets {
//     include!(concat!(env!("OUT_DIR"), "/protos_mod.rs"));
// }


// 사용자 로직
pub mod crypto {
    pub fn verify_signature(_data: &[u8], _signature: &str) -> bool {
        true
    }
}