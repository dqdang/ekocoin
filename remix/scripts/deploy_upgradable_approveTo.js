// scripts/deploy_upgradeable_approveTo.js
import { ethers, upgrades } from 'hardhat';

async function main () {
  const approveTo = await ethers.getContractFactory('_approveTo');
  console.log('Deploying approveTo...');
  const eko = await upgrades.deployProxy(approveTo, ['', 0xb53127684a568b3173ae13b9f8a6016e243e63b6e8ee1178d6a717850b5d6103], { initializer: 'store' });
  await eko.deployed();
  console.log('approveTo deployed to:', eko.address);
}

main();

// npx hardhat run --network localhost scripts/deploy_upgradeable_approveTo.js
// Deploying approveTo...
// approveTo deployed to: 0xb2603fc47331e3500eaf053bd7a971b57e613d36

// $ npx hardhat console --network localhost
// Welcome to Node.js v12.22.1.
// Type ".help" for more information.
// > const approveTo = await ethers.getContractFactory('_approveTo');
// undefined
// > const approveTo = await approveTo.attach('0xb2603fc47331e3500eaf053bd7a971b57e613d36');
// undefined
// > (await approveTo.retrieve()).toString();
// '0xb53127684a568b3173ae13b9f8a6016e243e63b6e8ee1178d6a717850b5d6103'
