async function main() {
  const [alice, bob] = await getSigners();
  // const contract = await getContractFromAddress(
  //   "cudos1wug8sewp6cedgkmrmvhl3lf3tulagm9hlyfr8q",
  //   bob
  // );
  const CREATE_GAME = {
    create_game: {
      bet: {
        amount: "7500000000",
        denom: "acudos",
      },
      zero: alice.address,
    },
  };

  await alice.execute(
    alice.address,
    "cudos1wug8sewp6cedgkmrmvhl3lf3tulagm9hlyfr8q",
    CREATE_GAME,


  )

  // const CREATE_GAME = {
  //   create_game: {
  //     bet: {
  //       amount: "7500000000",
  //       denom: "acudos",
  //     },
  //     zero: alice.address,
  //   },
  // };
  const result = await contract.execute(CREATE_GAME, bob, "auto", "testing");
  console.log(result);

  // count = await contract.query(QUERY_GET_COUNT, alice);
  // console.log("Count after increment: " + count.count);
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(`${error}`);
    process.exit(1);
  });
