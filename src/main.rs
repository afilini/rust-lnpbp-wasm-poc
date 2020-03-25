use std::os::raw::{c_char, c_uint, c_ulonglong};
use std::ffi::CStr;
use std::ffi::CString;
use std::str::FromStr;

use lnpbp::bitcoin as bitcoin;
use bitcoin::Amount;
use bitcoin::hashes::hex::{FromHex, ToHex};
use bitcoin::consensus::deserialize;
use bitcoin::secp256k1::PublicKey;

use lnpbp::AsSlice;
use lnpbp::cmt::tx::{TxContainer, TxCommitment};
use lnpbp::cmt::txout::{TxoutContainer, TxoutCommitment};
use lnpbp::cmt::committable::{EmbedCommittable, Verifiable};

fn my_string_safe(i: *mut c_char) -> String {
  unsafe {
      CStr::from_ptr(i).to_string_lossy().into_owned()
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Message<'a>(&'a str);
impl AsSlice for Message<'_> {
    fn as_slice(&self) -> &[u8] {
        &self.0.as_bytes()
    }
}

// from rust-lnpbp::cmt::tx::test::test_ability_to_commit
#[no_mangle]
pub fn lnpbp_test(tx: *mut c_char, fee: c_ulonglong, entropy: c_uint, pubkey: *mut c_char, message: *mut c_char) -> *mut c_char {
    let tx = deserialize(Vec::from_hex(my_string_safe(tx).as_str())
        .unwrap().as_slice()).unwrap();

    let pubkey = my_string_safe(pubkey);
    let container = TxContainer {
        tx,
        fee: Amount::from_sat(fee),
        entropy: entropy,
        txout_container: TxoutContainer::PubkeyHash(PublicKey::from_str(pubkey.as_str()).unwrap())
    };

    println!("tx {:#?}", container.tx);
    println!("fee {:#?}", container.fee);
    println!("entropy {:#?}", container.entropy);

    let message = my_string_safe(message);
    println!("message {:#?}", message);

    let msg = Message(message.as_str());

    let commitment: TxCommitment = msg.commit_embed(container).unwrap();
    assert_eq!(msg.verify(&commitment), true);

    if let TxoutCommitment::PublicKey(pk) = commitment.tweaked {
	    CString::new(pk.tweaked.to_hex().as_str())
	    	.unwrap()
	    	.into_raw()
    } else {
        unimplemented!();
    }
}

// will be stripped by emscripten since we don't export it
fn main() {
}
