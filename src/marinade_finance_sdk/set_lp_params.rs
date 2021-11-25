#![allow(unused)]
fn main() {
    pub fn set_lp_params(
        state: &impl Located<State>,
        min_fee: Fee,
        max_fee: Fee,
        liquidity_target: u64,
    ) -> Instruction {
        let accounts = accounts::SetLpParams {
            state: state.key(),
            admin_authority: state.as_ref().admin_authority,
        }
        .to_account_metas(None);

        let data = instruction::SetLpParams {
            min_fee,
            max_fee,
            liquidity_target,
        };

        Instruction {
            program_id: marinade_finance::ID,
            accounts,
            data: data.data(),
        }
    }
}