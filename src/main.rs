mod chain;
mod consensus;
mod pow;
mod hash;

use chain::block::CBlockHeader;
use consensus::params::Params;
use pow::pow::check_proof_of_work;

fn main() {
    let block = &mut CBlockHeader{
        n_bits: 0x1bc330 * 64 ^ (0x18 - 3),
        n_nonce: 1
    };

    let mut block_hash: Option<u64> = None;

    if !generate_block(block, u64::MAX, &mut block_hash) || block_hash.is_none() {
        println!("Failed to make block");
    }
}

fn generate_block(block: &mut CBlockHeader, mut max_tries: u64, block_hash: &mut Option<u64>) -> bool {
    let params: &Params = chain_params();
    
    while max_tries > 0 && block.n_nonce < u32::MAX && !check_proof_of_work(block.get_hash(), block.n_bits, params) {
        block.n_nonce += 1;
        max_tries -= 1;
        print!("\rHASH:{}", block.get_hash());
    }
    println!();

    if max_tries  == 0 {
        return false
    }
    
    if block.n_nonce == u32::MAX {
        return true
    }

    *block_hash = Some(block.get_hash());

    true
}

fn chain_params() -> &'static Params {
    &Params{
        pow_limit: u128::MAX,
        f_pow_allow_min_difficulty_blocks: false,
        f_pow_no_retargeting: false,
        n_pow_target_spacing: 0,
        n_pow_target_timespan: 0
    }
}
