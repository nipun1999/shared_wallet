const anchor = require('@project-serum/anchor');
const { BN } = require('bn.js');
const { SystemProgram } = anchor.web3;

describe('shared_wallet', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const sharedWallet= anchor.web3.Keypair.generate();
  const treasuryWallet = anchor.web3.Keypair.generate();
  const program = anchor.workspace.SharedWallet;
  const alice = anchor.web3.Keypair.generate();
  const bob = anchor.web3.Keypair.generate();
  const recipient = anchor.web3.Keypair.generate();

  it('Creates shared wallet', async () => {
    const signature_alice = await program.provider.connection.requestAirdrop(alice.publicKey, 2000000000);
    await program.provider.connection.confirmTransaction(signature_alice);

    const signature_bob = await program.provider.connection.requestAirdrop(bob.publicKey, 2000000000);
    await program.provider.connection.confirmTransaction(signature_bob);

    const signature_treasury = await program.provider.connection.requestAirdrop(treasuryWallet.publicKey, 1000000000);
    await program.provider.connection.confirmTransaction(signature_treasury);

    let user_1_contribution = new BN(1000000)
    let user_2_contribution = new BN(2000000)

    await program.rpc.createSharedWallet(user_1_contribution,user_2_contribution,{
      accounts: {
        sharedWallet: sharedWallet.publicKey,
        owner: treasuryWallet.publicKey,
        user1: alice.publicKey,
        user2: bob.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [alice,bob,treasuryWallet,sharedWallet]
    });

    const account = await program.account.sharedWallet.fetch(sharedWallet.publicKey);
    console.log(account)
    
  });

  it('excecutes transaction', async () => {

    const signature = await program.provider.connection.requestAirdrop(recipient.publicKey, 2000000000);
    await program.provider.connection.confirmTransaction(signature);

    let user_1_contribution = new BN(900000)
    let user_2_contribution = new BN(1900000)
    let transaction_amount = new BN(200000)

    await program.rpc.executeTransaction(user_1_contribution,user_2_contribution,transaction_amount,{
      accounts: {
        sharedWallet: sharedWallet.publicKey,
        owner: treasuryWallet.publicKey,
        user1: alice.publicKey,
        user2: bob.publicKey,
        recipient: recipient.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [alice,bob,treasuryWallet]
    });
    
  });

  it('excecutes fails transaction', async () => {
    try{
      const signature = await program.provider.connection.requestAirdrop(recipient.publicKey, 2000000000);
      await program.provider.connection.confirmTransaction(signature);
  
      let user_1_contribution = new BN(900000000)
      let user_2_contribution = new BN(1900000000)
      let transaction_amount = new BN(200000)
  
      await program.rpc.executeTransaction(user_1_contribution,user_2_contribution,transaction_amount,{
        accounts: {
          sharedWallet: sharedWallet.publicKey,
          owner: treasuryWallet.publicKey,
          user1: alice.publicKey,
          user2: bob.publicKey,
          recipient: recipient.publicKey,
          systemProgram: SystemProgram.programId,
        },
        signers: [alice,bob,treasuryWallet]
      });
    }catch(err){
      console.log(err)
    }
  });

  it('excecutes fails transaction', async () => {
    try{
      const signature = await program.provider.connection.requestAirdrop(recipient.publicKey, 2000000000);
      await program.provider.connection.confirmTransaction(signature);
  
      let user_1_contribution = new BN(900000000)
      let user_2_contribution = new BN(1900000000)
      let transaction_amount = new BN(200000)
  
      await program.rpc.executeTransaction(user_1_contribution,user_2_contribution,transaction_amount,{
        accounts: {
          sharedWallet: sharedWallet.publicKey,
          owner: treasuryWallet.publicKey,
          user1: alice.publicKey,
          user2: bob.publicKey,
          recipient: recipient.publicKey,
          systemProgram: SystemProgram.programId,
        },
        signers: [alice,bob,treasuryWallet]
      });
    }catch(err){
      console.log(err)
    }
  });

  it('withdraws wallet money', async () => {
    await program.rpc.withdrawsBalance({
      accounts: {
        sharedWallet: sharedWallet.publicKey,
        owner: treasuryWallet.publicKey,
        signer: alice.publicKey,
        user1: alice.publicKey,
        user2: bob.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [alice,treasuryWallet]
    });
    const sharedWalletObj = await program.account.sharedWallet.fetch(sharedWallet.publicKey);
    console.log(sharedWalletObj)
  });

});
