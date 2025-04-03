// Here we export some useful types and functions for interacting with the Anchor program.
import { AnchorProvider, Program } from '@coral-xyz/anchor'
import { Cluster, PublicKey } from '@solana/web3.js'
import VotingdapppIDL from '../target/idl/votingdappp.json'
import type { Votingdappp } from '../target/types/votingdappp'

// Re-export the generated IDL and type
export { Votingdappp, VotingdapppIDL }

// The programId is imported from the program IDL.
export const VOTINGDAPPP_PROGRAM_ID = new PublicKey(VotingdapppIDL.address)

// This is a helper function to get the Votingdappp Anchor program.
export function getVotingdapppProgram(provider: AnchorProvider, address?: PublicKey) {
  return new Program({ ...VotingdapppIDL, address: address ? address.toBase58() : VotingdapppIDL.address } as Votingdappp, provider)
}

// This is a helper function to get the program ID for the Votingdappp program depending on the cluster.
export function getVotingdapppProgramId(cluster: Cluster) {
  switch (cluster) {
    case 'devnet':
    case 'testnet':
      // This is the program ID for the Votingdappp program on devnet and testnet.
      return new PublicKey('coUnmi3oBUtwtd9fjeAvSsJssXh5A5xyPbhpewyzRVF')
    case 'mainnet-beta':
    default:
      return VOTINGDAPPP_PROGRAM_ID
  }
}
