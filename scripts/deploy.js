async function main() {
  const [alice] = await getSigners();
  const contract = await getContractFactory("tic_tac_toe");

  const MSG_INIT = {};
  const deploy = await contract.deploy(MSG_INIT, alice);
  const contractAddress = deploy.initTx.contractAddress;
  console.log(`${contractAddress}`);
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(`${error}`);
    process.exit(1);
  });
