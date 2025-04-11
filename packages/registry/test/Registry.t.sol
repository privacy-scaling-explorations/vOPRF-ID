// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "../src/Registry.sol";

contract RegistryTest is Test {
    Registry public registry;
    bytes32[2] public key1;
    bytes32[2] public key2;
    bytes32[2] public key3;
    bytes32[2] public key4;

    function setUp() public {
        registry = new Registry();

        // Create test keys
        key1 = [bytes32(uint256(1)), bytes32(uint256(2))];
        key2 = [bytes32(uint256(3)), bytes32(uint256(4))];
        key3 = [bytes32(uint256(5)), bytes32(uint256(6))];
        key4 = [bytes32(uint256(7)), bytes32(uint256(8))];
    }

    function testBasicRegistration() public {
        uint256 nodeId = registry.register(key1);

        assertTrue(registry.isRegistered(key1));
        assertEq(registry.numRegisteredNodes(), 1);
        assertEq(nodeId, 0);

        bytes32[2] memory storedKey = registry.getNodePublicKey(nodeId);
        assertEq(storedKey[0], key1[0]);
        assertEq(storedKey[1], key1[1]);
    }

    function testCannotRegisterDuplicateKey() public {
        registry.register(key1);

        vm.expectRevert(Registry.DuplicatePublicKey.selector);
        registry.register(key1);
    }

    function testMaxNodesLimit() public {
        // Register three nodes
        uint256 nodeId1 = registry.register(key1);
        uint256 nodeId2 = registry.register(key2);
        uint256 nodeId3 = registry.register(key3);

        // Verify all three are registered
        assertTrue(registry.isRegistered(key1));
        assertTrue(registry.isRegistered(key2));
        assertTrue(registry.isRegistered(key3));
        assertEq(registry.numRegisteredNodes(), 3);
        assertEq(nodeId1, 0);
        assertEq(nodeId2, 1);
        assertEq(nodeId3, 2);

        // Try to register a fourth node
        vm.expectRevert(Registry.MaxNodesReached.selector);
        registry.register(key4);
    }

    function testInvalidNodeId() public {
        vm.expectRevert(Registry.InvalidNodeId.selector);
        registry.getNodePublicKey(0);
    }

    function testEmittedEvents() public {
        vm.expectEmit(true, true, true, true);
        emit Registry.NodeRegistered(0, key1);

        registry.register(key1);
    }
}
