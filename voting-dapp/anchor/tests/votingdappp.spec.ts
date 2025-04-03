import * as anchor from '@coral-xyz/anchor'
import { Program } from '@coral-xyz/anchor'
import { Keypair } from '@solana/web3.js'
import { Votingdappp } from '../target/types/votingdappp'

describe('votingdappp', () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)
  const payer = provider.wallet as anchor.Wallet

  const program = anchor.workspace.Votingdappp as Program<Votingdappp>

  const votingdapppKeypair = Keypair.generate()

  it('Initialize Votingdappp', async () => {
    await program.methods
      .initialize()
      .accounts({
        votingdappp: votingdapppKeypair.publicKey,
        payer: payer.publicKey,
      })
      .signers([votingdapppKeypair])
      .rpc()

    const currentCount = await program.account.votingdappp.fetch(votingdapppKeypair.publicKey)

    expect(currentCount.count).toEqual(0)
  })

  it('Increment Votingdappp', async () => {
    await program.methods.increment().accounts({ votingdappp: votingdapppKeypair.publicKey }).rpc()

    const currentCount = await program.account.votingdappp.fetch(votingdapppKeypair.publicKey)

    expect(currentCount.count).toEqual(1)
  })

  it('Increment Votingdappp Again', async () => {
    await program.methods.increment().accounts({ votingdappp: votingdapppKeypair.publicKey }).rpc()

    const currentCount = await program.account.votingdappp.fetch(votingdapppKeypair.publicKey)

    expect(currentCount.count).toEqual(2)
  })

  it('Decrement Votingdappp', async () => {
    await program.methods.decrement().accounts({ votingdappp: votingdapppKeypair.publicKey }).rpc()

    const currentCount = await program.account.votingdappp.fetch(votingdapppKeypair.publicKey)

    expect(currentCount.count).toEqual(1)
  })

  it('Set votingdappp value', async () => {
    await program.methods.set(42).accounts({ votingdappp: votingdapppKeypair.publicKey }).rpc()

    const currentCount = await program.account.votingdappp.fetch(votingdapppKeypair.publicKey)

    expect(currentCount.count).toEqual(42)
  })

  it('Set close the votingdappp account', async () => {
    await program.methods
      .close()
      .accounts({
        payer: payer.publicKey,
        votingdappp: votingdapppKeypair.publicKey,
      })
      .rpc()

    // The account should no longer exist, returning null.
    const userAccount = await program.account.votingdappp.fetchNullable(votingdapppKeypair.publicKey)
    expect(userAccount).toBeNull()
  })
})
