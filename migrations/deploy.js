// Migrations are an early feature. Currently, they're nothing more than this
// single deploy script that's invoked from the CLI, injecting a provider
// configured from the workspace's Anchor.toml.

const anchor = require("@project-serum/anchor");;
const { SystemProgram, PublicKey } = anchor.web3;
const IDL = require('../target/idl/vault.json');
const PROGRAM_ID = 'APudR2nSdiDfBkfmYv1kdVryRGgy4JNdepMhM5PFNESz'
const dotenv = require('dotenv');
dotenv.config();
const init = async function (provider) {
  // Configure client to use the provider.
  anchor.setProvider(provider);
  // Add your deploy script here.
  program = new anchor.Program(IDL, new PublicKey(PROGRAM_ID), provider);
  const systemProgram = SystemProgram.programId;

  let [vaultPDA, _nonce] = await anchor.web3.PublicKey.findProgramAddress(
    [Buffer.from('vault')],
    program.programId
  );

  console.log('vault', vaultPDA.toString());
  console.log('user', provider.wallet.publicKey.toString());
  const result = await program.rpc.initVault(
    _nonce, {
    accounts: {
      vault: vaultPDA,
      user: provider.wallet.publicKey, //Admin wallet
      systemProgram: systemProgram
    }
  });
  console.log('vault result', result);
}

init()