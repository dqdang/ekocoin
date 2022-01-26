// scripts/upgrade_ekotoken.js
import { ethers, upgrades } from 'hardhat';

async function main () {
  const ekotokenV2 = await ethers.getContractFactory('EkoTokenV2');
  console.log('Upgrading ekotoken...');
  await upgrades.upgradeProxy('0xb53127684a568b3173ae13b9f8a6016e243e63b6e8ee1178d6a717850b5d6103', ekotokenV2);
  console.log('ekotokenV2 upgraded');
}

main();

// $ npx hardhat run --network localhost scripts/upgrade_box.js
// Compiling 1 file with 0.8.4
// Compilation finished successfully
// Upgrading ekotoken...
// ekotokenV2 upgraded

// $ npx hardhat console --network localhost
// Welcome to Node.js v12.22.1.
// Type ".help" for more information.
// > const ekotokenV2 = await ethers.getContractFactory('EkoTokenV2');
// undefined
// > const eko = await EkoTokenV2.attach('0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0');
// undefined
// > await eko.transfer(0x0, 1000);
// 0xb53127684a568b3173ae13b9f8a6016e243e63b6e8ee1178d6a717850b5d6107
// ...
// > (await eko.retrieve()).toString();
// 'true'
