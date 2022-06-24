const anchor = require('@project-serum/anchor');
const { PublicKey } = require('@solana/web3.js');
const { assert } = require('chai');
const { SystemProgram } = anchor.web3;
const idl = require('./vault.json');
(async () => {
  const provider = anchor.Provider.env(); 
  const program = new anchor.Program(idl, 'Da37QJFR26HJgi6Fr5oWGA16taypVFxhvqXaQbcsz1Qp', provider);
  const systemProgram = SystemProgram.programId;
  // Configure the client to use the local cluster.
  anchor.setProvider(provider);
  let [vaultPDA, _nonce] = await anchor.web3.PublicKey.findProgramAddress(
    [Buffer.from('vault')],
    program.programId
  );
  console.log('vault2', vaultPDA.toString());
  const result = await program.rpc.withdraw(
    _nonce, {
    accounts: {
      vault: vaultPDA,
      admin: provider.wallet.publicKey, // Admin wallet
      systemProgram: systemProgram
    }
  })
  console.log('result', result);
})()