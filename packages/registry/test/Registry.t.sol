// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "../src/Registry.sol";

contract RegistryTest is Test {
    Registry public registry;
    address public node1;
    address public node2;
    address public node3;
    address public node4;
    bytes32[2] public key1;
    bytes32[2] public key2;
    bytes32[2] public key3;
    bytes32[2] public key4;

    function setUp() public {
        registry = new Registry();

        // Create test accounts
        node1 = makeAddr("node1");
        node2 = makeAddr("node2");
        node3 = makeAddr("node3");
        node4 = makeAddr("node4");

        // Create test keys
        key1 = [bytes32(uint256(1)), bytes32(uint256(2))];
        key2 = [bytes32(uint256(3)), bytes32(uint256(4))];
        key3 = [bytes32(uint256(5)), bytes32(uint256(6))];
        key4 = [bytes32(uint256(7)), bytes32(uint256(8))];
    }

    function testBasicRegistration() public {
        vm.prank(node1);
        registry.register(key1);

        assertTrue(registry.isRegistered(node1));
        assertEq(registry.numRegisteredNodes(), 1);

        bytes32[2] memory storedKey = registry.getNodePublicKey(node1);
        assertEq(storedKey[0], key1[0]);
        assertEq(storedKey[1], key1[1]);
    }

    function testCannotRegisterTwice() public {
        vm.prank(node1);
        registry.register(key1);

        vm.prank(node1);
        vm.expectRevert(Registry.AlreadyRegistered.selector);
        registry.register(key1);
    }

    function testMaxNodesLimit() public {
        // Register three nodes
        vm.prank(node1);
        registry.register(key1);

        vm.prank(node2);
        registry.register(key2);

        vm.prank(node3);
        registry.register(key3);

        // Verify all three are registered
        assertTrue(registry.isRegistered(node1));
        assertTrue(registry.isRegistered(node2));
        assertTrue(registry.isRegistered(node3));
        assertEq(registry.numRegisteredNodes(), 3);

        // Try to register a fourth node
        vm.prank(node4);
        vm.expectRevert(Registry.MaxNodesReached.selector);
        registry.register(key4);
    }

    function testEmittedEvents() public {
        vm.expectEmit(true, true, true, true);
        emit Registry.NodeRegistered(node1, key1);

        vm.prank(node1);
        registry.register(key1);
    }
}
