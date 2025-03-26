// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console2} from "forge-std/Script.sol";
import {Registry} from "../src/Registry.sol";

contract RegistryScript is Script {
    function setUp() public {}

    function run() public {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        vm.startBroadcast(deployerPrivateKey);

        Registry registry = new Registry();
        console2.log("Registry deployed to:", address(registry));

        vm.stopBroadcast();
    }
}
