use crate::instructions::*;
use anchor_lang::prelude::*;

pub mod events;
pub mod instructions;
pub mod states;
pub mod errors;

declare_id!("BJy8cEtQ6PoXQUCgcCM1vGbi2PHWzn64YiMb3gnW92VC");

#[program]
pub mod soltip {
    use super::*;

    pub fn initialize_profile(
        ctx: Context<InitializeCreatorProfile>,
        name: String,
        email: String,
        bio: String,
        about_me: String,
    ) -> Result<()> {
        initialize_creator_profile(ctx, name, email, bio, about_me)
    }

    pub fn update_profile(
        ctx: Context<UpdateCreatorProfile>,
        name: Option<String>,
        email: Option<String>,
        bio: Option<String>,
        about_me: Option<String>
    ) -> Result<()> {
        update_creator_profile(ctx, name, email, bio, about_me)
    }

    pub fn close_profile(ctx: Context<CloseCreatorProfile>) -> Result<()> {
        close_creator_profile(ctx)
    }

    // pub fn set_coin_value(ctx: Context) -> Result<()> {
    //     todo!()
    // }

    // pub fn withdraw_coins(ctx: Context) -> Result {
    //     todo!()
    // }

    // pub fn support_creator(ctx: Context) -> Result<()> {
    //     todo!()
    // }
}
