use anchor_lang::prelude::*;

declare_id!("6iC2KrndewTWt5kE1Ltga3u1c6ezeMejbZ4zkWuE93zV");

const ANCHOR_DISCRIMINITATOR: usize = 8;

#[program]
pub mod set_favorites {
    use super::*;

    pub fn set_favorites(ctx: Context<SetFavorites>,
                     _number: u64,
                     _color: String,
                      hobbies: Vec<String>,
                    ) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);

        let user_public_key = ctx.accounts.user.key();

          msg!(
            "User {user_public_key}'s favorite number is {_number}, 
            favorite color is: {_color}, and their hobbies are {hobbies:?}",
        );

        ctx.accounts.favorites.set_inner(OtherFavorites {
            _number,
            _color,
            hobbies
        });
        Ok(())
    }
}


#[account]
#[derive(InitSpace)]
pub struct OtherFavorites{
    pub _number: u64,

    #[max_len(50)]
    pub _color: String,

    #[max_len(5,50)]
    pub hobbies: Vec<String>,

}

#[derive(Accounts)]
pub struct SetFavorites<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init_if_needed,
        payer = user,
        space = ANCHOR_DISCRIMINITATOR + OtherFavorites::INIT_SPACE,
        seeds=[b"favorites", user.key().as_ref()],
        bump
    )]
    pub favorites: Account<'info, OtherFavorites>,
    pub system_program: Program<'info, System>,
}
