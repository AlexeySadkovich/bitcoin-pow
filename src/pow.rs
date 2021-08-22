pub mod pow {
    use crate::consensus::params::Params;

    pub fn check_proof_of_work(hash: u64, n_bits: u32, params: &Params) -> bool {
        let bn_target: u64 = n_bits as u64;

        if bn_target == 0 || bn_target > (params.pow_limit as u64) {
            return false
        }

        if hash > bn_target {
            return false
        }

        true
    }
}
