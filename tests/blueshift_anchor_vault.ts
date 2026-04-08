import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BlueshiftAnchorVault } from "../target/types/blueshift_anchor_vault";

describe("blueshift_anchor_vault", () => {
  // Configure the client to use the local cluster.
 
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider)

  const program = anchor.workspace.blueshiftAnchorVault as Program<BlueshiftAnchorVault>;

  //user wallet
  const signer = provider.wallet;
  console.log("provider wallet", signer);

  //derive pda
  const [vaultpda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("vault"), signer.publicKey.toBuffer()],
    program.programId
  );


  it("Deposit SOL into vault", async () => {
    // Add your test here.
    const amount = new anchor.BN(1* anchor.web3.LAMPORTS_PER_SOL);
    await program.methods.deposit(amount).accounts(
      {
        signer:signer.publicKey,
        vault:vaultpda,
        systemProgram:anchor.web3.SystemProgram.programId,
      }
    ).rpc();

    const balance = await provider.connection.getBalance(vaultpda)
    console.log("Balance after deposit", balance);
  });

  it("Withdraw full amount", async () =>{
    const before_bal = await provider.connection.getBalance(
      signer.publicKey);
    console.log("wallet before deposit:", before_bal);
    const before_vaultBalance = await provider.connection.getBalance(vaultpda);
    console.log("vault before withdraw:", before_vaultBalance);

    
    await program.methods.withdraw().accounts({
      signer:signer.publicKey,
      vault:vaultpda,
      systemProgram:anchor.web3.SystemProgram.programId,

  }).rpc();
  const vaultBalance = await provider.connection.getBalance(vaultpda);
  const after_Balance = await provider.connection.getBalance(signer.publicKey);

  console.log("Vault after withdraw:", vaultBalance);
  console.log("wallet after deposit:", after_Balance);

  }
  )
});
