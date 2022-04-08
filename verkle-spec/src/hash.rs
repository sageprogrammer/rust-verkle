use verkle_trie::{committer::Committer,Fr,from_to_bytes::ToBytes};
use ethereum_types::{H256};
use crate::Hasher;

pub struct PedersenHasher;
impl Hasher for PedersenHasher{
    fn hash64(bytes64: [u8; 64]) -> H256{
        use verkle_trie::committer::test::TestCommitter;
        // let hasher = PedersenHasher::new(TestCommitter);
        let chunks = crate::util::chunk64(bytes64);
        let fr_data:Vec<_> = chunks.iter().map(|x| -> Fr {Fr::from(*x)}).collect();
        let bytes = TestCommitter.commit_lagrange(&fr_data[..]).to_bytes();
        return H256::from_slice(&bytes[..]);
        // hasher.hash(&chunks[..])
    }
}

#[cfg(test)]
mod test{
    use crate::hash::{PedersenHasher,Hasher};
    use hex::FromHex;
    use crate::H256;
    #[test]
    fn hash_test(){
        let tests = [
            (
                <[u8;64]>::from_hex("00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000").expect("Decoding failed"),
                <[u8;32]>::from_hex("695921dca3b16c5cc850e94cdd63f573c467669e89cec88935d03474d6bdf9d4").expect("Decoding failed")
            ),
            (
                <[u8;64]>::from_hex("00020300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000").expect("Decoding failed"),
                <[u8;32]>::from_hex("5010fabfb319bf84136db68445972cdd5476ff2fbf3e5133330b3946b84b4e6a").expect("Decoding failed")
            )
            // (
            //     <[u8;64]>::from_hex("0071562b71999873db5b286df957af199ec946170000000000000000000000000000000000000000000000000000000000000000000000000000000000000000").expect("Decoding failed"),
            //     <[u8;32]>::from_hex("f56e644224f4576490cfe0de1424a4532212529bfe374713d84e7d7e8e927200").expect("Decoding failed")
            // )
        ];
        // print!("{:?}",<[u8;64]>::from_hex("0071562b71999873db5b286df957af199ec946170000000000000000000000000000000000000000000000000000000000000000000000000000000000000000").expect("Decoding failed"));
        for (input,output) in tests.iter(){
            assert_eq!(PedersenHasher::hash64(*input),H256::from_slice(&output[..]));
        }
    }
}