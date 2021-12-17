use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

/*
add a comments section to any pubkey
natively mint the comment into an asset

so, with this, if u wanted to get all the comments in the linked list, u would have to query for comments at id
then check the number of links, query for all the links, pull the data into one struct, and that's your comment section

interesting
*/

#[program]
pub mod comments {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

//from seed ["comments", pubkey]
#[account]
pub struct Comments {
    //pubkey for the account that these comments are associated with
    pubkey: Pubkey,
    //number of links in the comments linked-list
    length: u32,
}

//from seed ["link", acct_pubkey, index]
#[account]
pub struct CommentLink {
    comments: [Comment; 20],
    bump: u8,
}

#[derive(Copy, Clone, AnchorDeserialize, AnchorSerialize)]
pub struct Comment {
    creator: Pubkey,
    body: [u8; 140],
    has_artifact: bool,
}

impl Default for Comment {
    fn default() -> Comment {
        Comment {
            creator: Pubkey::default(),
            body: [0; 140],
            has_artifact: false,
        }
    }
}

//when u mint a comment into an artifact --- u will set these fields
// from seed [ "artifact", comment_pubkey ]
pub struct ArtifactAttribution {
    //address of the comment
    comment: Pubkey,
    //mint for the NFT
    mint: Pubkey,
}

/*
how are people paying arweave txn fees?
like what do they use to do that, or is arweave so cheap that it doesn't matter
bc if u have a client that subsidizes your pushes to arweave couldn't somebody just rip a bunch of txns to arweave for free? or worse just drain the account
i want to play around with it but i literally can't get AR atm
*/
