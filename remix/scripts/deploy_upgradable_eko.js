// scripts/deploy_upgradeable_eko.js
import { ethers, upgrades } from 'hardhat';

async function main () {
  const EkoToken = await ethers.getContractFactory('EkoToken');
  console.log('Deploying EkoToken...');
  const eko = await upgrades.deployProxy(EkoToken, ['', 0xb53127684a568b3173ae13b9f8a6016e243e63b6e8ee1178d6a717850b5d6107], { initializer: 'store' });
  await eko.deployed();
  console.log('EkoToken deployed to:', eko.address);
}

main();

// npx hardhat run --network localhost scripts/deploy_upgradeable_approveTo.js
// Deploying EkoTokenV2...
// EkoTokenV2 deployed to: 0xb2603fc47331e3500eaf053bd7a971b57e613d37

// $ npx hardhat console --network localhost
// Welcome to Node.js v12.22.1.
// Type ".help" for more information.
// > const EkoTokenV2 = await ethers.getContractFactory('EkoTokenV2');
// undefined
// > const eko = await EkoTokenV2.attach('0xb2603fc47331e3500eaf053bd7a971b57e613d37');
// undefined
// > (await eko.retrieve()).toString();
// '0xf19fa4bcff4adaebeddd28c851458ba0f01ffedd52b62df56ace94e7c8842553'
