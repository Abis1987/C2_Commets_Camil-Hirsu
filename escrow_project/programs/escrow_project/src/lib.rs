pub mod constants;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;
declare_id!("5nU5yGrnUW9ApWwtg51jNA2LQ7rX3nvKQ5jBVi5ewCEA");

#[program]
pub mod escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }

    pub fn make_offer(
            context: Context<MakeOffer>,
            id: u64,
            token_a_offered_amount: u64,
            token_b_wanted_amount: u64,
        ) -> Result<()> {

        instructions::make_offer::send_offered_tokens_to_vault(&context,token_a_offered_amount)?;
        instructions::make_offer::save_offer(context, id, token_b_wanted_amount)
    }

     pub fn take_offer(context: Context<TakeOffer>) -> Result<()> {
        instructions::take_offer::send_wanted_tokens_to_maker(&context)?;
        instructions::take_offer::withdraw_and_close_vault(context)
    }

}
